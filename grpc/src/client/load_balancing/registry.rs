/*
 *
 * Copyright 2025 gRPC authors.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to
 * deal in the Software without restriction, including without limitation the
 * rights to use, copy, modify, merge, publish, distribute, sublicense, and/or
 * sell copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
 * FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS
 * IN THE SOFTWARE.
 *
 */

use std::any::type_name;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::LazyLock;
use std::sync::Mutex;

use crate::client::load_balancing::ChannelController;
use crate::client::load_balancing::DynLbConfig;
use crate::client::load_balancing::DynLbPolicy;
use crate::client::load_balancing::DynLbPolicyBuilder;
use crate::client::load_balancing::LbConfigJson;
use crate::client::load_balancing::LbPolicy;
use crate::client::load_balancing::LbPolicyBuilder;
use crate::client::load_balancing::LbPolicyOptions;
use crate::client::load_balancing::ParsedLbConfig;
use crate::client::load_balancing::WorkData;
use crate::client::load_balancing::subchannel::Subchannel;
use crate::client::load_balancing::subchannel::SubchannelState;
use crate::client::name_resolution::ResolverUpdate;

/// A registry to store and retrieve LB policies.  LB policies are indexed by
/// their names.
pub struct LbPolicyRegistry {
    m: Arc<Mutex<HashMap<String, Arc<DynLbPolicyBuilder>>>>,
}

impl LbPolicyRegistry {
    /// Constructs an empty LB policy registry.
    pub fn new() -> Self {
        Self { m: Arc::default() }
    }

    /// Adds a LB policy into the registry.
    pub fn add_builder<B: LbPolicyBuilder>(&self, builder: B) {
        self.m
            .lock()
            .unwrap()
            .insert(builder.name().to_string(), DynAdapter::new_arc(builder));
    }

    /// Adds a dynamic LB policy into the registry.
    pub fn add_dyn_builder(&self, builder: Arc<DynLbPolicyBuilder>) {
        self.m
            .lock()
            .unwrap()
            .insert(builder.name().to_string(), builder);
    }

    /// Retrieves a LB policy from the registry, or None if not found.
    pub fn get_policy(&self, name: &str) -> Option<Arc<DynLbPolicyBuilder>> {
        self.m.lock().unwrap().get(name).cloned()
    }

    /// Evaluates an ordered child policy list against the registry.
    ///
    /// Halts child policy evaluation immediately on the first supported policy
    /// if configuration parsing fails (gRFC A24).
    ///
    /// # Return Value
    ///
    /// - `Ok(Some(ParsedLbConfig))` when a registered child policy passing the optional
    ///   filter is selected and its configuration successfully parses.
    /// - `Ok(None)` when the child policy list is empty (`[]`), `"null"`, or contains only
    ///   whitespace. Top-level channel service configuration uses this to fall back
    ///   to the legacy `loadBalancingPolicy` or default policy.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The child policy list is malformed JSON or contains invalid entries (e.g. violating
    ///   the gRFC A24 single-property `oneOf` constraint).
    /// - None of the load balancers are registered or permitted by the filter.
    /// - The first supported, permitted policy fails to parse its configuration. In
    ///   accordance with gRFC A24, evaluation halts immediately and subsequent
    ///   child policies are not evaluated.
    ///
    /// # Example
    ///
    /// ```ignore (cannot be compiled as doctest because `load_balancing` is pub(crate))
    /// use grpc::client::load_balancing::GLOBAL_LB_REGISTRY;
    ///
    /// let children_json = r#"[
    ///     {"xds": {}},
    ///     {"round_robin": {}}
    /// ]"#;
    ///
    /// // A parent policy (e.g. a composite LB policy) can filter child policies to only allow specific children.
    /// let selected = GLOBAL_LB_REGISTRY
    ///     .select_child_with_filter(
    ///         children_json,
    ///         Some(|name: &str| name == "round_robin" || name == "pick_first"),
    ///     )
    ///     .unwrap();
    ///
    /// assert!(selected.is_some());
    /// assert_eq!(selected.unwrap().builder.name(), "round_robin");
    /// ```
    pub fn select_child_with_filter<F>(
        &self,
        child_list_json: &str,
        filter: Option<F>,
    ) -> Result<Option<ParsedLbConfig>, String>
    where
        F: Fn(&str) -> bool,
    {
        let trimmed = child_list_json.trim();
        if trimmed.is_empty() || trimmed == "null" {
            return Ok(None);
        }

        let raw_children: Vec<&serde_json::value::RawValue> = serde_json::from_str(trimmed)
            .map_err(|e| format!("failed to parse load balancer configuration JSON: {e}"))?;

        if raw_children.is_empty() {
            return Ok(None);
        }

        let mut unregistered = Vec::new();
        let mut filtered_out = Vec::new();

        for raw_child in raw_children {
            let child: ChildEntry<'_> = serde_json::from_str(raw_child.get())
                .map_err(|e| format!("failed to parse load balancer entry: {e}"))?;

            if let Some(ref f) = filter
                && !f(child.policy_name)
            {
                filtered_out.push(child.policy_name);
                continue;
            }

            if let Some(builder) = self.get_policy(child.policy_name) {
                let payload = LbConfigJson::new(child.raw_config);
                let parsed_config = builder.parse_config(&payload).map_err(|e| {
                    format!(
                        "failed to parse config for policy '{}': {e}",
                        child.policy_name
                    )
                })?;
                return Ok(Some(ParsedLbConfig {
                    builder,
                    config: parsed_config,
                }));
            }

            unregistered.push(child.policy_name);
        }

        let err_msg = match (filtered_out.is_empty(), unregistered.is_empty()) {
            (false, true) => format!(
                "None of the load balancers were permitted by filter: [{}].",
                filtered_out.join(", ")
            ),
            (true, false) => format!(
                "None of the load balancers are registered: [{}].",
                unregistered.join(", ")
            ),
            (false, false) => format!(
                "No supported load balancer selected (unregistered: [{}], filtered out: [{}]).",
                unregistered.join(", "),
                filtered_out.join(", ")
            ),
            (true, true) => "No supported load balancer found in config.".to_string(),
        };

        Err(err_msg)
    }

    /// Evaluates an ordered child policy list against the registry without a filter.
    ///
    /// Returns `Ok(Some(ParsedLbConfig))` on successful selection, `Ok(None)` if the list
    /// is empty or null, or `Err(String)` on syntax errors, unregistered policies, or
    /// configuration parsing failures.
    pub fn select_child(&self, child_list_json: &str) -> Result<Option<ParsedLbConfig>, String> {
        self.select_child_with_filter::<fn(&str) -> bool>(child_list_json, None)
    }

    /// Evaluates an ordered child policy list against the registry, requiring a supported policy.
    ///
    /// Unlike [`select_child`](Self::select_child), an empty, `"null"`, or whitespace-only list
    /// is treated as an error rather than returning `Ok(None)`. This helper is intended for
    /// composite load balancing policies where configuring child policies is mandatory.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The child policy list is empty, `"null"`, or contains only whitespace.
    /// - The child policy list is malformed JSON or contains invalid entries.
    /// - None of the load balancers are registered.
    /// - The first supported policy fails to parse its configuration.
    pub fn select_required_child(&self, child_list_json: &str) -> Result<ParsedLbConfig, String> {
        self.select_child(child_list_json)?
            .ok_or_else(|| "Load balancer configuration is required.".to_string())
    }
}

/// Internal visitor to deserialize a child policy object and strictly enforce the
/// gRFC A24 oneOf rule with zero AST allocation.
#[derive(Debug)]
struct ChildEntry<'de> {
    policy_name: &'de str,
    raw_config: &'de str,
}

impl<'de> serde::Deserialize<'de> for ChildEntry<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ChildEntry<'de>;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a single-property object representing a load balancer")
            }

            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: serde::de::MapAccess<'de>,
            {
                let policy_name: &'de str = access.next_key()?.ok_or_else(|| {
                    serde::de::Error::custom(
                        "Each load balancing config entry must contain exactly one policy name.",
                    )
                })?;
                let raw_val: &'de serde_json::value::RawValue = access.next_value()?;
                if access.next_key::<serde::de::IgnoredAny>()?.is_some() {
                    return Err(serde::de::Error::custom(
                        "Each load balancing config entry must contain exactly one policy name.",
                    ));
                }
                Ok(ChildEntry {
                    policy_name,
                    raw_config: raw_val.get(),
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}

impl Default for LbPolicyRegistry {
    fn default() -> Self {
        Self::new()
    }
}

pub static GLOBAL_LB_REGISTRY: LazyLock<LbPolicyRegistry> = LazyLock::new(|| {
    let registry = LbPolicyRegistry::new();
    registry.add_builder(super::pick_first::PickFirstBuilder {});
    registry.add_builder(super::round_robin::RoundRobinBuilder {});
    registry
});

/// Implements `DynLbPolicy` and `DynLbPolicyBuilder` around the enclosed
/// `LbPolicy` or `LbPolicyBuilder`, respectively.
#[derive(Debug)]
struct DynAdapter<T>(T);

impl<T: LbPolicyBuilder> LbPolicyBuilder for DynAdapter<T> {
    type LbPolicy = Box<DynLbPolicy>;

    fn build(&self, options: LbPolicyOptions) -> Self::LbPolicy {
        Box::new(DynAdapter(self.0.build(options)))
    }

    fn name(&self) -> &'static str {
        self.0.name()
    }

    fn parse_config(&self, config: &LbConfigJson<'_>) -> Result<Option<DynLbConfig>, String> {
        // Call the real parse config and then wrap its result in a DynLbConfig if it is Ok(Some).
        let cfg = self.0.parse_config(config)?;
        Ok(cfg.map(|c| Arc::new(c) as DynLbConfig))
    }
}

impl<T: LbPolicy> LbPolicy for DynAdapter<T> {
    type LbConfig = DynLbConfig;

    fn resolver_update(
        &mut self,
        update: ResolverUpdate,
        config: Option<&DynLbConfig>,
        channel_controller: &mut dyn ChannelController,
    ) -> Result<(), String> {
        let config = config.map(|c| {
            c.downcast_ref::<T::LbConfig>().unwrap_or_else(|| {
                panic!("LB config type should be {}", type_name::<T::LbConfig>())
            })
        });
        self.0.resolver_update(update, config, channel_controller)
    }

    fn subchannel_update(
        &mut self,
        subchannel: Arc<dyn Subchannel>,
        state: &SubchannelState,
        channel_controller: &mut dyn ChannelController,
    ) {
        self.0
            .subchannel_update(subchannel, state, channel_controller);
    }

    fn work(&mut self, data: Option<WorkData>, channel_controller: &mut dyn ChannelController) {
        self.0.work(data, channel_controller);
    }

    fn exit_idle(&mut self, channel_controller: &mut dyn ChannelController) {
        self.0.exit_idle(channel_controller);
    }
}

impl<T> DynAdapter<T> {
    fn new_arc(policy: T) -> Arc<Self> {
        Arc::new(DynAdapter(policy))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filter_skips_registered_policy_and_ignores_trailing_malformed_entries() {
        let json = r#"[
            { "pick_first": { "shuffleAddressList": true } },
            { "round_robin": {} },
            { "invalid": 123, "second_key": true }
        ]"#;

        // Filter rejects pick_first despite it being registered in GLOBAL_LB_REGISTRY.
        let selected = GLOBAL_LB_REGISTRY
            .select_child_with_filter(json, Some(|name: &str| name == "round_robin"))
            .expect("selection should succeed")
            .expect("should find a supported child policy");

        assert_eq!(
            selected.builder.name(),
            "round_robin",
            "selected builder name does not match expected round_robin."
        );
    }

    #[test]
    fn filter_rejecting_all_registered_load_balancers_returns_error() {
        let json = r#"[
            { "pick_first": { "shuffleAddressList": true } },
            { "round_robin": {} }
        ]"#;

        // Filter rejects all policies.
        let result = GLOBAL_LB_REGISTRY.select_child_with_filter(json, Some(|_: &str| false));

        assert!(
            result.is_err(),
            "expected error when all load balancers are rejected by filter."
        );
        let err = result.err().unwrap();
        assert_eq!(
            err, "None of the load balancers were permitted by filter: [pick_first, round_robin].",
            "error message should detail which policies were filtered out."
        );
    }

    #[test]
    fn unregistered_load_balancers_returns_error() {
        let json = r#"[
            { "unregistered_1": {} },
            { "unregistered_2": {} }
        ]"#;

        let result = GLOBAL_LB_REGISTRY.select_child(json);

        assert!(
            result.is_err(),
            "expected error when load balancers are not registered."
        );
        let err = result.err().unwrap();
        assert_eq!(
            err, "None of the load balancers are registered: [unregistered_1, unregistered_2].",
            "error message should detail which policies were unregistered."
        );
    }

    #[test]
    fn mixed_unregistered_and_filtered_load_balancers_returns_error() {
        let json = r#"[
            { "unregistered_1": {} },
            { "round_robin": {} }
        ]"#;

        let result = GLOBAL_LB_REGISTRY
            .select_child_with_filter(json, Some(|name: &str| name == "unregistered_1"));

        assert!(
            result.is_err(),
            "expected error when load balancers are either unregistered or filtered out."
        );
        let err = result.err().unwrap();
        assert_eq!(
            err,
            "No supported load balancer selected (unregistered: [unregistered_1], filtered out: [round_robin]).",
            "error message should distinguish between unregistered and filtered policies."
        );
    }

    #[test]
    fn empty_and_null_child_lists_return_none() {
        let empty_res = GLOBAL_LB_REGISTRY
            .select_child_with_filter::<fn(&str) -> bool>("[]", None)
            .expect("empty array should succeed");
        assert!(empty_res.is_none(), "empty child list should return None.");

        let null_res = GLOBAL_LB_REGISTRY
            .select_child_with_filter::<fn(&str) -> bool>("null", None)
            .expect("null should succeed");
        assert!(null_res.is_none(), "null child list should return None.");

        let whitespace_res = GLOBAL_LB_REGISTRY
            .select_child_with_filter::<fn(&str) -> bool>("   ", None)
            .expect("whitespace should succeed");
        assert!(
            whitespace_res.is_none(),
            "whitespace child list should return None."
        );
    }

    #[test]
    fn select_required_child_succeeds_for_valid_config() {
        let json = r#"[
            { "unsupported_lb": {} },
            { "pick_first": { "shuffleAddressList": true } }
        ]"#;

        let selected = GLOBAL_LB_REGISTRY
            .select_required_child(json)
            .expect("selection should succeed");
        assert_eq!(
            selected.builder.name(),
            "pick_first",
            "selected builder name does not match expected pick_first."
        );
    }

    #[test]
    fn select_required_child_errors_on_empty_and_null_lists() {
        let empty_res = GLOBAL_LB_REGISTRY.select_required_child("[]");
        assert!(
            empty_res.is_err(),
            "empty list should return error for select_required_child."
        );
        assert_eq!(
            empty_res.err().unwrap(),
            "Load balancer configuration is required.",
            "error message should indicate load balancer configuration is required."
        );

        let null_res = GLOBAL_LB_REGISTRY.select_required_child("null");
        assert!(
            null_res.is_err(),
            "null list should return error for select_required_child."
        );

        let whitespace_res = GLOBAL_LB_REGISTRY.select_required_child("   ");
        assert!(
            whitespace_res.is_err(),
            "whitespace list should return error for select_required_child."
        );
    }
}
