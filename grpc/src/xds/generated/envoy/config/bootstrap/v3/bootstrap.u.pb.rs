const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.35.1-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__bootstrap__v3__Bootstrap_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct Bootstrap {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Bootstrap>
}

impl ::protobuf::Message for Bootstrap {
  type MessageView<'msg> = BootstrapView<'msg>;
  type MessageMut<'msg> = BootstrapMut<'msg>;
}

impl ::std::default::Default for Bootstrap {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for Bootstrap {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `Bootstrap` is `Sync` because it does not implement interior mutability.
//    Neither does `BootstrapMut`.
unsafe impl ::std::marker::Sync for Bootstrap {}

// SAFETY:
// - `Bootstrap` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for Bootstrap {}

impl ::protobuf::Proxied for Bootstrap {
  type View<'msg> = BootstrapView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Bootstrap {}

impl ::protobuf::MutProxied for Bootstrap {
  type Mut<'msg> = BootstrapMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct BootstrapView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Bootstrap>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for BootstrapView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for BootstrapView<'msg> {
  type Message = Bootstrap;
}

impl ::std::fmt::Debug for BootstrapView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for BootstrapView<'_> {
  fn default() -> BootstrapView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Bootstrap>> for BootstrapView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Bootstrap>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> BootstrapView<'msg> {

  pub fn to_owned(&self) -> Bootstrap {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // node: optional message envoy.config.core.v3.Node
  pub fn has_node(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn node_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::NodeView<'msg>> {
    self.has_node().then(|| self.node())
  }
  pub fn node(self) -> crate::xds::generated::envoy::config::core::v3::base::NodeView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::NodeView::default())
  }

  // node_context_params: repeated string
  pub fn node_context_params(self) -> ::protobuf::RepeatedView<'msg, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        23
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // static_resources: optional message envoy.config.bootstrap.v3.Bootstrap.StaticResources
  pub fn has_static_resources(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn static_resources_opt(self) -> ::std::option::Option<super::bootstrap::StaticResourcesView<'msg>> {
    self.has_static_resources().then(|| self.static_resources())
  }
  pub fn static_resources(self) -> super::bootstrap::StaticResourcesView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::bootstrap::StaticResourcesView::default())
  }

  // dynamic_resources: optional message envoy.config.bootstrap.v3.Bootstrap.DynamicResources
  pub fn has_dynamic_resources(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn dynamic_resources_opt(self) -> ::std::option::Option<super::bootstrap::DynamicResourcesView<'msg>> {
    self.has_dynamic_resources().then(|| self.dynamic_resources())
  }
  pub fn dynamic_resources(self) -> super::bootstrap::DynamicResourcesView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::bootstrap::DynamicResourcesView::default())
  }

  // cluster_manager: optional message envoy.config.bootstrap.v3.ClusterManager
  pub fn has_cluster_manager(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn cluster_manager_opt(self) -> ::std::option::Option<super::ClusterManagerView<'msg>> {
    self.has_cluster_manager().then(|| self.cluster_manager())
  }
  pub fn cluster_manager(self) -> super::ClusterManagerView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ClusterManagerView::default())
  }

  // hds_config: optional message envoy.config.core.v3.ApiConfigSource
  pub fn has_hds_config(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(11)
    }
  }
  pub fn hds_config_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::config_source::ApiConfigSourceView<'msg>> {
    self.has_hds_config().then(|| self.hds_config())
  }
  pub fn hds_config(self) -> crate::xds::generated::envoy::config::core::v3::config_source::ApiConfigSourceView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(11)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::config_source::ApiConfigSourceView::default())
  }

  // flags_path: optional string
  pub fn flags_path(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        4, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // stats_sinks: repeated message envoy.config.metrics.v3.StatsSink
  pub fn stats_sinks(self) -> ::protobuf::RepeatedView<'msg, crate::xds::generated::envoy::config::metrics::v3::stats::StatsSink> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        5
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::metrics::v3::stats::StatsSink>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // deferred_stat_options: optional message envoy.config.bootstrap.v3.Bootstrap.DeferredStatOptions
  pub fn has_deferred_stat_options(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(36)
    }
  }
  pub fn deferred_stat_options_opt(self) -> ::std::option::Option<super::bootstrap::DeferredStatOptionsView<'msg>> {
    self.has_deferred_stat_options().then(|| self.deferred_stat_options())
  }
  pub fn deferred_stat_options(self) -> super::bootstrap::DeferredStatOptionsView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(36)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::bootstrap::DeferredStatOptionsView::default())
  }

  // stats_config: optional message envoy.config.metrics.v3.StatsConfig
  pub fn has_stats_config(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(10)
    }
  }
  pub fn stats_config_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::metrics::v3::stats::StatsConfigView<'msg>> {
    self.has_stats_config().then(|| self.stats_config())
  }
  pub fn stats_config(self) -> crate::xds::generated::envoy::config::metrics::v3::stats::StatsConfigView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(10)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::metrics::v3::stats::StatsConfigView::default())
  }

  // stats_flush_interval: optional message google.protobuf.Duration
  pub fn has_stats_flush_interval(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn stats_flush_interval_opt(self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'msg>> {
    self.has_stats_flush_interval().then(|| self.stats_flush_interval())
  }
  pub fn stats_flush_interval(self) -> ::protobuf_well_known_types::DurationView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(6)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }

  // stats_flush_on_admin: optional bool
  pub fn has_stats_flush_on_admin(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(26)
    }
  }
  pub fn stats_flush_on_admin_opt(self) -> ::std::option::Option<bool> {
    self.has_stats_flush_on_admin().then(|| self.stats_flush_on_admin())
  }
  pub fn stats_flush_on_admin(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        26, (false).into()
      ).try_into().unwrap()
    }
  }

  // stats_eviction_interval: optional message google.protobuf.Duration
  pub fn has_stats_eviction_interval(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(39)
    }
  }
  pub fn stats_eviction_interval_opt(self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'msg>> {
    self.has_stats_eviction_interval().then(|| self.stats_eviction_interval())
  }
  pub fn stats_eviction_interval(self) -> ::protobuf_well_known_types::DurationView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(39)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }

  // watchdog: optional message envoy.config.bootstrap.v3.Watchdog
  pub fn has_watchdog(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn watchdog_opt(self) -> ::std::option::Option<super::WatchdogView<'msg>> {
    self.has_watchdog().then(|| self.watchdog())
  }
  pub fn watchdog(self) -> super::WatchdogView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(7)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::WatchdogView::default())
  }

  // watchdogs: optional message envoy.config.bootstrap.v3.Watchdogs
  pub fn has_watchdogs(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(24)
    }
  }
  pub fn watchdogs_opt(self) -> ::std::option::Option<super::WatchdogsView<'msg>> {
    self.has_watchdogs().then(|| self.watchdogs())
  }
  pub fn watchdogs(self) -> super::WatchdogsView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(24)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::WatchdogsView::default())
  }

  // tracing: optional message envoy.config.trace.v3.Tracing
  pub fn has_tracing(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(8)
    }
  }
  pub fn tracing_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::trace::v3::http_tracer::TracingView<'msg>> {
    self.has_tracing().then(|| self.tracing())
  }
  pub fn tracing(self) -> crate::xds::generated::envoy::config::trace::v3::http_tracer::TracingView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(8)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::trace::v3::http_tracer::TracingView::default())
  }

  // layered_runtime: optional message envoy.config.bootstrap.v3.LayeredRuntime
  pub fn has_layered_runtime(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(14)
    }
  }
  pub fn layered_runtime_opt(self) -> ::std::option::Option<super::LayeredRuntimeView<'msg>> {
    self.has_layered_runtime().then(|| self.layered_runtime())
  }
  pub fn layered_runtime(self) -> super::LayeredRuntimeView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(14)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::LayeredRuntimeView::default())
  }

  // admin: optional message envoy.config.bootstrap.v3.Admin
  pub fn has_admin(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(9)
    }
  }
  pub fn admin_opt(self) -> ::std::option::Option<super::AdminView<'msg>> {
    self.has_admin().then(|| self.admin())
  }
  pub fn admin(self) -> super::AdminView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(9)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::AdminView::default())
  }

  // overload_manager: optional message envoy.config.overload.v3.OverloadManager
  pub fn has_overload_manager(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(12)
    }
  }
  pub fn overload_manager_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::overload::v3::overload::OverloadManagerView<'msg>> {
    self.has_overload_manager().then(|| self.overload_manager())
  }
  pub fn overload_manager(self) -> crate::xds::generated::envoy::config::overload::v3::overload::OverloadManagerView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(12)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::overload::v3::overload::OverloadManagerView::default())
  }

  // enable_dispatcher_stats: optional bool
  pub fn enable_dispatcher_stats(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        13, (false).into()
      ).try_into().unwrap()
    }
  }

  // header_prefix: optional string
  pub fn header_prefix(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        15, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // stats_server_version_override: optional message google.protobuf.UInt64Value
  pub fn has_stats_server_version_override(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(16)
    }
  }
  pub fn stats_server_version_override_opt(self) -> ::std::option::Option<::protobuf_well_known_types::UInt64ValueView<'msg>> {
    self.has_stats_server_version_override().then(|| self.stats_server_version_override())
  }
  pub fn stats_server_version_override(self) -> ::protobuf_well_known_types::UInt64ValueView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(16)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt64ValueView::default())
  }

  // use_tcp_for_dns_lookups: optional bool
  pub fn use_tcp_for_dns_lookups(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        17, (false).into()
      ).try_into().unwrap()
    }
  }

  // dns_resolution_config: optional message envoy.config.core.v3.DnsResolutionConfig
  pub fn has_dns_resolution_config(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(27)
    }
  }
  pub fn dns_resolution_config_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::resolver::DnsResolutionConfigView<'msg>> {
    self.has_dns_resolution_config().then(|| self.dns_resolution_config())
  }
  pub fn dns_resolution_config(self) -> crate::xds::generated::envoy::config::core::v3::resolver::DnsResolutionConfigView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(27)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::resolver::DnsResolutionConfigView::default())
  }

  // typed_dns_resolver_config: optional message envoy.config.core.v3.TypedExtensionConfig
  pub fn has_typed_dns_resolver_config(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(28)
    }
  }
  pub fn typed_dns_resolver_config_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'msg>> {
    self.has_typed_dns_resolver_config().then(|| self.typed_dns_resolver_config())
  }
  pub fn typed_dns_resolver_config(self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(28)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView::default())
  }

  // bootstrap_extensions: repeated message envoy.config.core.v3.TypedExtensionConfig
  pub fn bootstrap_extensions(self) -> ::protobuf::RepeatedView<'msg, crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        18
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // fatal_actions: repeated message envoy.config.bootstrap.v3.FatalAction
  pub fn fatal_actions(self) -> ::protobuf::RepeatedView<'msg, super::FatalAction> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        25
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::FatalAction>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // config_sources: repeated message envoy.config.core.v3.ConfigSource
  pub fn config_sources(self) -> ::protobuf::RepeatedView<'msg, crate::xds::generated::envoy::config::core::v3::config_source::ConfigSource> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        19
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::core::v3::config_source::ConfigSource>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // default_config_source: optional message envoy.config.core.v3.ConfigSource
  pub fn has_default_config_source(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(20)
    }
  }
  pub fn default_config_source_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::config_source::ConfigSourceView<'msg>> {
    self.has_default_config_source().then(|| self.default_config_source())
  }
  pub fn default_config_source(self) -> crate::xds::generated::envoy::config::core::v3::config_source::ConfigSourceView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(20)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::config_source::ConfigSourceView::default())
  }

  // default_socket_interface: optional string
  pub fn default_socket_interface(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        21, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // certificate_provider_instances: repeated message envoy.config.bootstrap.v3.Bootstrap.CertificateProviderInstancesEntry
  pub fn certificate_provider_instances(self)
    -> ::protobuf::MapView<'msg, ::protobuf::ProtoString, crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig> {
    unsafe {
      <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(&self, ::protobuf::__internal::Private)
        .get_map_at_index(22)
        .map_or_else(
          ::protobuf::__internal::runtime::empty_map::<::protobuf::ProtoString, crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig>,
          |raw| ::protobuf::MapView::from_raw(::protobuf::__internal::Private, raw)
        )
    }
  }

  // inline_headers: repeated message envoy.config.bootstrap.v3.CustomInlineHeader
  pub fn inline_headers(self) -> ::protobuf::RepeatedView<'msg, super::CustomInlineHeader> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        29
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::CustomInlineHeader>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // perf_tracing_file_path: optional string
  pub fn perf_tracing_file_path(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        30, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // default_regex_engine: optional message envoy.config.core.v3.TypedExtensionConfig
  pub fn has_default_regex_engine(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(31)
    }
  }
  pub fn default_regex_engine_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'msg>> {
    self.has_default_regex_engine().then(|| self.default_regex_engine())
  }
  pub fn default_regex_engine(self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(31)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView::default())
  }

  // xds_delegate_extension: optional message envoy.config.core.v3.TypedExtensionConfig
  pub fn has_xds_delegate_extension(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(32)
    }
  }
  pub fn xds_delegate_extension_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'msg>> {
    self.has_xds_delegate_extension().then(|| self.xds_delegate_extension())
  }
  pub fn xds_delegate_extension(self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(32)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView::default())
  }

  // xds_config_tracker_extension: optional message envoy.config.core.v3.TypedExtensionConfig
  pub fn has_xds_config_tracker_extension(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(33)
    }
  }
  pub fn xds_config_tracker_extension_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'msg>> {
    self.has_xds_config_tracker_extension().then(|| self.xds_config_tracker_extension())
  }
  pub fn xds_config_tracker_extension(self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(33)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView::default())
  }

  // listener_manager: optional message envoy.config.core.v3.TypedExtensionConfig
  pub fn has_listener_manager(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(34)
    }
  }
  pub fn listener_manager_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'msg>> {
    self.has_listener_manager().then(|| self.listener_manager())
  }
  pub fn listener_manager(self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(34)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView::default())
  }

  // application_log_config: optional message envoy.config.bootstrap.v3.Bootstrap.ApplicationLogConfig
  pub fn has_application_log_config(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(35)
    }
  }
  pub fn application_log_config_opt(self) -> ::std::option::Option<super::bootstrap::ApplicationLogConfigView<'msg>> {
    self.has_application_log_config().then(|| self.application_log_config())
  }
  pub fn application_log_config(self) -> super::bootstrap::ApplicationLogConfigView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(35)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::bootstrap::ApplicationLogConfigView::default())
  }

  // grpc_async_client_manager_config: optional message envoy.config.bootstrap.v3.Bootstrap.GrpcAsyncClientManagerConfig
  pub fn has_grpc_async_client_manager_config(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(37)
    }
  }
  pub fn grpc_async_client_manager_config_opt(self) -> ::std::option::Option<super::bootstrap::GrpcAsyncClientManagerConfigView<'msg>> {
    self.has_grpc_async_client_manager_config().then(|| self.grpc_async_client_manager_config())
  }
  pub fn grpc_async_client_manager_config(self) -> super::bootstrap::GrpcAsyncClientManagerConfigView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(37)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::bootstrap::GrpcAsyncClientManagerConfigView::default())
  }

  // memory_allocator_manager: optional message envoy.config.bootstrap.v3.MemoryAllocatorManager
  pub fn has_memory_allocator_manager(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(38)
    }
  }
  pub fn memory_allocator_manager_opt(self) -> ::std::option::Option<super::MemoryAllocatorManagerView<'msg>> {
    self.has_memory_allocator_manager().then(|| self.memory_allocator_manager())
  }
  pub fn memory_allocator_manager(self) -> super::MemoryAllocatorManagerView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(38)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::MemoryAllocatorManagerView::default())
  }

  pub fn stats_flush(self) -> super::bootstrap::StatsFlushOneof<'msg> {
    match self.stats_flush_case() {
      super::bootstrap::StatsFlushCase::StatsFlushOnAdmin =>
          super::bootstrap::StatsFlushOneof::StatsFlushOnAdmin(self.stats_flush_on_admin()),
      _ => super::bootstrap::StatsFlushOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn stats_flush_case(self) -> super::bootstrap::StatsFlushCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(26);
      super::bootstrap::StatsFlushCase::try_from(field_num).unwrap_unchecked()
    }
  }
  pub fn stats_eviction(self) -> super::bootstrap::StatsEvictionOneof<'msg> {
    match self.stats_eviction_case() {
      super::bootstrap::StatsEvictionCase::StatsEvictionInterval =>
          super::bootstrap::StatsEvictionOneof::StatsEvictionInterval(self.stats_eviction_interval()),
      _ => super::bootstrap::StatsEvictionOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn stats_eviction_case(self) -> super::bootstrap::StatsEvictionCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(39);
      super::bootstrap::StatsEvictionCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `BootstrapView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for BootstrapView<'_> {}

// SAFETY:
// - `BootstrapView` is `Send` because while its alive a `BootstrapMut` cannot.
// - `BootstrapView` does not use thread-local data.
unsafe impl ::std::marker::Send for BootstrapView<'_> {}

impl<'msg> ::protobuf::AsView for BootstrapView<'msg> {
  type Proxied = Bootstrap;
  fn as_view(&self) -> ::protobuf::View<'msg, Bootstrap> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for BootstrapView<'msg> {
  fn into_view<'shorter>(self) -> BootstrapView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Bootstrap> for BootstrapView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Bootstrap {
    let mut dst = Bootstrap::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Bootstrap> for BootstrapMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Bootstrap {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for Bootstrap {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for BootstrapView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for BootstrapMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct BootstrapMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Bootstrap>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for BootstrapMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for BootstrapMut<'msg> {
  type Message = Bootstrap;
}

impl ::std::fmt::Debug for BootstrapMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Bootstrap>> for BootstrapMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Bootstrap>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> BootstrapMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Bootstrap> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> Bootstrap {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // node: optional message envoy.config.core.v3.Node
  pub fn has_node(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_node(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn node_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::NodeView<'_>> {
    self.has_node().then(|| self.node())
  }
  pub fn node(&self) -> crate::xds::generated::envoy::config::core::v3::base::NodeView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::NodeView::default())
  }
  pub fn node_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::base::NodeMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         0, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_node(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::base::Node>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // node_context_params: repeated string
  pub fn node_context_params(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        23
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn node_context_params_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        23,
        self.inner.arena()
      ).expect("alloc should not fail");
      ::protobuf::RepeatedMut::from_inner(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::InnerRepeatedMut::new(
          raw_array, self.inner.arena(),
        ),
      )
    }
  }
  pub fn set_node_context_params(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        23,
        src);
    }
  }

  // static_resources: optional message envoy.config.bootstrap.v3.Bootstrap.StaticResources
  pub fn has_static_resources(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_static_resources(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn static_resources_opt(&self) -> ::std::option::Option<super::bootstrap::StaticResourcesView<'_>> {
    self.has_static_resources().then(|| self.static_resources())
  }
  pub fn static_resources(&self) -> super::bootstrap::StaticResourcesView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::bootstrap::StaticResourcesView::default())
  }
  pub fn static_resources_mut(&mut self) -> super::bootstrap::StaticResourcesMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         1, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_static_resources(&mut self,
    val: impl ::protobuf::IntoProxied<super::bootstrap::StaticResources>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // dynamic_resources: optional message envoy.config.bootstrap.v3.Bootstrap.DynamicResources
  pub fn has_dynamic_resources(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_dynamic_resources(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn dynamic_resources_opt(&self) -> ::std::option::Option<super::bootstrap::DynamicResourcesView<'_>> {
    self.has_dynamic_resources().then(|| self.dynamic_resources())
  }
  pub fn dynamic_resources(&self) -> super::bootstrap::DynamicResourcesView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::bootstrap::DynamicResourcesView::default())
  }
  pub fn dynamic_resources_mut(&mut self) -> super::bootstrap::DynamicResourcesMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         2, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_dynamic_resources(&mut self,
    val: impl ::protobuf::IntoProxied<super::bootstrap::DynamicResources>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // cluster_manager: optional message envoy.config.bootstrap.v3.ClusterManager
  pub fn has_cluster_manager(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_cluster_manager(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn cluster_manager_opt(&self) -> ::std::option::Option<super::ClusterManagerView<'_>> {
    self.has_cluster_manager().then(|| self.cluster_manager())
  }
  pub fn cluster_manager(&self) -> super::ClusterManagerView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ClusterManagerView::default())
  }
  pub fn cluster_manager_mut(&mut self) -> super::ClusterManagerMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         3, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_cluster_manager(&mut self,
    val: impl ::protobuf::IntoProxied<super::ClusterManager>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // hds_config: optional message envoy.config.core.v3.ApiConfigSource
  pub fn has_hds_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(11)
    }
  }
  pub fn clear_hds_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        11
      );
    }
  }
  pub fn hds_config_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::config_source::ApiConfigSourceView<'_>> {
    self.has_hds_config().then(|| self.hds_config())
  }
  pub fn hds_config(&self) -> crate::xds::generated::envoy::config::core::v3::config_source::ApiConfigSourceView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(11)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::config_source::ApiConfigSourceView::default())
  }
  pub fn hds_config_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::config_source::ApiConfigSourceMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         11, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_hds_config(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::config_source::ApiConfigSource>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        11,
        val
      );
    }
  }

  // flags_path: optional string
  pub fn flags_path(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        4, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_flags_path(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val);
    }
  }

  // stats_sinks: repeated message envoy.config.metrics.v3.StatsSink
  pub fn stats_sinks(&self) -> ::protobuf::RepeatedView<'_, crate::xds::generated::envoy::config::metrics::v3::stats::StatsSink> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        5
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::metrics::v3::stats::StatsSink>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn stats_sinks_mut(&mut self) -> ::protobuf::RepeatedMut<'_, crate::xds::generated::envoy::config::metrics::v3::stats::StatsSink> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        5,
        self.inner.arena()
      ).expect("alloc should not fail");
      ::protobuf::RepeatedMut::from_inner(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::InnerRepeatedMut::new(
          raw_array, self.inner.arena(),
        ),
      )
    }
  }
  pub fn set_stats_sinks(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<crate::xds::generated::envoy::config::metrics::v3::stats::StatsSink>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        src);
    }
  }

  // deferred_stat_options: optional message envoy.config.bootstrap.v3.Bootstrap.DeferredStatOptions
  pub fn has_deferred_stat_options(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(36)
    }
  }
  pub fn clear_deferred_stat_options(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        36
      );
    }
  }
  pub fn deferred_stat_options_opt(&self) -> ::std::option::Option<super::bootstrap::DeferredStatOptionsView<'_>> {
    self.has_deferred_stat_options().then(|| self.deferred_stat_options())
  }
  pub fn deferred_stat_options(&self) -> super::bootstrap::DeferredStatOptionsView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(36)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::bootstrap::DeferredStatOptionsView::default())
  }
  pub fn deferred_stat_options_mut(&mut self) -> super::bootstrap::DeferredStatOptionsMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         36, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_deferred_stat_options(&mut self,
    val: impl ::protobuf::IntoProxied<super::bootstrap::DeferredStatOptions>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        36,
        val
      );
    }
  }

  // stats_config: optional message envoy.config.metrics.v3.StatsConfig
  pub fn has_stats_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(10)
    }
  }
  pub fn clear_stats_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        10
      );
    }
  }
  pub fn stats_config_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::metrics::v3::stats::StatsConfigView<'_>> {
    self.has_stats_config().then(|| self.stats_config())
  }
  pub fn stats_config(&self) -> crate::xds::generated::envoy::config::metrics::v3::stats::StatsConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(10)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::metrics::v3::stats::StatsConfigView::default())
  }
  pub fn stats_config_mut(&mut self) -> crate::xds::generated::envoy::config::metrics::v3::stats::StatsConfigMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         10, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_stats_config(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::metrics::v3::stats::StatsConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        10,
        val
      );
    }
  }

  // stats_flush_interval: optional message google.protobuf.Duration
  pub fn has_stats_flush_interval(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn clear_stats_flush_interval(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        6
      );
    }
  }
  pub fn stats_flush_interval_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_stats_flush_interval().then(|| self.stats_flush_interval())
  }
  pub fn stats_flush_interval(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(6)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn stats_flush_interval_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         6, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_stats_flush_interval(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        val
      );
    }
  }

  // stats_flush_on_admin: optional bool
  pub fn has_stats_flush_on_admin(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(26)
    }
  }
  pub fn clear_stats_flush_on_admin(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        26
      );
    }
  }
  pub fn stats_flush_on_admin_opt(&self) -> ::std::option::Option<bool> {
    self.has_stats_flush_on_admin().then(|| self.stats_flush_on_admin())
  }
  pub fn stats_flush_on_admin(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        26, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_stats_flush_on_admin(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        26, val.into()
      )
    }
  }

  // stats_eviction_interval: optional message google.protobuf.Duration
  pub fn has_stats_eviction_interval(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(39)
    }
  }
  pub fn clear_stats_eviction_interval(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        39
      );
    }
  }
  pub fn stats_eviction_interval_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_stats_eviction_interval().then(|| self.stats_eviction_interval())
  }
  pub fn stats_eviction_interval(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(39)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn stats_eviction_interval_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         39, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_stats_eviction_interval(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        39,
        val
      );
    }
  }

  // watchdog: optional message envoy.config.bootstrap.v3.Watchdog
  pub fn has_watchdog(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn clear_watchdog(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        7
      );
    }
  }
  pub fn watchdog_opt(&self) -> ::std::option::Option<super::WatchdogView<'_>> {
    self.has_watchdog().then(|| self.watchdog())
  }
  pub fn watchdog(&self) -> super::WatchdogView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(7)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::WatchdogView::default())
  }
  pub fn watchdog_mut(&mut self) -> super::WatchdogMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         7, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_watchdog(&mut self,
    val: impl ::protobuf::IntoProxied<super::Watchdog>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        7,
        val
      );
    }
  }

  // watchdogs: optional message envoy.config.bootstrap.v3.Watchdogs
  pub fn has_watchdogs(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(24)
    }
  }
  pub fn clear_watchdogs(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        24
      );
    }
  }
  pub fn watchdogs_opt(&self) -> ::std::option::Option<super::WatchdogsView<'_>> {
    self.has_watchdogs().then(|| self.watchdogs())
  }
  pub fn watchdogs(&self) -> super::WatchdogsView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(24)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::WatchdogsView::default())
  }
  pub fn watchdogs_mut(&mut self) -> super::WatchdogsMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         24, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_watchdogs(&mut self,
    val: impl ::protobuf::IntoProxied<super::Watchdogs>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        24,
        val
      );
    }
  }

  // tracing: optional message envoy.config.trace.v3.Tracing
  pub fn has_tracing(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(8)
    }
  }
  pub fn clear_tracing(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        8
      );
    }
  }
  pub fn tracing_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::trace::v3::http_tracer::TracingView<'_>> {
    self.has_tracing().then(|| self.tracing())
  }
  pub fn tracing(&self) -> crate::xds::generated::envoy::config::trace::v3::http_tracer::TracingView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(8)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::trace::v3::http_tracer::TracingView::default())
  }
  pub fn tracing_mut(&mut self) -> crate::xds::generated::envoy::config::trace::v3::http_tracer::TracingMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         8, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_tracing(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::trace::v3::http_tracer::Tracing>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        8,
        val
      );
    }
  }

  // layered_runtime: optional message envoy.config.bootstrap.v3.LayeredRuntime
  pub fn has_layered_runtime(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(14)
    }
  }
  pub fn clear_layered_runtime(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        14
      );
    }
  }
  pub fn layered_runtime_opt(&self) -> ::std::option::Option<super::LayeredRuntimeView<'_>> {
    self.has_layered_runtime().then(|| self.layered_runtime())
  }
  pub fn layered_runtime(&self) -> super::LayeredRuntimeView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(14)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::LayeredRuntimeView::default())
  }
  pub fn layered_runtime_mut(&mut self) -> super::LayeredRuntimeMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         14, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_layered_runtime(&mut self,
    val: impl ::protobuf::IntoProxied<super::LayeredRuntime>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        14,
        val
      );
    }
  }

  // admin: optional message envoy.config.bootstrap.v3.Admin
  pub fn has_admin(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(9)
    }
  }
  pub fn clear_admin(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        9
      );
    }
  }
  pub fn admin_opt(&self) -> ::std::option::Option<super::AdminView<'_>> {
    self.has_admin().then(|| self.admin())
  }
  pub fn admin(&self) -> super::AdminView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(9)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::AdminView::default())
  }
  pub fn admin_mut(&mut self) -> super::AdminMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         9, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_admin(&mut self,
    val: impl ::protobuf::IntoProxied<super::Admin>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        9,
        val
      );
    }
  }

  // overload_manager: optional message envoy.config.overload.v3.OverloadManager
  pub fn has_overload_manager(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(12)
    }
  }
  pub fn clear_overload_manager(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        12
      );
    }
  }
  pub fn overload_manager_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::overload::v3::overload::OverloadManagerView<'_>> {
    self.has_overload_manager().then(|| self.overload_manager())
  }
  pub fn overload_manager(&self) -> crate::xds::generated::envoy::config::overload::v3::overload::OverloadManagerView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(12)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::overload::v3::overload::OverloadManagerView::default())
  }
  pub fn overload_manager_mut(&mut self) -> crate::xds::generated::envoy::config::overload::v3::overload::OverloadManagerMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         12, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_overload_manager(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::overload::v3::overload::OverloadManager>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        12,
        val
      );
    }
  }

  // enable_dispatcher_stats: optional bool
  pub fn enable_dispatcher_stats(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        13, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_enable_dispatcher_stats(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        13, val.into()
      )
    }
  }

  // header_prefix: optional string
  pub fn header_prefix(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        15, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_header_prefix(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        15,
        val);
    }
  }

  // stats_server_version_override: optional message google.protobuf.UInt64Value
  pub fn has_stats_server_version_override(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(16)
    }
  }
  pub fn clear_stats_server_version_override(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        16
      );
    }
  }
  pub fn stats_server_version_override_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt64ValueView<'_>> {
    self.has_stats_server_version_override().then(|| self.stats_server_version_override())
  }
  pub fn stats_server_version_override(&self) -> ::protobuf_well_known_types::UInt64ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(16)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt64ValueView::default())
  }
  pub fn stats_server_version_override_mut(&mut self) -> ::protobuf_well_known_types::UInt64ValueMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         16, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_stats_server_version_override(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt64Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        16,
        val
      );
    }
  }

  // use_tcp_for_dns_lookups: optional bool
  pub fn use_tcp_for_dns_lookups(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        17, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_use_tcp_for_dns_lookups(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        17, val.into()
      )
    }
  }

  // dns_resolution_config: optional message envoy.config.core.v3.DnsResolutionConfig
  pub fn has_dns_resolution_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(27)
    }
  }
  pub fn clear_dns_resolution_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        27
      );
    }
  }
  pub fn dns_resolution_config_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::resolver::DnsResolutionConfigView<'_>> {
    self.has_dns_resolution_config().then(|| self.dns_resolution_config())
  }
  pub fn dns_resolution_config(&self) -> crate::xds::generated::envoy::config::core::v3::resolver::DnsResolutionConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(27)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::resolver::DnsResolutionConfigView::default())
  }
  pub fn dns_resolution_config_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::resolver::DnsResolutionConfigMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         27, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_dns_resolution_config(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::resolver::DnsResolutionConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        27,
        val
      );
    }
  }

  // typed_dns_resolver_config: optional message envoy.config.core.v3.TypedExtensionConfig
  pub fn has_typed_dns_resolver_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(28)
    }
  }
  pub fn clear_typed_dns_resolver_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        28
      );
    }
  }
  pub fn typed_dns_resolver_config_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'_>> {
    self.has_typed_dns_resolver_config().then(|| self.typed_dns_resolver_config())
  }
  pub fn typed_dns_resolver_config(&self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(28)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView::default())
  }
  pub fn typed_dns_resolver_config_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         28, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_typed_dns_resolver_config(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        28,
        val
      );
    }
  }

  // bootstrap_extensions: repeated message envoy.config.core.v3.TypedExtensionConfig
  pub fn bootstrap_extensions(&self) -> ::protobuf::RepeatedView<'_, crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        18
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn bootstrap_extensions_mut(&mut self) -> ::protobuf::RepeatedMut<'_, crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        18,
        self.inner.arena()
      ).expect("alloc should not fail");
      ::protobuf::RepeatedMut::from_inner(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::InnerRepeatedMut::new(
          raw_array, self.inner.arena(),
        ),
      )
    }
  }
  pub fn set_bootstrap_extensions(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        18,
        src);
    }
  }

  // fatal_actions: repeated message envoy.config.bootstrap.v3.FatalAction
  pub fn fatal_actions(&self) -> ::protobuf::RepeatedView<'_, super::FatalAction> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        25
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::FatalAction>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn fatal_actions_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::FatalAction> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        25,
        self.inner.arena()
      ).expect("alloc should not fail");
      ::protobuf::RepeatedMut::from_inner(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::InnerRepeatedMut::new(
          raw_array, self.inner.arena(),
        ),
      )
    }
  }
  pub fn set_fatal_actions(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::FatalAction>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        25,
        src);
    }
  }

  // config_sources: repeated message envoy.config.core.v3.ConfigSource
  pub fn config_sources(&self) -> ::protobuf::RepeatedView<'_, crate::xds::generated::envoy::config::core::v3::config_source::ConfigSource> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        19
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::core::v3::config_source::ConfigSource>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn config_sources_mut(&mut self) -> ::protobuf::RepeatedMut<'_, crate::xds::generated::envoy::config::core::v3::config_source::ConfigSource> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        19,
        self.inner.arena()
      ).expect("alloc should not fail");
      ::protobuf::RepeatedMut::from_inner(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::InnerRepeatedMut::new(
          raw_array, self.inner.arena(),
        ),
      )
    }
  }
  pub fn set_config_sources(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<crate::xds::generated::envoy::config::core::v3::config_source::ConfigSource>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        19,
        src);
    }
  }

  // default_config_source: optional message envoy.config.core.v3.ConfigSource
  pub fn has_default_config_source(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(20)
    }
  }
  pub fn clear_default_config_source(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        20
      );
    }
  }
  pub fn default_config_source_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::config_source::ConfigSourceView<'_>> {
    self.has_default_config_source().then(|| self.default_config_source())
  }
  pub fn default_config_source(&self) -> crate::xds::generated::envoy::config::core::v3::config_source::ConfigSourceView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(20)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::config_source::ConfigSourceView::default())
  }
  pub fn default_config_source_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::config_source::ConfigSourceMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         20, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_default_config_source(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::config_source::ConfigSource>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        20,
        val
      );
    }
  }

  // default_socket_interface: optional string
  pub fn default_socket_interface(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        21, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_default_socket_interface(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        21,
        val);
    }
  }

  // certificate_provider_instances: repeated message envoy.config.bootstrap.v3.Bootstrap.CertificateProviderInstancesEntry
  pub fn certificate_provider_instances(&self)
    -> ::protobuf::MapView<'_, ::protobuf::ProtoString, crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig> {
    unsafe {
      <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(&self, ::protobuf::__internal::Private)
        .get_map_at_index(22)
        .map_or_else(
          ::protobuf::__internal::runtime::empty_map::<::protobuf::ProtoString, crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig>,
          |raw| ::protobuf::MapView::from_raw(::protobuf::__internal::Private, raw)
        )
    }
  }
  pub fn certificate_provider_instances_mut(&mut self)
    -> ::protobuf::MapMut<'_, ::protobuf::ProtoString, crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig> {
    unsafe {
      let raw_map = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtrMut>::get_ptr_mut(self, ::protobuf::__internal::Private)
        .get_or_create_mutable_map_at_index(
          22, self.inner.arena()).unwrap();
      let inner = ::protobuf::__internal::runtime::InnerMapMut::new(
        raw_map, self.inner.arena());
      ::protobuf::MapMut::from_inner(::protobuf::__internal::Private, inner)
    }
  }
  pub fn set_certificate_provider_instances(
      &mut self,
      src: impl ::protobuf::IntoProxied<::protobuf::Map<::protobuf::ProtoString, crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_map_field(
        ::protobuf::AsMut::as_mut(self).inner,
        22,
        src);
    }
  }

  // inline_headers: repeated message envoy.config.bootstrap.v3.CustomInlineHeader
  pub fn inline_headers(&self) -> ::protobuf::RepeatedView<'_, super::CustomInlineHeader> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        29
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::CustomInlineHeader>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn inline_headers_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::CustomInlineHeader> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        29,
        self.inner.arena()
      ).expect("alloc should not fail");
      ::protobuf::RepeatedMut::from_inner(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::InnerRepeatedMut::new(
          raw_array, self.inner.arena(),
        ),
      )
    }
  }
  pub fn set_inline_headers(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::CustomInlineHeader>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        29,
        src);
    }
  }

  // perf_tracing_file_path: optional string
  pub fn perf_tracing_file_path(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        30, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_perf_tracing_file_path(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        30,
        val);
    }
  }

  // default_regex_engine: optional message envoy.config.core.v3.TypedExtensionConfig
  pub fn has_default_regex_engine(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(31)
    }
  }
  pub fn clear_default_regex_engine(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        31
      );
    }
  }
  pub fn default_regex_engine_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'_>> {
    self.has_default_regex_engine().then(|| self.default_regex_engine())
  }
  pub fn default_regex_engine(&self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(31)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView::default())
  }
  pub fn default_regex_engine_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         31, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_default_regex_engine(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        31,
        val
      );
    }
  }

  // xds_delegate_extension: optional message envoy.config.core.v3.TypedExtensionConfig
  pub fn has_xds_delegate_extension(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(32)
    }
  }
  pub fn clear_xds_delegate_extension(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        32
      );
    }
  }
  pub fn xds_delegate_extension_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'_>> {
    self.has_xds_delegate_extension().then(|| self.xds_delegate_extension())
  }
  pub fn xds_delegate_extension(&self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(32)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView::default())
  }
  pub fn xds_delegate_extension_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         32, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_xds_delegate_extension(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        32,
        val
      );
    }
  }

  // xds_config_tracker_extension: optional message envoy.config.core.v3.TypedExtensionConfig
  pub fn has_xds_config_tracker_extension(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(33)
    }
  }
  pub fn clear_xds_config_tracker_extension(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        33
      );
    }
  }
  pub fn xds_config_tracker_extension_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'_>> {
    self.has_xds_config_tracker_extension().then(|| self.xds_config_tracker_extension())
  }
  pub fn xds_config_tracker_extension(&self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(33)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView::default())
  }
  pub fn xds_config_tracker_extension_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         33, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_xds_config_tracker_extension(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        33,
        val
      );
    }
  }

  // listener_manager: optional message envoy.config.core.v3.TypedExtensionConfig
  pub fn has_listener_manager(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(34)
    }
  }
  pub fn clear_listener_manager(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        34
      );
    }
  }
  pub fn listener_manager_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'_>> {
    self.has_listener_manager().then(|| self.listener_manager())
  }
  pub fn listener_manager(&self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(34)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView::default())
  }
  pub fn listener_manager_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         34, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_listener_manager(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        34,
        val
      );
    }
  }

  // application_log_config: optional message envoy.config.bootstrap.v3.Bootstrap.ApplicationLogConfig
  pub fn has_application_log_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(35)
    }
  }
  pub fn clear_application_log_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        35
      );
    }
  }
  pub fn application_log_config_opt(&self) -> ::std::option::Option<super::bootstrap::ApplicationLogConfigView<'_>> {
    self.has_application_log_config().then(|| self.application_log_config())
  }
  pub fn application_log_config(&self) -> super::bootstrap::ApplicationLogConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(35)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::bootstrap::ApplicationLogConfigView::default())
  }
  pub fn application_log_config_mut(&mut self) -> super::bootstrap::ApplicationLogConfigMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         35, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_application_log_config(&mut self,
    val: impl ::protobuf::IntoProxied<super::bootstrap::ApplicationLogConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        35,
        val
      );
    }
  }

  // grpc_async_client_manager_config: optional message envoy.config.bootstrap.v3.Bootstrap.GrpcAsyncClientManagerConfig
  pub fn has_grpc_async_client_manager_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(37)
    }
  }
  pub fn clear_grpc_async_client_manager_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        37
      );
    }
  }
  pub fn grpc_async_client_manager_config_opt(&self) -> ::std::option::Option<super::bootstrap::GrpcAsyncClientManagerConfigView<'_>> {
    self.has_grpc_async_client_manager_config().then(|| self.grpc_async_client_manager_config())
  }
  pub fn grpc_async_client_manager_config(&self) -> super::bootstrap::GrpcAsyncClientManagerConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(37)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::bootstrap::GrpcAsyncClientManagerConfigView::default())
  }
  pub fn grpc_async_client_manager_config_mut(&mut self) -> super::bootstrap::GrpcAsyncClientManagerConfigMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         37, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_grpc_async_client_manager_config(&mut self,
    val: impl ::protobuf::IntoProxied<super::bootstrap::GrpcAsyncClientManagerConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        37,
        val
      );
    }
  }

  // memory_allocator_manager: optional message envoy.config.bootstrap.v3.MemoryAllocatorManager
  pub fn has_memory_allocator_manager(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(38)
    }
  }
  pub fn clear_memory_allocator_manager(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        38
      );
    }
  }
  pub fn memory_allocator_manager_opt(&self) -> ::std::option::Option<super::MemoryAllocatorManagerView<'_>> {
    self.has_memory_allocator_manager().then(|| self.memory_allocator_manager())
  }
  pub fn memory_allocator_manager(&self) -> super::MemoryAllocatorManagerView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(38)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::MemoryAllocatorManagerView::default())
  }
  pub fn memory_allocator_manager_mut(&mut self) -> super::MemoryAllocatorManagerMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         38, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_memory_allocator_manager(&mut self,
    val: impl ::protobuf::IntoProxied<super::MemoryAllocatorManager>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        38,
        val
      );
    }
  }

  pub fn stats_flush(&self) -> super::bootstrap::StatsFlushOneof<'_> {
    match &self.stats_flush_case() {
      super::bootstrap::StatsFlushCase::StatsFlushOnAdmin =>
          super::bootstrap::StatsFlushOneof::StatsFlushOnAdmin(self.stats_flush_on_admin()),
      _ => super::bootstrap::StatsFlushOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn stats_flush_case(&self) -> super::bootstrap::StatsFlushCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(26);
      super::bootstrap::StatsFlushCase::try_from(field_num).unwrap_unchecked()
    }
  }
  pub fn stats_eviction(&self) -> super::bootstrap::StatsEvictionOneof<'_> {
    match &self.stats_eviction_case() {
      super::bootstrap::StatsEvictionCase::StatsEvictionInterval =>
          super::bootstrap::StatsEvictionOneof::StatsEvictionInterval(self.stats_eviction_interval()),
      _ => super::bootstrap::StatsEvictionOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn stats_eviction_case(&self) -> super::bootstrap::StatsEvictionCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(39);
      super::bootstrap::StatsEvictionCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `BootstrapMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for BootstrapMut<'_> {}

// SAFETY:
// - `BootstrapMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for BootstrapMut<'_> {}

impl<'msg> ::protobuf::AsView for BootstrapMut<'msg> {
  type Proxied = Bootstrap;
  fn as_view(&self) -> ::protobuf::View<'_, Bootstrap> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for BootstrapMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Bootstrap>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for BootstrapMut<'msg> {
  type MutProxied = Bootstrap;
  fn as_mut(&mut self) -> BootstrapMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for BootstrapMut<'msg> {
  fn into_mut<'shorter>(self) -> BootstrapMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Bootstrap {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Bootstrap> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> BootstrapView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> BootstrapMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // node: optional message envoy.config.core.v3.Node
  pub fn has_node(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_node(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn node_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::base::NodeView<'_>> {
    self.has_node().then(|| self.node())
  }
  pub fn node(&self) -> crate::xds::generated::envoy::config::core::v3::base::NodeView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::base::NodeView::default())
  }
  pub fn node_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::base::NodeMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         0, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_node(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::base::Node>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // node_context_params: repeated string
  pub fn node_context_params(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        23
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn node_context_params_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        23,
        self.inner.arena()
      ).expect("alloc should not fail");
      ::protobuf::RepeatedMut::from_inner(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::InnerRepeatedMut::new(
          raw_array, self.inner.arena(),
        ),
      )
    }
  }
  pub fn set_node_context_params(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        23,
        src);
    }
  }

  // static_resources: optional message envoy.config.bootstrap.v3.Bootstrap.StaticResources
  pub fn has_static_resources(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_static_resources(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn static_resources_opt(&self) -> ::std::option::Option<super::bootstrap::StaticResourcesView<'_>> {
    self.has_static_resources().then(|| self.static_resources())
  }
  pub fn static_resources(&self) -> super::bootstrap::StaticResourcesView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::bootstrap::StaticResourcesView::default())
  }
  pub fn static_resources_mut(&mut self) -> super::bootstrap::StaticResourcesMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         1, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_static_resources(&mut self,
    val: impl ::protobuf::IntoProxied<super::bootstrap::StaticResources>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // dynamic_resources: optional message envoy.config.bootstrap.v3.Bootstrap.DynamicResources
  pub fn has_dynamic_resources(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_dynamic_resources(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn dynamic_resources_opt(&self) -> ::std::option::Option<super::bootstrap::DynamicResourcesView<'_>> {
    self.has_dynamic_resources().then(|| self.dynamic_resources())
  }
  pub fn dynamic_resources(&self) -> super::bootstrap::DynamicResourcesView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::bootstrap::DynamicResourcesView::default())
  }
  pub fn dynamic_resources_mut(&mut self) -> super::bootstrap::DynamicResourcesMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         2, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_dynamic_resources(&mut self,
    val: impl ::protobuf::IntoProxied<super::bootstrap::DynamicResources>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // cluster_manager: optional message envoy.config.bootstrap.v3.ClusterManager
  pub fn has_cluster_manager(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_cluster_manager(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn cluster_manager_opt(&self) -> ::std::option::Option<super::ClusterManagerView<'_>> {
    self.has_cluster_manager().then(|| self.cluster_manager())
  }
  pub fn cluster_manager(&self) -> super::ClusterManagerView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ClusterManagerView::default())
  }
  pub fn cluster_manager_mut(&mut self) -> super::ClusterManagerMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         3, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_cluster_manager(&mut self,
    val: impl ::protobuf::IntoProxied<super::ClusterManager>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // hds_config: optional message envoy.config.core.v3.ApiConfigSource
  pub fn has_hds_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(11)
    }
  }
  pub fn clear_hds_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        11
      );
    }
  }
  pub fn hds_config_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::config_source::ApiConfigSourceView<'_>> {
    self.has_hds_config().then(|| self.hds_config())
  }
  pub fn hds_config(&self) -> crate::xds::generated::envoy::config::core::v3::config_source::ApiConfigSourceView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(11)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::config_source::ApiConfigSourceView::default())
  }
  pub fn hds_config_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::config_source::ApiConfigSourceMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         11, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_hds_config(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::config_source::ApiConfigSource>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        11,
        val
      );
    }
  }

  // flags_path: optional string
  pub fn flags_path(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        4, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_flags_path(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val);
    }
  }

  // stats_sinks: repeated message envoy.config.metrics.v3.StatsSink
  pub fn stats_sinks(&self) -> ::protobuf::RepeatedView<'_, crate::xds::generated::envoy::config::metrics::v3::stats::StatsSink> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        5
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::metrics::v3::stats::StatsSink>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn stats_sinks_mut(&mut self) -> ::protobuf::RepeatedMut<'_, crate::xds::generated::envoy::config::metrics::v3::stats::StatsSink> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        5,
        self.inner.arena()
      ).expect("alloc should not fail");
      ::protobuf::RepeatedMut::from_inner(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::InnerRepeatedMut::new(
          raw_array, self.inner.arena(),
        ),
      )
    }
  }
  pub fn set_stats_sinks(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<crate::xds::generated::envoy::config::metrics::v3::stats::StatsSink>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        src);
    }
  }

  // deferred_stat_options: optional message envoy.config.bootstrap.v3.Bootstrap.DeferredStatOptions
  pub fn has_deferred_stat_options(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(36)
    }
  }
  pub fn clear_deferred_stat_options(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        36
      );
    }
  }
  pub fn deferred_stat_options_opt(&self) -> ::std::option::Option<super::bootstrap::DeferredStatOptionsView<'_>> {
    self.has_deferred_stat_options().then(|| self.deferred_stat_options())
  }
  pub fn deferred_stat_options(&self) -> super::bootstrap::DeferredStatOptionsView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(36)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::bootstrap::DeferredStatOptionsView::default())
  }
  pub fn deferred_stat_options_mut(&mut self) -> super::bootstrap::DeferredStatOptionsMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         36, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_deferred_stat_options(&mut self,
    val: impl ::protobuf::IntoProxied<super::bootstrap::DeferredStatOptions>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        36,
        val
      );
    }
  }

  // stats_config: optional message envoy.config.metrics.v3.StatsConfig
  pub fn has_stats_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(10)
    }
  }
  pub fn clear_stats_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        10
      );
    }
  }
  pub fn stats_config_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::metrics::v3::stats::StatsConfigView<'_>> {
    self.has_stats_config().then(|| self.stats_config())
  }
  pub fn stats_config(&self) -> crate::xds::generated::envoy::config::metrics::v3::stats::StatsConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(10)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::metrics::v3::stats::StatsConfigView::default())
  }
  pub fn stats_config_mut(&mut self) -> crate::xds::generated::envoy::config::metrics::v3::stats::StatsConfigMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         10, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_stats_config(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::metrics::v3::stats::StatsConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        10,
        val
      );
    }
  }

  // stats_flush_interval: optional message google.protobuf.Duration
  pub fn has_stats_flush_interval(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn clear_stats_flush_interval(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        6
      );
    }
  }
  pub fn stats_flush_interval_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_stats_flush_interval().then(|| self.stats_flush_interval())
  }
  pub fn stats_flush_interval(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(6)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn stats_flush_interval_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         6, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_stats_flush_interval(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        val
      );
    }
  }

  // stats_flush_on_admin: optional bool
  pub fn has_stats_flush_on_admin(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(26)
    }
  }
  pub fn clear_stats_flush_on_admin(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        26
      );
    }
  }
  pub fn stats_flush_on_admin_opt(&self) -> ::std::option::Option<bool> {
    self.has_stats_flush_on_admin().then(|| self.stats_flush_on_admin())
  }
  pub fn stats_flush_on_admin(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        26, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_stats_flush_on_admin(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        26, val.into()
      )
    }
  }

  // stats_eviction_interval: optional message google.protobuf.Duration
  pub fn has_stats_eviction_interval(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(39)
    }
  }
  pub fn clear_stats_eviction_interval(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        39
      );
    }
  }
  pub fn stats_eviction_interval_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_stats_eviction_interval().then(|| self.stats_eviction_interval())
  }
  pub fn stats_eviction_interval(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(39)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn stats_eviction_interval_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         39, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_stats_eviction_interval(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        39,
        val
      );
    }
  }

  // watchdog: optional message envoy.config.bootstrap.v3.Watchdog
  pub fn has_watchdog(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn clear_watchdog(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        7
      );
    }
  }
  pub fn watchdog_opt(&self) -> ::std::option::Option<super::WatchdogView<'_>> {
    self.has_watchdog().then(|| self.watchdog())
  }
  pub fn watchdog(&self) -> super::WatchdogView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(7)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::WatchdogView::default())
  }
  pub fn watchdog_mut(&mut self) -> super::WatchdogMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         7, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_watchdog(&mut self,
    val: impl ::protobuf::IntoProxied<super::Watchdog>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        7,
        val
      );
    }
  }

  // watchdogs: optional message envoy.config.bootstrap.v3.Watchdogs
  pub fn has_watchdogs(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(24)
    }
  }
  pub fn clear_watchdogs(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        24
      );
    }
  }
  pub fn watchdogs_opt(&self) -> ::std::option::Option<super::WatchdogsView<'_>> {
    self.has_watchdogs().then(|| self.watchdogs())
  }
  pub fn watchdogs(&self) -> super::WatchdogsView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(24)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::WatchdogsView::default())
  }
  pub fn watchdogs_mut(&mut self) -> super::WatchdogsMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         24, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_watchdogs(&mut self,
    val: impl ::protobuf::IntoProxied<super::Watchdogs>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        24,
        val
      );
    }
  }

  // tracing: optional message envoy.config.trace.v3.Tracing
  pub fn has_tracing(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(8)
    }
  }
  pub fn clear_tracing(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        8
      );
    }
  }
  pub fn tracing_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::trace::v3::http_tracer::TracingView<'_>> {
    self.has_tracing().then(|| self.tracing())
  }
  pub fn tracing(&self) -> crate::xds::generated::envoy::config::trace::v3::http_tracer::TracingView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(8)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::trace::v3::http_tracer::TracingView::default())
  }
  pub fn tracing_mut(&mut self) -> crate::xds::generated::envoy::config::trace::v3::http_tracer::TracingMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         8, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_tracing(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::trace::v3::http_tracer::Tracing>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        8,
        val
      );
    }
  }

  // layered_runtime: optional message envoy.config.bootstrap.v3.LayeredRuntime
  pub fn has_layered_runtime(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(14)
    }
  }
  pub fn clear_layered_runtime(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        14
      );
    }
  }
  pub fn layered_runtime_opt(&self) -> ::std::option::Option<super::LayeredRuntimeView<'_>> {
    self.has_layered_runtime().then(|| self.layered_runtime())
  }
  pub fn layered_runtime(&self) -> super::LayeredRuntimeView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(14)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::LayeredRuntimeView::default())
  }
  pub fn layered_runtime_mut(&mut self) -> super::LayeredRuntimeMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         14, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_layered_runtime(&mut self,
    val: impl ::protobuf::IntoProxied<super::LayeredRuntime>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        14,
        val
      );
    }
  }

  // admin: optional message envoy.config.bootstrap.v3.Admin
  pub fn has_admin(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(9)
    }
  }
  pub fn clear_admin(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        9
      );
    }
  }
  pub fn admin_opt(&self) -> ::std::option::Option<super::AdminView<'_>> {
    self.has_admin().then(|| self.admin())
  }
  pub fn admin(&self) -> super::AdminView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(9)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::AdminView::default())
  }
  pub fn admin_mut(&mut self) -> super::AdminMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         9, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_admin(&mut self,
    val: impl ::protobuf::IntoProxied<super::Admin>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        9,
        val
      );
    }
  }

  // overload_manager: optional message envoy.config.overload.v3.OverloadManager
  pub fn has_overload_manager(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(12)
    }
  }
  pub fn clear_overload_manager(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        12
      );
    }
  }
  pub fn overload_manager_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::overload::v3::overload::OverloadManagerView<'_>> {
    self.has_overload_manager().then(|| self.overload_manager())
  }
  pub fn overload_manager(&self) -> crate::xds::generated::envoy::config::overload::v3::overload::OverloadManagerView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(12)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::overload::v3::overload::OverloadManagerView::default())
  }
  pub fn overload_manager_mut(&mut self) -> crate::xds::generated::envoy::config::overload::v3::overload::OverloadManagerMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         12, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_overload_manager(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::overload::v3::overload::OverloadManager>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        12,
        val
      );
    }
  }

  // enable_dispatcher_stats: optional bool
  pub fn enable_dispatcher_stats(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        13, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_enable_dispatcher_stats(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        13, val.into()
      )
    }
  }

  // header_prefix: optional string
  pub fn header_prefix(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        15, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_header_prefix(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        15,
        val);
    }
  }

  // stats_server_version_override: optional message google.protobuf.UInt64Value
  pub fn has_stats_server_version_override(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(16)
    }
  }
  pub fn clear_stats_server_version_override(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        16
      );
    }
  }
  pub fn stats_server_version_override_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::UInt64ValueView<'_>> {
    self.has_stats_server_version_override().then(|| self.stats_server_version_override())
  }
  pub fn stats_server_version_override(&self) -> ::protobuf_well_known_types::UInt64ValueView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(16)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::UInt64ValueView::default())
  }
  pub fn stats_server_version_override_mut(&mut self) -> ::protobuf_well_known_types::UInt64ValueMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         16, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_stats_server_version_override(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::UInt64Value>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        16,
        val
      );
    }
  }

  // use_tcp_for_dns_lookups: optional bool
  pub fn use_tcp_for_dns_lookups(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        17, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_use_tcp_for_dns_lookups(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        17, val.into()
      )
    }
  }

  // dns_resolution_config: optional message envoy.config.core.v3.DnsResolutionConfig
  pub fn has_dns_resolution_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(27)
    }
  }
  pub fn clear_dns_resolution_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        27
      );
    }
  }
  pub fn dns_resolution_config_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::resolver::DnsResolutionConfigView<'_>> {
    self.has_dns_resolution_config().then(|| self.dns_resolution_config())
  }
  pub fn dns_resolution_config(&self) -> crate::xds::generated::envoy::config::core::v3::resolver::DnsResolutionConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(27)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::resolver::DnsResolutionConfigView::default())
  }
  pub fn dns_resolution_config_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::resolver::DnsResolutionConfigMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         27, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_dns_resolution_config(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::resolver::DnsResolutionConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        27,
        val
      );
    }
  }

  // typed_dns_resolver_config: optional message envoy.config.core.v3.TypedExtensionConfig
  pub fn has_typed_dns_resolver_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(28)
    }
  }
  pub fn clear_typed_dns_resolver_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        28
      );
    }
  }
  pub fn typed_dns_resolver_config_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'_>> {
    self.has_typed_dns_resolver_config().then(|| self.typed_dns_resolver_config())
  }
  pub fn typed_dns_resolver_config(&self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(28)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView::default())
  }
  pub fn typed_dns_resolver_config_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         28, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_typed_dns_resolver_config(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        28,
        val
      );
    }
  }

  // bootstrap_extensions: repeated message envoy.config.core.v3.TypedExtensionConfig
  pub fn bootstrap_extensions(&self) -> ::protobuf::RepeatedView<'_, crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        18
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn bootstrap_extensions_mut(&mut self) -> ::protobuf::RepeatedMut<'_, crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        18,
        self.inner.arena()
      ).expect("alloc should not fail");
      ::protobuf::RepeatedMut::from_inner(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::InnerRepeatedMut::new(
          raw_array, self.inner.arena(),
        ),
      )
    }
  }
  pub fn set_bootstrap_extensions(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        18,
        src);
    }
  }

  // fatal_actions: repeated message envoy.config.bootstrap.v3.FatalAction
  pub fn fatal_actions(&self) -> ::protobuf::RepeatedView<'_, super::FatalAction> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        25
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::FatalAction>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn fatal_actions_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::FatalAction> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        25,
        self.inner.arena()
      ).expect("alloc should not fail");
      ::protobuf::RepeatedMut::from_inner(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::InnerRepeatedMut::new(
          raw_array, self.inner.arena(),
        ),
      )
    }
  }
  pub fn set_fatal_actions(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::FatalAction>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        25,
        src);
    }
  }

  // config_sources: repeated message envoy.config.core.v3.ConfigSource
  pub fn config_sources(&self) -> ::protobuf::RepeatedView<'_, crate::xds::generated::envoy::config::core::v3::config_source::ConfigSource> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        19
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::core::v3::config_source::ConfigSource>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn config_sources_mut(&mut self) -> ::protobuf::RepeatedMut<'_, crate::xds::generated::envoy::config::core::v3::config_source::ConfigSource> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        19,
        self.inner.arena()
      ).expect("alloc should not fail");
      ::protobuf::RepeatedMut::from_inner(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::InnerRepeatedMut::new(
          raw_array, self.inner.arena(),
        ),
      )
    }
  }
  pub fn set_config_sources(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<crate::xds::generated::envoy::config::core::v3::config_source::ConfigSource>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        19,
        src);
    }
  }

  // default_config_source: optional message envoy.config.core.v3.ConfigSource
  pub fn has_default_config_source(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(20)
    }
  }
  pub fn clear_default_config_source(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        20
      );
    }
  }
  pub fn default_config_source_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::config_source::ConfigSourceView<'_>> {
    self.has_default_config_source().then(|| self.default_config_source())
  }
  pub fn default_config_source(&self) -> crate::xds::generated::envoy::config::core::v3::config_source::ConfigSourceView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(20)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::config_source::ConfigSourceView::default())
  }
  pub fn default_config_source_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::config_source::ConfigSourceMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         20, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_default_config_source(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::config_source::ConfigSource>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        20,
        val
      );
    }
  }

  // default_socket_interface: optional string
  pub fn default_socket_interface(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        21, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_default_socket_interface(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        21,
        val);
    }
  }

  // certificate_provider_instances: repeated message envoy.config.bootstrap.v3.Bootstrap.CertificateProviderInstancesEntry
  pub fn certificate_provider_instances(&self)
    -> ::protobuf::MapView<'_, ::protobuf::ProtoString, crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig> {
    unsafe {
      <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(&self, ::protobuf::__internal::Private)
        .get_map_at_index(22)
        .map_or_else(
          ::protobuf::__internal::runtime::empty_map::<::protobuf::ProtoString, crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig>,
          |raw| ::protobuf::MapView::from_raw(::protobuf::__internal::Private, raw)
        )
    }
  }
  pub fn certificate_provider_instances_mut(&mut self)
    -> ::protobuf::MapMut<'_, ::protobuf::ProtoString, crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig> {
    unsafe {
      let raw_map = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtrMut>::get_ptr_mut(self, ::protobuf::__internal::Private)
        .get_or_create_mutable_map_at_index(
          22, self.inner.arena()).unwrap();
      let inner = ::protobuf::__internal::runtime::InnerMapMut::new(
        raw_map, self.inner.arena());
      ::protobuf::MapMut::from_inner(::protobuf::__internal::Private, inner)
    }
  }
  pub fn set_certificate_provider_instances(
      &mut self,
      src: impl ::protobuf::IntoProxied<::protobuf::Map<::protobuf::ProtoString, crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_map_field(
        ::protobuf::AsMut::as_mut(self).inner,
        22,
        src);
    }
  }

  // inline_headers: repeated message envoy.config.bootstrap.v3.CustomInlineHeader
  pub fn inline_headers(&self) -> ::protobuf::RepeatedView<'_, super::CustomInlineHeader> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        29
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::CustomInlineHeader>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn inline_headers_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::CustomInlineHeader> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        29,
        self.inner.arena()
      ).expect("alloc should not fail");
      ::protobuf::RepeatedMut::from_inner(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::InnerRepeatedMut::new(
          raw_array, self.inner.arena(),
        ),
      )
    }
  }
  pub fn set_inline_headers(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::CustomInlineHeader>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        29,
        src);
    }
  }

  // perf_tracing_file_path: optional string
  pub fn perf_tracing_file_path(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        30, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_perf_tracing_file_path(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        30,
        val);
    }
  }

  // default_regex_engine: optional message envoy.config.core.v3.TypedExtensionConfig
  pub fn has_default_regex_engine(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(31)
    }
  }
  pub fn clear_default_regex_engine(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        31
      );
    }
  }
  pub fn default_regex_engine_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'_>> {
    self.has_default_regex_engine().then(|| self.default_regex_engine())
  }
  pub fn default_regex_engine(&self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(31)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView::default())
  }
  pub fn default_regex_engine_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         31, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_default_regex_engine(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        31,
        val
      );
    }
  }

  // xds_delegate_extension: optional message envoy.config.core.v3.TypedExtensionConfig
  pub fn has_xds_delegate_extension(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(32)
    }
  }
  pub fn clear_xds_delegate_extension(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        32
      );
    }
  }
  pub fn xds_delegate_extension_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'_>> {
    self.has_xds_delegate_extension().then(|| self.xds_delegate_extension())
  }
  pub fn xds_delegate_extension(&self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(32)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView::default())
  }
  pub fn xds_delegate_extension_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         32, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_xds_delegate_extension(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        32,
        val
      );
    }
  }

  // xds_config_tracker_extension: optional message envoy.config.core.v3.TypedExtensionConfig
  pub fn has_xds_config_tracker_extension(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(33)
    }
  }
  pub fn clear_xds_config_tracker_extension(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        33
      );
    }
  }
  pub fn xds_config_tracker_extension_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'_>> {
    self.has_xds_config_tracker_extension().then(|| self.xds_config_tracker_extension())
  }
  pub fn xds_config_tracker_extension(&self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(33)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView::default())
  }
  pub fn xds_config_tracker_extension_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         33, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_xds_config_tracker_extension(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        33,
        val
      );
    }
  }

  // listener_manager: optional message envoy.config.core.v3.TypedExtensionConfig
  pub fn has_listener_manager(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(34)
    }
  }
  pub fn clear_listener_manager(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        34
      );
    }
  }
  pub fn listener_manager_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'_>> {
    self.has_listener_manager().then(|| self.listener_manager())
  }
  pub fn listener_manager(&self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(34)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView::default())
  }
  pub fn listener_manager_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         34, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_listener_manager(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        34,
        val
      );
    }
  }

  // application_log_config: optional message envoy.config.bootstrap.v3.Bootstrap.ApplicationLogConfig
  pub fn has_application_log_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(35)
    }
  }
  pub fn clear_application_log_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        35
      );
    }
  }
  pub fn application_log_config_opt(&self) -> ::std::option::Option<super::bootstrap::ApplicationLogConfigView<'_>> {
    self.has_application_log_config().then(|| self.application_log_config())
  }
  pub fn application_log_config(&self) -> super::bootstrap::ApplicationLogConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(35)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::bootstrap::ApplicationLogConfigView::default())
  }
  pub fn application_log_config_mut(&mut self) -> super::bootstrap::ApplicationLogConfigMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         35, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_application_log_config(&mut self,
    val: impl ::protobuf::IntoProxied<super::bootstrap::ApplicationLogConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        35,
        val
      );
    }
  }

  // grpc_async_client_manager_config: optional message envoy.config.bootstrap.v3.Bootstrap.GrpcAsyncClientManagerConfig
  pub fn has_grpc_async_client_manager_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(37)
    }
  }
  pub fn clear_grpc_async_client_manager_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        37
      );
    }
  }
  pub fn grpc_async_client_manager_config_opt(&self) -> ::std::option::Option<super::bootstrap::GrpcAsyncClientManagerConfigView<'_>> {
    self.has_grpc_async_client_manager_config().then(|| self.grpc_async_client_manager_config())
  }
  pub fn grpc_async_client_manager_config(&self) -> super::bootstrap::GrpcAsyncClientManagerConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(37)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::bootstrap::GrpcAsyncClientManagerConfigView::default())
  }
  pub fn grpc_async_client_manager_config_mut(&mut self) -> super::bootstrap::GrpcAsyncClientManagerConfigMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         37, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_grpc_async_client_manager_config(&mut self,
    val: impl ::protobuf::IntoProxied<super::bootstrap::GrpcAsyncClientManagerConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        37,
        val
      );
    }
  }

  // memory_allocator_manager: optional message envoy.config.bootstrap.v3.MemoryAllocatorManager
  pub fn has_memory_allocator_manager(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(38)
    }
  }
  pub fn clear_memory_allocator_manager(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        38
      );
    }
  }
  pub fn memory_allocator_manager_opt(&self) -> ::std::option::Option<super::MemoryAllocatorManagerView<'_>> {
    self.has_memory_allocator_manager().then(|| self.memory_allocator_manager())
  }
  pub fn memory_allocator_manager(&self) -> super::MemoryAllocatorManagerView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(38)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::MemoryAllocatorManagerView::default())
  }
  pub fn memory_allocator_manager_mut(&mut self) -> super::MemoryAllocatorManagerMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         38, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_memory_allocator_manager(&mut self,
    val: impl ::protobuf::IntoProxied<super::MemoryAllocatorManager>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        38,
        val
      );
    }
  }

  pub fn stats_flush(&self) -> super::bootstrap::StatsFlushOneof<'_> {
    match &self.stats_flush_case() {
      super::bootstrap::StatsFlushCase::StatsFlushOnAdmin =>
          super::bootstrap::StatsFlushOneof::StatsFlushOnAdmin(self.stats_flush_on_admin()),
      _ => super::bootstrap::StatsFlushOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn stats_flush_case(&self) -> super::bootstrap::StatsFlushCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(26);
      super::bootstrap::StatsFlushCase::try_from(field_num).unwrap_unchecked()
    }
  }
  pub fn stats_eviction(&self) -> super::bootstrap::StatsEvictionOneof<'_> {
    match &self.stats_eviction_case() {
      super::bootstrap::StatsEvictionCase::StatsEvictionInterval =>
          super::bootstrap::StatsEvictionOneof::StatsEvictionInterval(self.stats_eviction_interval()),
      _ => super::bootstrap::StatsEvictionOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn stats_eviction_case(&self) -> super::bootstrap::StatsEvictionCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(39);
      super::bootstrap::StatsEvictionCase::try_from(field_num).unwrap_unchecked()
    }
  }
}  // impl Bootstrap

impl ::std::ops::Drop for Bootstrap {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Bootstrap {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Bootstrap {
  type Proxied = Self;
  fn as_view(&self) -> BootstrapView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Bootstrap {
  type MutProxied = Self;
  fn as_mut(&mut self) -> BootstrapMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Bootstrap {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__bootstrap__v3__Bootstrap_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$33331XG333b3333/P31X3/PGG31XGET3G/33G1X333333333^?~L");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__bootstrap__v3__Bootstrap_msg_init.0, &[<crate::xds::generated::envoy::config::core::v3::base::Node as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::bootstrap::StaticResources as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::bootstrap::DynamicResources as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::ClusterManager as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::metrics::v3::stats::StatsSink as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::Duration as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::Watchdog as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::trace::v3::http_tracer::Tracing as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::Admin as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::metrics::v3::stats::StatsConfig as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::core::v3::config_source::ApiConfigSource as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::overload::v3::overload::OverloadManager as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::LayeredRuntime as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::UInt64Value as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::core::v3::config_source::ConfigSource as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::core::v3::config_source::ConfigSource as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::bootstrap::CertificateProviderInstancesEntry as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::Watchdogs as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::FatalAction as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::core::v3::resolver::DnsResolutionConfig as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::CustomInlineHeader as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::bootstrap::ApplicationLogConfig as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::bootstrap::DeferredStatOptions as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::bootstrap::GrpcAsyncClientManagerConfig as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::MemoryAllocatorManager as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::Duration as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__bootstrap__v3__Bootstrap_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Bootstrap {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Bootstrap {
  type Msg = Bootstrap;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Bootstrap> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Bootstrap {
  type Msg = Bootstrap;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Bootstrap> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for BootstrapMut<'_> {
  type Msg = Bootstrap;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Bootstrap> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for BootstrapMut<'_> {
  type Msg = Bootstrap;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Bootstrap> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for BootstrapView<'_> {
  type Msg = Bootstrap;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Bootstrap> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for BootstrapMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod bootstrap {// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__bootstrap__v3__Bootstrap__StaticResources_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct StaticResources {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<StaticResources>
}

impl ::protobuf::Message for StaticResources {
  type MessageView<'msg> = StaticResourcesView<'msg>;
  type MessageMut<'msg> = StaticResourcesMut<'msg>;
}

impl ::std::default::Default for StaticResources {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for StaticResources {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `StaticResources` is `Sync` because it does not implement interior mutability.
//    Neither does `StaticResourcesMut`.
unsafe impl ::std::marker::Sync for StaticResources {}

// SAFETY:
// - `StaticResources` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for StaticResources {}

impl ::protobuf::Proxied for StaticResources {
  type View<'msg> = StaticResourcesView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for StaticResources {}

impl ::protobuf::MutProxied for StaticResources {
  type Mut<'msg> = StaticResourcesMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct StaticResourcesView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, StaticResources>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for StaticResourcesView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for StaticResourcesView<'msg> {
  type Message = StaticResources;
}

impl ::std::fmt::Debug for StaticResourcesView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for StaticResourcesView<'_> {
  fn default() -> StaticResourcesView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, StaticResources>> for StaticResourcesView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, StaticResources>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> StaticResourcesView<'msg> {

  pub fn to_owned(&self) -> StaticResources {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // listeners: repeated message envoy.config.listener.v3.Listener
  pub fn listeners(self) -> ::protobuf::RepeatedView<'msg, crate::xds::generated::envoy::config::listener::v3::listener::Listener> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::listener::v3::listener::Listener>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // clusters: repeated message envoy.config.cluster.v3.Cluster
  pub fn clusters(self) -> ::protobuf::RepeatedView<'msg, crate::xds::generated::envoy::config::cluster::v3::cluster::Cluster> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::cluster::v3::cluster::Cluster>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // secrets: repeated message envoy.extensions.transport_sockets.tls.v3.Secret
  pub fn secrets(self) -> ::protobuf::RepeatedView<'msg, crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::secret::Secret> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        2
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::secret::Secret>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

}

// SAFETY:
// - `StaticResourcesView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for StaticResourcesView<'_> {}

// SAFETY:
// - `StaticResourcesView` is `Send` because while its alive a `StaticResourcesMut` cannot.
// - `StaticResourcesView` does not use thread-local data.
unsafe impl ::std::marker::Send for StaticResourcesView<'_> {}

impl<'msg> ::protobuf::AsView for StaticResourcesView<'msg> {
  type Proxied = StaticResources;
  fn as_view(&self) -> ::protobuf::View<'msg, StaticResources> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for StaticResourcesView<'msg> {
  fn into_view<'shorter>(self) -> StaticResourcesView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<StaticResources> for StaticResourcesView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> StaticResources {
    let mut dst = StaticResources::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<StaticResources> for StaticResourcesMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> StaticResources {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for StaticResources {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for StaticResourcesView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for StaticResourcesMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct StaticResourcesMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, StaticResources>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for StaticResourcesMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for StaticResourcesMut<'msg> {
  type Message = StaticResources;
}

impl ::std::fmt::Debug for StaticResourcesMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, StaticResources>> for StaticResourcesMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, StaticResources>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> StaticResourcesMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, StaticResources> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> StaticResources {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // listeners: repeated message envoy.config.listener.v3.Listener
  pub fn listeners(&self) -> ::protobuf::RepeatedView<'_, crate::xds::generated::envoy::config::listener::v3::listener::Listener> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::listener::v3::listener::Listener>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn listeners_mut(&mut self) -> ::protobuf::RepeatedMut<'_, crate::xds::generated::envoy::config::listener::v3::listener::Listener> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        0,
        self.inner.arena()
      ).expect("alloc should not fail");
      ::protobuf::RepeatedMut::from_inner(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::InnerRepeatedMut::new(
          raw_array, self.inner.arena(),
        ),
      )
    }
  }
  pub fn set_listeners(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<crate::xds::generated::envoy::config::listener::v3::listener::Listener>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

  // clusters: repeated message envoy.config.cluster.v3.Cluster
  pub fn clusters(&self) -> ::protobuf::RepeatedView<'_, crate::xds::generated::envoy::config::cluster::v3::cluster::Cluster> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::cluster::v3::cluster::Cluster>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn clusters_mut(&mut self) -> ::protobuf::RepeatedMut<'_, crate::xds::generated::envoy::config::cluster::v3::cluster::Cluster> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        1,
        self.inner.arena()
      ).expect("alloc should not fail");
      ::protobuf::RepeatedMut::from_inner(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::InnerRepeatedMut::new(
          raw_array, self.inner.arena(),
        ),
      )
    }
  }
  pub fn set_clusters(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<crate::xds::generated::envoy::config::cluster::v3::cluster::Cluster>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
    }
  }

  // secrets: repeated message envoy.extensions.transport_sockets.tls.v3.Secret
  pub fn secrets(&self) -> ::protobuf::RepeatedView<'_, crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::secret::Secret> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        2
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::secret::Secret>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn secrets_mut(&mut self) -> ::protobuf::RepeatedMut<'_, crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::secret::Secret> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        2,
        self.inner.arena()
      ).expect("alloc should not fail");
      ::protobuf::RepeatedMut::from_inner(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::InnerRepeatedMut::new(
          raw_array, self.inner.arena(),
        ),
      )
    }
  }
  pub fn set_secrets(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::secret::Secret>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        src);
    }
  }

}

// SAFETY:
// - `StaticResourcesMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for StaticResourcesMut<'_> {}

// SAFETY:
// - `StaticResourcesMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for StaticResourcesMut<'_> {}

impl<'msg> ::protobuf::AsView for StaticResourcesMut<'msg> {
  type Proxied = StaticResources;
  fn as_view(&self) -> ::protobuf::View<'_, StaticResources> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for StaticResourcesMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, StaticResources>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for StaticResourcesMut<'msg> {
  type MutProxied = StaticResources;
  fn as_mut(&mut self) -> StaticResourcesMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for StaticResourcesMut<'msg> {
  fn into_mut<'shorter>(self) -> StaticResourcesMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl StaticResources {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, StaticResources> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> StaticResourcesView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> StaticResourcesMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // listeners: repeated message envoy.config.listener.v3.Listener
  pub fn listeners(&self) -> ::protobuf::RepeatedView<'_, crate::xds::generated::envoy::config::listener::v3::listener::Listener> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::listener::v3::listener::Listener>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn listeners_mut(&mut self) -> ::protobuf::RepeatedMut<'_, crate::xds::generated::envoy::config::listener::v3::listener::Listener> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        0,
        self.inner.arena()
      ).expect("alloc should not fail");
      ::protobuf::RepeatedMut::from_inner(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::InnerRepeatedMut::new(
          raw_array, self.inner.arena(),
        ),
      )
    }
  }
  pub fn set_listeners(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<crate::xds::generated::envoy::config::listener::v3::listener::Listener>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

  // clusters: repeated message envoy.config.cluster.v3.Cluster
  pub fn clusters(&self) -> ::protobuf::RepeatedView<'_, crate::xds::generated::envoy::config::cluster::v3::cluster::Cluster> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::cluster::v3::cluster::Cluster>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn clusters_mut(&mut self) -> ::protobuf::RepeatedMut<'_, crate::xds::generated::envoy::config::cluster::v3::cluster::Cluster> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        1,
        self.inner.arena()
      ).expect("alloc should not fail");
      ::protobuf::RepeatedMut::from_inner(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::InnerRepeatedMut::new(
          raw_array, self.inner.arena(),
        ),
      )
    }
  }
  pub fn set_clusters(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<crate::xds::generated::envoy::config::cluster::v3::cluster::Cluster>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
    }
  }

  // secrets: repeated message envoy.extensions.transport_sockets.tls.v3.Secret
  pub fn secrets(&self) -> ::protobuf::RepeatedView<'_, crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::secret::Secret> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        2
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::secret::Secret>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn secrets_mut(&mut self) -> ::protobuf::RepeatedMut<'_, crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::secret::Secret> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        2,
        self.inner.arena()
      ).expect("alloc should not fail");
      ::protobuf::RepeatedMut::from_inner(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::InnerRepeatedMut::new(
          raw_array, self.inner.arena(),
        ),
      )
    }
  }
  pub fn set_secrets(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::secret::Secret>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        src);
    }
  }

}  // impl StaticResources

impl ::std::ops::Drop for StaticResources {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for StaticResources {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for StaticResources {
  type Proxied = Self;
  fn as_view(&self) -> StaticResourcesView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for StaticResources {
  type MutProxied = Self;
  fn as_mut(&mut self) -> StaticResourcesMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for StaticResources {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::bootstrap::envoy__config__bootstrap__v3__Bootstrap__StaticResources_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$GGG");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::bootstrap::envoy__config__bootstrap__v3__Bootstrap__StaticResources_msg_init.0, &[<crate::xds::generated::envoy::config::listener::v3::listener::Listener as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::cluster::v3::cluster::Cluster as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::extensions::transport_sockets::tls::v3::secret::Secret as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::bootstrap::envoy__config__bootstrap__v3__Bootstrap__StaticResources_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for StaticResources {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for StaticResources {
  type Msg = StaticResources;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<StaticResources> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for StaticResources {
  type Msg = StaticResources;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<StaticResources> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for StaticResourcesMut<'_> {
  type Msg = StaticResources;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<StaticResources> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for StaticResourcesMut<'_> {
  type Msg = StaticResources;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<StaticResources> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for StaticResourcesView<'_> {
  type Msg = StaticResources;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<StaticResources> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for StaticResourcesMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__bootstrap__v3__Bootstrap__DynamicResources_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct DynamicResources {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<DynamicResources>
}

impl ::protobuf::Message for DynamicResources {
  type MessageView<'msg> = DynamicResourcesView<'msg>;
  type MessageMut<'msg> = DynamicResourcesMut<'msg>;
}

impl ::std::default::Default for DynamicResources {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for DynamicResources {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `DynamicResources` is `Sync` because it does not implement interior mutability.
//    Neither does `DynamicResourcesMut`.
unsafe impl ::std::marker::Sync for DynamicResources {}

// SAFETY:
// - `DynamicResources` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for DynamicResources {}

impl ::protobuf::Proxied for DynamicResources {
  type View<'msg> = DynamicResourcesView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for DynamicResources {}

impl ::protobuf::MutProxied for DynamicResources {
  type Mut<'msg> = DynamicResourcesMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct DynamicResourcesView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, DynamicResources>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for DynamicResourcesView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for DynamicResourcesView<'msg> {
  type Message = DynamicResources;
}

impl ::std::fmt::Debug for DynamicResourcesView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for DynamicResourcesView<'_> {
  fn default() -> DynamicResourcesView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, DynamicResources>> for DynamicResourcesView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, DynamicResources>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> DynamicResourcesView<'msg> {

  pub fn to_owned(&self) -> DynamicResources {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // lds_config: optional message envoy.config.core.v3.ConfigSource
  pub fn has_lds_config(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn lds_config_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::config_source::ConfigSourceView<'msg>> {
    self.has_lds_config().then(|| self.lds_config())
  }
  pub fn lds_config(self) -> crate::xds::generated::envoy::config::core::v3::config_source::ConfigSourceView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::config_source::ConfigSourceView::default())
  }

  // lds_resources_locator: optional string
  pub fn lds_resources_locator(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        3, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // cds_config: optional message envoy.config.core.v3.ConfigSource
  pub fn has_cds_config(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn cds_config_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::config_source::ConfigSourceView<'msg>> {
    self.has_cds_config().then(|| self.cds_config())
  }
  pub fn cds_config(self) -> crate::xds::generated::envoy::config::core::v3::config_source::ConfigSourceView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::config_source::ConfigSourceView::default())
  }

  // cds_resources_locator: optional string
  pub fn cds_resources_locator(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        4, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // ads_config: optional message envoy.config.core.v3.ApiConfigSource
  pub fn has_ads_config(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn ads_config_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::config_source::ApiConfigSourceView<'msg>> {
    self.has_ads_config().then(|| self.ads_config())
  }
  pub fn ads_config(self) -> crate::xds::generated::envoy::config::core::v3::config_source::ApiConfigSourceView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::config_source::ApiConfigSourceView::default())
  }

}

// SAFETY:
// - `DynamicResourcesView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for DynamicResourcesView<'_> {}

// SAFETY:
// - `DynamicResourcesView` is `Send` because while its alive a `DynamicResourcesMut` cannot.
// - `DynamicResourcesView` does not use thread-local data.
unsafe impl ::std::marker::Send for DynamicResourcesView<'_> {}

impl<'msg> ::protobuf::AsView for DynamicResourcesView<'msg> {
  type Proxied = DynamicResources;
  fn as_view(&self) -> ::protobuf::View<'msg, DynamicResources> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for DynamicResourcesView<'msg> {
  fn into_view<'shorter>(self) -> DynamicResourcesView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<DynamicResources> for DynamicResourcesView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> DynamicResources {
    let mut dst = DynamicResources::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<DynamicResources> for DynamicResourcesMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> DynamicResources {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for DynamicResources {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for DynamicResourcesView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for DynamicResourcesMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct DynamicResourcesMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, DynamicResources>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for DynamicResourcesMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for DynamicResourcesMut<'msg> {
  type Message = DynamicResources;
}

impl ::std::fmt::Debug for DynamicResourcesMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, DynamicResources>> for DynamicResourcesMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, DynamicResources>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> DynamicResourcesMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, DynamicResources> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> DynamicResources {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // lds_config: optional message envoy.config.core.v3.ConfigSource
  pub fn has_lds_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_lds_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn lds_config_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::config_source::ConfigSourceView<'_>> {
    self.has_lds_config().then(|| self.lds_config())
  }
  pub fn lds_config(&self) -> crate::xds::generated::envoy::config::core::v3::config_source::ConfigSourceView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::config_source::ConfigSourceView::default())
  }
  pub fn lds_config_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::config_source::ConfigSourceMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         0, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_lds_config(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::config_source::ConfigSource>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // lds_resources_locator: optional string
  pub fn lds_resources_locator(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        3, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_lds_resources_locator(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val);
    }
  }

  // cds_config: optional message envoy.config.core.v3.ConfigSource
  pub fn has_cds_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_cds_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn cds_config_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::config_source::ConfigSourceView<'_>> {
    self.has_cds_config().then(|| self.cds_config())
  }
  pub fn cds_config(&self) -> crate::xds::generated::envoy::config::core::v3::config_source::ConfigSourceView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::config_source::ConfigSourceView::default())
  }
  pub fn cds_config_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::config_source::ConfigSourceMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         1, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_cds_config(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::config_source::ConfigSource>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // cds_resources_locator: optional string
  pub fn cds_resources_locator(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        4, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_cds_resources_locator(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val);
    }
  }

  // ads_config: optional message envoy.config.core.v3.ApiConfigSource
  pub fn has_ads_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_ads_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn ads_config_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::config_source::ApiConfigSourceView<'_>> {
    self.has_ads_config().then(|| self.ads_config())
  }
  pub fn ads_config(&self) -> crate::xds::generated::envoy::config::core::v3::config_source::ApiConfigSourceView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::config_source::ApiConfigSourceView::default())
  }
  pub fn ads_config_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::config_source::ApiConfigSourceMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         2, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_ads_config(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::config_source::ApiConfigSource>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

}

// SAFETY:
// - `DynamicResourcesMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for DynamicResourcesMut<'_> {}

// SAFETY:
// - `DynamicResourcesMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for DynamicResourcesMut<'_> {}

impl<'msg> ::protobuf::AsView for DynamicResourcesMut<'msg> {
  type Proxied = DynamicResources;
  fn as_view(&self) -> ::protobuf::View<'_, DynamicResources> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for DynamicResourcesMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, DynamicResources>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for DynamicResourcesMut<'msg> {
  type MutProxied = DynamicResources;
  fn as_mut(&mut self) -> DynamicResourcesMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for DynamicResourcesMut<'msg> {
  fn into_mut<'shorter>(self) -> DynamicResourcesMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl DynamicResources {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, DynamicResources> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> DynamicResourcesView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> DynamicResourcesMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // lds_config: optional message envoy.config.core.v3.ConfigSource
  pub fn has_lds_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_lds_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn lds_config_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::config_source::ConfigSourceView<'_>> {
    self.has_lds_config().then(|| self.lds_config())
  }
  pub fn lds_config(&self) -> crate::xds::generated::envoy::config::core::v3::config_source::ConfigSourceView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::config_source::ConfigSourceView::default())
  }
  pub fn lds_config_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::config_source::ConfigSourceMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         0, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_lds_config(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::config_source::ConfigSource>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // lds_resources_locator: optional string
  pub fn lds_resources_locator(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        3, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_lds_resources_locator(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val);
    }
  }

  // cds_config: optional message envoy.config.core.v3.ConfigSource
  pub fn has_cds_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_cds_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn cds_config_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::config_source::ConfigSourceView<'_>> {
    self.has_cds_config().then(|| self.cds_config())
  }
  pub fn cds_config(&self) -> crate::xds::generated::envoy::config::core::v3::config_source::ConfigSourceView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::config_source::ConfigSourceView::default())
  }
  pub fn cds_config_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::config_source::ConfigSourceMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         1, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_cds_config(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::config_source::ConfigSource>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // cds_resources_locator: optional string
  pub fn cds_resources_locator(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        4, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_cds_resources_locator(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val);
    }
  }

  // ads_config: optional message envoy.config.core.v3.ApiConfigSource
  pub fn has_ads_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_ads_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn ads_config_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::config_source::ApiConfigSourceView<'_>> {
    self.has_ads_config().then(|| self.ads_config())
  }
  pub fn ads_config(&self) -> crate::xds::generated::envoy::config::core::v3::config_source::ApiConfigSourceView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::config_source::ApiConfigSourceView::default())
  }
  pub fn ads_config_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::config_source::ApiConfigSourceMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         2, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_ads_config(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::config_source::ApiConfigSource>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

}  // impl DynamicResources

impl ::std::ops::Drop for DynamicResources {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for DynamicResources {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for DynamicResources {
  type Proxied = Self;
  fn as_view(&self) -> DynamicResourcesView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for DynamicResources {
  type MutProxied = Self;
  fn as_mut(&mut self) -> DynamicResourcesMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for DynamicResources {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::bootstrap::envoy__config__bootstrap__v3__Bootstrap__DynamicResources_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$333a1X1X");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::bootstrap::envoy__config__bootstrap__v3__Bootstrap__DynamicResources_msg_init.0, &[<crate::xds::generated::envoy::config::core::v3::config_source::ConfigSource as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::core::v3::config_source::ConfigSource as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::core::v3::config_source::ApiConfigSource as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::bootstrap::envoy__config__bootstrap__v3__Bootstrap__DynamicResources_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for DynamicResources {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for DynamicResources {
  type Msg = DynamicResources;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DynamicResources> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DynamicResources {
  type Msg = DynamicResources;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DynamicResources> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for DynamicResourcesMut<'_> {
  type Msg = DynamicResources;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DynamicResources> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DynamicResourcesMut<'_> {
  type Msg = DynamicResources;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DynamicResources> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DynamicResourcesView<'_> {
  type Msg = DynamicResources;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DynamicResources> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for DynamicResourcesMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__bootstrap__v3__Bootstrap__ApplicationLogConfig_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct ApplicationLogConfig {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<ApplicationLogConfig>
}

impl ::protobuf::Message for ApplicationLogConfig {
  type MessageView<'msg> = ApplicationLogConfigView<'msg>;
  type MessageMut<'msg> = ApplicationLogConfigMut<'msg>;
}

impl ::std::default::Default for ApplicationLogConfig {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for ApplicationLogConfig {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `ApplicationLogConfig` is `Sync` because it does not implement interior mutability.
//    Neither does `ApplicationLogConfigMut`.
unsafe impl ::std::marker::Sync for ApplicationLogConfig {}

// SAFETY:
// - `ApplicationLogConfig` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for ApplicationLogConfig {}

impl ::protobuf::Proxied for ApplicationLogConfig {
  type View<'msg> = ApplicationLogConfigView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for ApplicationLogConfig {}

impl ::protobuf::MutProxied for ApplicationLogConfig {
  type Mut<'msg> = ApplicationLogConfigMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ApplicationLogConfigView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ApplicationLogConfig>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ApplicationLogConfigView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ApplicationLogConfigView<'msg> {
  type Message = ApplicationLogConfig;
}

impl ::std::fmt::Debug for ApplicationLogConfigView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ApplicationLogConfigView<'_> {
  fn default() -> ApplicationLogConfigView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, ApplicationLogConfig>> for ApplicationLogConfigView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ApplicationLogConfig>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ApplicationLogConfigView<'msg> {

  pub fn to_owned(&self) -> ApplicationLogConfig {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // log_format: optional message envoy.config.bootstrap.v3.Bootstrap.ApplicationLogConfig.LogFormat
  pub fn has_log_format(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn log_format_opt(self) -> ::std::option::Option<super::super::bootstrap::application_log_config::LogFormatView<'msg>> {
    self.has_log_format().then(|| self.log_format())
  }
  pub fn log_format(self) -> super::super::bootstrap::application_log_config::LogFormatView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::bootstrap::application_log_config::LogFormatView::default())
  }

}

// SAFETY:
// - `ApplicationLogConfigView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for ApplicationLogConfigView<'_> {}

// SAFETY:
// - `ApplicationLogConfigView` is `Send` because while its alive a `ApplicationLogConfigMut` cannot.
// - `ApplicationLogConfigView` does not use thread-local data.
unsafe impl ::std::marker::Send for ApplicationLogConfigView<'_> {}

impl<'msg> ::protobuf::AsView for ApplicationLogConfigView<'msg> {
  type Proxied = ApplicationLogConfig;
  fn as_view(&self) -> ::protobuf::View<'msg, ApplicationLogConfig> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ApplicationLogConfigView<'msg> {
  fn into_view<'shorter>(self) -> ApplicationLogConfigView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<ApplicationLogConfig> for ApplicationLogConfigView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ApplicationLogConfig {
    let mut dst = ApplicationLogConfig::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<ApplicationLogConfig> for ApplicationLogConfigMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ApplicationLogConfig {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for ApplicationLogConfig {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ApplicationLogConfigView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ApplicationLogConfigMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ApplicationLogConfigMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ApplicationLogConfig>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ApplicationLogConfigMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ApplicationLogConfigMut<'msg> {
  type Message = ApplicationLogConfig;
}

impl ::std::fmt::Debug for ApplicationLogConfigMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, ApplicationLogConfig>> for ApplicationLogConfigMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ApplicationLogConfig>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ApplicationLogConfigMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, ApplicationLogConfig> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> ApplicationLogConfig {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // log_format: optional message envoy.config.bootstrap.v3.Bootstrap.ApplicationLogConfig.LogFormat
  pub fn has_log_format(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_log_format(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn log_format_opt(&self) -> ::std::option::Option<super::super::bootstrap::application_log_config::LogFormatView<'_>> {
    self.has_log_format().then(|| self.log_format())
  }
  pub fn log_format(&self) -> super::super::bootstrap::application_log_config::LogFormatView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::bootstrap::application_log_config::LogFormatView::default())
  }
  pub fn log_format_mut(&mut self) -> super::super::bootstrap::application_log_config::LogFormatMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         0, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_log_format(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::bootstrap::application_log_config::LogFormat>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

}

// SAFETY:
// - `ApplicationLogConfigMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for ApplicationLogConfigMut<'_> {}

// SAFETY:
// - `ApplicationLogConfigMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for ApplicationLogConfigMut<'_> {}

impl<'msg> ::protobuf::AsView for ApplicationLogConfigMut<'msg> {
  type Proxied = ApplicationLogConfig;
  fn as_view(&self) -> ::protobuf::View<'_, ApplicationLogConfig> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ApplicationLogConfigMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, ApplicationLogConfig>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for ApplicationLogConfigMut<'msg> {
  type MutProxied = ApplicationLogConfig;
  fn as_mut(&mut self) -> ApplicationLogConfigMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ApplicationLogConfigMut<'msg> {
  fn into_mut<'shorter>(self) -> ApplicationLogConfigMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl ApplicationLogConfig {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, ApplicationLogConfig> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ApplicationLogConfigView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ApplicationLogConfigMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // log_format: optional message envoy.config.bootstrap.v3.Bootstrap.ApplicationLogConfig.LogFormat
  pub fn has_log_format(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_log_format(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn log_format_opt(&self) -> ::std::option::Option<super::super::bootstrap::application_log_config::LogFormatView<'_>> {
    self.has_log_format().then(|| self.log_format())
  }
  pub fn log_format(&self) -> super::super::bootstrap::application_log_config::LogFormatView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::bootstrap::application_log_config::LogFormatView::default())
  }
  pub fn log_format_mut(&mut self) -> super::super::bootstrap::application_log_config::LogFormatMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         0, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_log_format(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::bootstrap::application_log_config::LogFormat>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

}  // impl ApplicationLogConfig

impl ::std::ops::Drop for ApplicationLogConfig {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for ApplicationLogConfig {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for ApplicationLogConfig {
  type Proxied = Self;
  fn as_view(&self) -> ApplicationLogConfigView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for ApplicationLogConfig {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ApplicationLogConfigMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for ApplicationLogConfig {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::bootstrap::envoy__config__bootstrap__v3__Bootstrap__ApplicationLogConfig_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$3");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::bootstrap::envoy__config__bootstrap__v3__Bootstrap__ApplicationLogConfig_msg_init.0, &[<super::super::bootstrap::application_log_config::LogFormat as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::bootstrap::envoy__config__bootstrap__v3__Bootstrap__ApplicationLogConfig_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ApplicationLogConfig {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ApplicationLogConfig {
  type Msg = ApplicationLogConfig;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ApplicationLogConfig> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ApplicationLogConfig {
  type Msg = ApplicationLogConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ApplicationLogConfig> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ApplicationLogConfigMut<'_> {
  type Msg = ApplicationLogConfig;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ApplicationLogConfig> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ApplicationLogConfigMut<'_> {
  type Msg = ApplicationLogConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ApplicationLogConfig> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ApplicationLogConfigView<'_> {
  type Msg = ApplicationLogConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ApplicationLogConfig> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ApplicationLogConfigMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod application_log_config {// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__bootstrap__v3__Bootstrap__ApplicationLogConfig__LogFormat_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct LogFormat {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<LogFormat>
}

impl ::protobuf::Message for LogFormat {
  type MessageView<'msg> = LogFormatView<'msg>;
  type MessageMut<'msg> = LogFormatMut<'msg>;
}

impl ::std::default::Default for LogFormat {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for LogFormat {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `LogFormat` is `Sync` because it does not implement interior mutability.
//    Neither does `LogFormatMut`.
unsafe impl ::std::marker::Sync for LogFormat {}

// SAFETY:
// - `LogFormat` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for LogFormat {}

impl ::protobuf::Proxied for LogFormat {
  type View<'msg> = LogFormatView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for LogFormat {}

impl ::protobuf::MutProxied for LogFormat {
  type Mut<'msg> = LogFormatMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct LogFormatView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, LogFormat>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for LogFormatView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for LogFormatView<'msg> {
  type Message = LogFormat;
}

impl ::std::fmt::Debug for LogFormatView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for LogFormatView<'_> {
  fn default() -> LogFormatView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, LogFormat>> for LogFormatView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, LogFormat>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> LogFormatView<'msg> {

  pub fn to_owned(&self) -> LogFormat {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // json_format: optional message google.protobuf.Struct
  pub fn has_json_format(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn json_format_opt(self) -> ::std::option::Option<::protobuf_well_known_types::StructView<'msg>> {
    self.has_json_format().then(|| self.json_format())
  }
  pub fn json_format(self) -> ::protobuf_well_known_types::StructView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::StructView::default())
  }

  // text_format: optional string
  pub fn has_text_format(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn text_format_opt(self) -> ::std::option::Option<&'msg ::protobuf::ProtoStr> {
    self.has_text_format().then(|| self.text_format())
  }
  pub fn text_format(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  pub fn log_format(self) -> super::super::super::bootstrap::application_log_config::log_format::LogFormatOneof<'msg> {
    match self.log_format_case() {
      super::super::super::bootstrap::application_log_config::log_format::LogFormatCase::JsonFormat =>
          super::super::super::bootstrap::application_log_config::log_format::LogFormatOneof::JsonFormat(self.json_format()),
      super::super::super::bootstrap::application_log_config::log_format::LogFormatCase::TextFormat =>
          super::super::super::bootstrap::application_log_config::log_format::LogFormatOneof::TextFormat(self.text_format()),
      _ => super::super::super::bootstrap::application_log_config::log_format::LogFormatOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn log_format_case(self) -> super::super::super::bootstrap::application_log_config::log_format::LogFormatCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::super::super::bootstrap::application_log_config::log_format::LogFormatCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `LogFormatView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for LogFormatView<'_> {}

// SAFETY:
// - `LogFormatView` is `Send` because while its alive a `LogFormatMut` cannot.
// - `LogFormatView` does not use thread-local data.
unsafe impl ::std::marker::Send for LogFormatView<'_> {}

impl<'msg> ::protobuf::AsView for LogFormatView<'msg> {
  type Proxied = LogFormat;
  fn as_view(&self) -> ::protobuf::View<'msg, LogFormat> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for LogFormatView<'msg> {
  fn into_view<'shorter>(self) -> LogFormatView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<LogFormat> for LogFormatView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> LogFormat {
    let mut dst = LogFormat::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<LogFormat> for LogFormatMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> LogFormat {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for LogFormat {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for LogFormatView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for LogFormatMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct LogFormatMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, LogFormat>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for LogFormatMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for LogFormatMut<'msg> {
  type Message = LogFormat;
}

impl ::std::fmt::Debug for LogFormatMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, LogFormat>> for LogFormatMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, LogFormat>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> LogFormatMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, LogFormat> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> LogFormat {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // json_format: optional message google.protobuf.Struct
  pub fn has_json_format(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_json_format(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn json_format_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::StructView<'_>> {
    self.has_json_format().then(|| self.json_format())
  }
  pub fn json_format(&self) -> ::protobuf_well_known_types::StructView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::StructView::default())
  }
  pub fn json_format_mut(&mut self) -> ::protobuf_well_known_types::StructMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         0, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_json_format(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Struct>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // text_format: optional string
  pub fn has_text_format(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_text_format(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn text_format_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_text_format().then(|| self.text_format())
  }
  pub fn text_format(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_text_format(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  pub fn log_format(&self) -> super::super::super::bootstrap::application_log_config::log_format::LogFormatOneof<'_> {
    match &self.log_format_case() {
      super::super::super::bootstrap::application_log_config::log_format::LogFormatCase::JsonFormat =>
          super::super::super::bootstrap::application_log_config::log_format::LogFormatOneof::JsonFormat(self.json_format()),
      super::super::super::bootstrap::application_log_config::log_format::LogFormatCase::TextFormat =>
          super::super::super::bootstrap::application_log_config::log_format::LogFormatOneof::TextFormat(self.text_format()),
      _ => super::super::super::bootstrap::application_log_config::log_format::LogFormatOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn log_format_case(&self) -> super::super::super::bootstrap::application_log_config::log_format::LogFormatCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::super::super::bootstrap::application_log_config::log_format::LogFormatCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `LogFormatMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for LogFormatMut<'_> {}

// SAFETY:
// - `LogFormatMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for LogFormatMut<'_> {}

impl<'msg> ::protobuf::AsView for LogFormatMut<'msg> {
  type Proxied = LogFormat;
  fn as_view(&self) -> ::protobuf::View<'_, LogFormat> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for LogFormatMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, LogFormat>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for LogFormatMut<'msg> {
  type MutProxied = LogFormat;
  fn as_mut(&mut self) -> LogFormatMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for LogFormatMut<'msg> {
  fn into_mut<'shorter>(self) -> LogFormatMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl LogFormat {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, LogFormat> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> LogFormatView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> LogFormatMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // json_format: optional message google.protobuf.Struct
  pub fn has_json_format(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_json_format(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn json_format_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::StructView<'_>> {
    self.has_json_format().then(|| self.json_format())
  }
  pub fn json_format(&self) -> ::protobuf_well_known_types::StructView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::StructView::default())
  }
  pub fn json_format_mut(&mut self) -> ::protobuf_well_known_types::StructMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         0, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_json_format(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Struct>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // text_format: optional string
  pub fn has_text_format(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_text_format(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn text_format_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_text_format().then(|| self.text_format())
  }
  pub fn text_format(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_text_format(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  pub fn log_format(&self) -> super::super::super::bootstrap::application_log_config::log_format::LogFormatOneof<'_> {
    match &self.log_format_case() {
      super::super::super::bootstrap::application_log_config::log_format::LogFormatCase::JsonFormat =>
          super::super::super::bootstrap::application_log_config::log_format::LogFormatOneof::JsonFormat(self.json_format()),
      super::super::super::bootstrap::application_log_config::log_format::LogFormatCase::TextFormat =>
          super::super::super::bootstrap::application_log_config::log_format::LogFormatOneof::TextFormat(self.text_format()),
      _ => super::super::super::bootstrap::application_log_config::log_format::LogFormatOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn log_format_case(&self) -> super::super::super::bootstrap::application_log_config::log_format::LogFormatCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(0);
      super::super::super::bootstrap::application_log_config::log_format::LogFormatCase::try_from(field_num).unwrap_unchecked()
    }
  }
}  // impl LogFormat

impl ::std::ops::Drop for LogFormat {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for LogFormat {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for LogFormat {
  type Proxied = Self;
  fn as_view(&self) -> LogFormatView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for LogFormat {
  type MutProxied = Self;
  fn as_mut(&mut self) -> LogFormatMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for LogFormat {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::super::bootstrap::application_log_config::envoy__config__bootstrap__v3__Bootstrap__ApplicationLogConfig__LogFormat_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$31T^!|#");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::super::bootstrap::application_log_config::envoy__config__bootstrap__v3__Bootstrap__ApplicationLogConfig__LogFormat_msg_init.0, &[<::protobuf_well_known_types::Struct as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::super::bootstrap::application_log_config::envoy__config__bootstrap__v3__Bootstrap__ApplicationLogConfig__LogFormat_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for LogFormat {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for LogFormat {
  type Msg = LogFormat;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<LogFormat> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for LogFormat {
  type Msg = LogFormat;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<LogFormat> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for LogFormatMut<'_> {
  type Msg = LogFormat;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<LogFormat> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for LogFormatMut<'_> {
  type Msg = LogFormat;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<LogFormat> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for LogFormatView<'_> {
  type Msg = LogFormat;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<LogFormat> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for LogFormatMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod log_format {

#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
#[repr(u32)]
pub enum LogFormatOneof<'msg> {
  JsonFormat(::protobuf::View<'msg, ::protobuf_well_known_types::Struct>) = 1,
  TextFormat(&'msg ::protobuf::ProtoStr) = 2,

  not_set(std::marker::PhantomData<&'msg ()>) = 0
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[allow(dead_code)]
pub enum LogFormatCase {
  JsonFormat = 1,
  TextFormat = 2,

  not_set = 0
}

impl LogFormatCase {
  #[allow(dead_code)]
  pub(crate) fn try_from(v: u32) -> ::std::option::Option<LogFormatCase> {
    match v {
      0 => Some(LogFormatCase::not_set),
      1 => Some(LogFormatCase::JsonFormat),
      2 => Some(LogFormatCase::TextFormat),
      _ => None
    }
  }
}
}  // pub mod log_format


}  // pub mod application_log_config

// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__bootstrap__v3__Bootstrap__DeferredStatOptions_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct DeferredStatOptions {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<DeferredStatOptions>
}

impl ::protobuf::Message for DeferredStatOptions {
  type MessageView<'msg> = DeferredStatOptionsView<'msg>;
  type MessageMut<'msg> = DeferredStatOptionsMut<'msg>;
}

impl ::std::default::Default for DeferredStatOptions {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for DeferredStatOptions {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `DeferredStatOptions` is `Sync` because it does not implement interior mutability.
//    Neither does `DeferredStatOptionsMut`.
unsafe impl ::std::marker::Sync for DeferredStatOptions {}

// SAFETY:
// - `DeferredStatOptions` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for DeferredStatOptions {}

impl ::protobuf::Proxied for DeferredStatOptions {
  type View<'msg> = DeferredStatOptionsView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for DeferredStatOptions {}

impl ::protobuf::MutProxied for DeferredStatOptions {
  type Mut<'msg> = DeferredStatOptionsMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct DeferredStatOptionsView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, DeferredStatOptions>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for DeferredStatOptionsView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for DeferredStatOptionsView<'msg> {
  type Message = DeferredStatOptions;
}

impl ::std::fmt::Debug for DeferredStatOptionsView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for DeferredStatOptionsView<'_> {
  fn default() -> DeferredStatOptionsView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, DeferredStatOptions>> for DeferredStatOptionsView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, DeferredStatOptions>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> DeferredStatOptionsView<'msg> {

  pub fn to_owned(&self) -> DeferredStatOptions {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // enable_deferred_creation_stats: optional bool
  pub fn enable_deferred_creation_stats(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        0, (false).into()
      ).try_into().unwrap()
    }
  }

}

// SAFETY:
// - `DeferredStatOptionsView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for DeferredStatOptionsView<'_> {}

// SAFETY:
// - `DeferredStatOptionsView` is `Send` because while its alive a `DeferredStatOptionsMut` cannot.
// - `DeferredStatOptionsView` does not use thread-local data.
unsafe impl ::std::marker::Send for DeferredStatOptionsView<'_> {}

impl<'msg> ::protobuf::AsView for DeferredStatOptionsView<'msg> {
  type Proxied = DeferredStatOptions;
  fn as_view(&self) -> ::protobuf::View<'msg, DeferredStatOptions> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for DeferredStatOptionsView<'msg> {
  fn into_view<'shorter>(self) -> DeferredStatOptionsView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<DeferredStatOptions> for DeferredStatOptionsView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> DeferredStatOptions {
    let mut dst = DeferredStatOptions::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<DeferredStatOptions> for DeferredStatOptionsMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> DeferredStatOptions {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for DeferredStatOptions {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for DeferredStatOptionsView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for DeferredStatOptionsMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct DeferredStatOptionsMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, DeferredStatOptions>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for DeferredStatOptionsMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for DeferredStatOptionsMut<'msg> {
  type Message = DeferredStatOptions;
}

impl ::std::fmt::Debug for DeferredStatOptionsMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, DeferredStatOptions>> for DeferredStatOptionsMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, DeferredStatOptions>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> DeferredStatOptionsMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, DeferredStatOptions> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> DeferredStatOptions {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // enable_deferred_creation_stats: optional bool
  pub fn enable_deferred_creation_stats(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        0, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_enable_deferred_creation_stats(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        0, val.into()
      )
    }
  }

}

// SAFETY:
// - `DeferredStatOptionsMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for DeferredStatOptionsMut<'_> {}

// SAFETY:
// - `DeferredStatOptionsMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for DeferredStatOptionsMut<'_> {}

impl<'msg> ::protobuf::AsView for DeferredStatOptionsMut<'msg> {
  type Proxied = DeferredStatOptions;
  fn as_view(&self) -> ::protobuf::View<'_, DeferredStatOptions> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for DeferredStatOptionsMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, DeferredStatOptions>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for DeferredStatOptionsMut<'msg> {
  type MutProxied = DeferredStatOptions;
  fn as_mut(&mut self) -> DeferredStatOptionsMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for DeferredStatOptionsMut<'msg> {
  fn into_mut<'shorter>(self) -> DeferredStatOptionsMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl DeferredStatOptions {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, DeferredStatOptions> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> DeferredStatOptionsView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> DeferredStatOptionsMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // enable_deferred_creation_stats: optional bool
  pub fn enable_deferred_creation_stats(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        0, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_enable_deferred_creation_stats(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        0, val.into()
      )
    }
  }

}  // impl DeferredStatOptions

impl ::std::ops::Drop for DeferredStatOptions {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for DeferredStatOptions {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for DeferredStatOptions {
  type Proxied = Self;
  fn as_view(&self) -> DeferredStatOptionsView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for DeferredStatOptions {
  type MutProxied = Self;
  fn as_mut(&mut self) -> DeferredStatOptionsMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for DeferredStatOptions {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::bootstrap::envoy__config__bootstrap__v3__Bootstrap__DeferredStatOptions_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$/P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::bootstrap::envoy__config__bootstrap__v3__Bootstrap__DeferredStatOptions_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::bootstrap::envoy__config__bootstrap__v3__Bootstrap__DeferredStatOptions_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for DeferredStatOptions {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for DeferredStatOptions {
  type Msg = DeferredStatOptions;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DeferredStatOptions> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DeferredStatOptions {
  type Msg = DeferredStatOptions;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DeferredStatOptions> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for DeferredStatOptionsMut<'_> {
  type Msg = DeferredStatOptions;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DeferredStatOptions> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DeferredStatOptionsMut<'_> {
  type Msg = DeferredStatOptions;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DeferredStatOptions> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DeferredStatOptionsView<'_> {
  type Msg = DeferredStatOptions;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DeferredStatOptions> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for DeferredStatOptionsMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__bootstrap__v3__Bootstrap__GrpcAsyncClientManagerConfig_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct GrpcAsyncClientManagerConfig {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<GrpcAsyncClientManagerConfig>
}

impl ::protobuf::Message for GrpcAsyncClientManagerConfig {
  type MessageView<'msg> = GrpcAsyncClientManagerConfigView<'msg>;
  type MessageMut<'msg> = GrpcAsyncClientManagerConfigMut<'msg>;
}

impl ::std::default::Default for GrpcAsyncClientManagerConfig {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for GrpcAsyncClientManagerConfig {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `GrpcAsyncClientManagerConfig` is `Sync` because it does not implement interior mutability.
//    Neither does `GrpcAsyncClientManagerConfigMut`.
unsafe impl ::std::marker::Sync for GrpcAsyncClientManagerConfig {}

// SAFETY:
// - `GrpcAsyncClientManagerConfig` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for GrpcAsyncClientManagerConfig {}

impl ::protobuf::Proxied for GrpcAsyncClientManagerConfig {
  type View<'msg> = GrpcAsyncClientManagerConfigView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for GrpcAsyncClientManagerConfig {}

impl ::protobuf::MutProxied for GrpcAsyncClientManagerConfig {
  type Mut<'msg> = GrpcAsyncClientManagerConfigMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct GrpcAsyncClientManagerConfigView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, GrpcAsyncClientManagerConfig>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for GrpcAsyncClientManagerConfigView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for GrpcAsyncClientManagerConfigView<'msg> {
  type Message = GrpcAsyncClientManagerConfig;
}

impl ::std::fmt::Debug for GrpcAsyncClientManagerConfigView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for GrpcAsyncClientManagerConfigView<'_> {
  fn default() -> GrpcAsyncClientManagerConfigView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, GrpcAsyncClientManagerConfig>> for GrpcAsyncClientManagerConfigView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, GrpcAsyncClientManagerConfig>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> GrpcAsyncClientManagerConfigView<'msg> {

  pub fn to_owned(&self) -> GrpcAsyncClientManagerConfig {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // max_cached_entry_idle_duration: optional message google.protobuf.Duration
  pub fn has_max_cached_entry_idle_duration(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn max_cached_entry_idle_duration_opt(self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'msg>> {
    self.has_max_cached_entry_idle_duration().then(|| self.max_cached_entry_idle_duration())
  }
  pub fn max_cached_entry_idle_duration(self) -> ::protobuf_well_known_types::DurationView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }

}

// SAFETY:
// - `GrpcAsyncClientManagerConfigView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for GrpcAsyncClientManagerConfigView<'_> {}

// SAFETY:
// - `GrpcAsyncClientManagerConfigView` is `Send` because while its alive a `GrpcAsyncClientManagerConfigMut` cannot.
// - `GrpcAsyncClientManagerConfigView` does not use thread-local data.
unsafe impl ::std::marker::Send for GrpcAsyncClientManagerConfigView<'_> {}

impl<'msg> ::protobuf::AsView for GrpcAsyncClientManagerConfigView<'msg> {
  type Proxied = GrpcAsyncClientManagerConfig;
  fn as_view(&self) -> ::protobuf::View<'msg, GrpcAsyncClientManagerConfig> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for GrpcAsyncClientManagerConfigView<'msg> {
  fn into_view<'shorter>(self) -> GrpcAsyncClientManagerConfigView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<GrpcAsyncClientManagerConfig> for GrpcAsyncClientManagerConfigView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> GrpcAsyncClientManagerConfig {
    let mut dst = GrpcAsyncClientManagerConfig::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<GrpcAsyncClientManagerConfig> for GrpcAsyncClientManagerConfigMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> GrpcAsyncClientManagerConfig {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for GrpcAsyncClientManagerConfig {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for GrpcAsyncClientManagerConfigView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for GrpcAsyncClientManagerConfigMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct GrpcAsyncClientManagerConfigMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, GrpcAsyncClientManagerConfig>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for GrpcAsyncClientManagerConfigMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for GrpcAsyncClientManagerConfigMut<'msg> {
  type Message = GrpcAsyncClientManagerConfig;
}

impl ::std::fmt::Debug for GrpcAsyncClientManagerConfigMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, GrpcAsyncClientManagerConfig>> for GrpcAsyncClientManagerConfigMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, GrpcAsyncClientManagerConfig>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> GrpcAsyncClientManagerConfigMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, GrpcAsyncClientManagerConfig> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> GrpcAsyncClientManagerConfig {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // max_cached_entry_idle_duration: optional message google.protobuf.Duration
  pub fn has_max_cached_entry_idle_duration(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_max_cached_entry_idle_duration(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn max_cached_entry_idle_duration_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_max_cached_entry_idle_duration().then(|| self.max_cached_entry_idle_duration())
  }
  pub fn max_cached_entry_idle_duration(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn max_cached_entry_idle_duration_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         0, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_max_cached_entry_idle_duration(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

}

// SAFETY:
// - `GrpcAsyncClientManagerConfigMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for GrpcAsyncClientManagerConfigMut<'_> {}

// SAFETY:
// - `GrpcAsyncClientManagerConfigMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for GrpcAsyncClientManagerConfigMut<'_> {}

impl<'msg> ::protobuf::AsView for GrpcAsyncClientManagerConfigMut<'msg> {
  type Proxied = GrpcAsyncClientManagerConfig;
  fn as_view(&self) -> ::protobuf::View<'_, GrpcAsyncClientManagerConfig> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for GrpcAsyncClientManagerConfigMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, GrpcAsyncClientManagerConfig>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for GrpcAsyncClientManagerConfigMut<'msg> {
  type MutProxied = GrpcAsyncClientManagerConfig;
  fn as_mut(&mut self) -> GrpcAsyncClientManagerConfigMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for GrpcAsyncClientManagerConfigMut<'msg> {
  fn into_mut<'shorter>(self) -> GrpcAsyncClientManagerConfigMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl GrpcAsyncClientManagerConfig {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, GrpcAsyncClientManagerConfig> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> GrpcAsyncClientManagerConfigView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> GrpcAsyncClientManagerConfigMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // max_cached_entry_idle_duration: optional message google.protobuf.Duration
  pub fn has_max_cached_entry_idle_duration(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_max_cached_entry_idle_duration(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn max_cached_entry_idle_duration_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_max_cached_entry_idle_duration().then(|| self.max_cached_entry_idle_duration())
  }
  pub fn max_cached_entry_idle_duration(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn max_cached_entry_idle_duration_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         0, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_max_cached_entry_idle_duration(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

}  // impl GrpcAsyncClientManagerConfig

impl ::std::ops::Drop for GrpcAsyncClientManagerConfig {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for GrpcAsyncClientManagerConfig {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for GrpcAsyncClientManagerConfig {
  type Proxied = Self;
  fn as_view(&self) -> GrpcAsyncClientManagerConfigView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for GrpcAsyncClientManagerConfig {
  type MutProxied = Self;
  fn as_mut(&mut self) -> GrpcAsyncClientManagerConfigMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for GrpcAsyncClientManagerConfig {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::bootstrap::envoy__config__bootstrap__v3__Bootstrap__GrpcAsyncClientManagerConfig_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$3");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::bootstrap::envoy__config__bootstrap__v3__Bootstrap__GrpcAsyncClientManagerConfig_msg_init.0, &[<::protobuf_well_known_types::Duration as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::bootstrap::envoy__config__bootstrap__v3__Bootstrap__GrpcAsyncClientManagerConfig_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for GrpcAsyncClientManagerConfig {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for GrpcAsyncClientManagerConfig {
  type Msg = GrpcAsyncClientManagerConfig;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GrpcAsyncClientManagerConfig> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for GrpcAsyncClientManagerConfig {
  type Msg = GrpcAsyncClientManagerConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GrpcAsyncClientManagerConfig> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for GrpcAsyncClientManagerConfigMut<'_> {
  type Msg = GrpcAsyncClientManagerConfig;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GrpcAsyncClientManagerConfig> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for GrpcAsyncClientManagerConfigMut<'_> {
  type Msg = GrpcAsyncClientManagerConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GrpcAsyncClientManagerConfig> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for GrpcAsyncClientManagerConfigView<'_> {
  type Msg = GrpcAsyncClientManagerConfig;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<GrpcAsyncClientManagerConfig> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for GrpcAsyncClientManagerConfigMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__bootstrap__v3__Bootstrap__CertificateProviderInstancesEntry_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(dead_code)]
pub(super) struct CertificateProviderInstancesEntry;

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for CertificateProviderInstancesEntry {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::bootstrap::envoy__config__bootstrap__v3__Bootstrap__CertificateProviderInstancesEntry_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("%1X3");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::bootstrap::envoy__config__bootstrap__v3__Bootstrap__CertificateProviderInstancesEntry_msg_init.0, &[<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::bootstrap::envoy__config__bootstrap__v3__Bootstrap__CertificateProviderInstancesEntry_msg_init.0)
      }).0
    }
  }
}

#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
#[repr(u32)]
pub enum StatsFlushOneof<'msg> {
  StatsFlushOnAdmin(bool) = 29,

  not_set(std::marker::PhantomData<&'msg ()>) = 0
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[allow(dead_code)]
pub enum StatsFlushCase {
  StatsFlushOnAdmin = 29,

  not_set = 0
}

impl StatsFlushCase {
  #[allow(dead_code)]
  pub(crate) fn try_from(v: u32) -> ::std::option::Option<StatsFlushCase> {
    match v {
      0 => Some(StatsFlushCase::not_set),
      29 => Some(StatsFlushCase::StatsFlushOnAdmin),
      _ => None
    }
  }
}
#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
#[repr(u32)]
pub enum StatsEvictionOneof<'msg> {
  StatsEvictionInterval(::protobuf::View<'msg, ::protobuf_well_known_types::Duration>) = 42,

  not_set(std::marker::PhantomData<&'msg ()>) = 0
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[allow(dead_code)]
pub enum StatsEvictionCase {
  StatsEvictionInterval = 42,

  not_set = 0
}

impl StatsEvictionCase {
  #[allow(dead_code)]
  pub(crate) fn try_from(v: u32) -> ::std::option::Option<StatsEvictionCase> {
    match v {
      0 => Some(StatsEvictionCase::not_set),
      42 => Some(StatsEvictionCase::StatsEvictionInterval),
      _ => None
    }
  }
}
}  // pub mod bootstrap


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__bootstrap__v3__Admin_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct Admin {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Admin>
}

impl ::protobuf::Message for Admin {
  type MessageView<'msg> = AdminView<'msg>;
  type MessageMut<'msg> = AdminMut<'msg>;
}

impl ::std::default::Default for Admin {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for Admin {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `Admin` is `Sync` because it does not implement interior mutability.
//    Neither does `AdminMut`.
unsafe impl ::std::marker::Sync for Admin {}

// SAFETY:
// - `Admin` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for Admin {}

impl ::protobuf::Proxied for Admin {
  type View<'msg> = AdminView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Admin {}

impl ::protobuf::MutProxied for Admin {
  type Mut<'msg> = AdminMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct AdminView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Admin>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for AdminView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for AdminView<'msg> {
  type Message = Admin;
}

impl ::std::fmt::Debug for AdminView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for AdminView<'_> {
  fn default() -> AdminView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Admin>> for AdminView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Admin>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> AdminView<'msg> {

  pub fn to_owned(&self) -> Admin {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // access_log: repeated message envoy.config.accesslog.v3.AccessLog
  pub fn access_log(self) -> ::protobuf::RepeatedView<'msg, crate::xds::generated::envoy::config::accesslog::v3::accesslog::AccessLog> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        4
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::accesslog::v3::accesslog::AccessLog>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // access_log_path: optional string
  pub fn access_log_path(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // profile_path: optional string
  pub fn profile_path(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // address: optional message envoy.config.core.v3.Address
  pub fn has_address(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn address_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::address::AddressView<'msg>> {
    self.has_address().then(|| self.address())
  }
  pub fn address(self) -> crate::xds::generated::envoy::config::core::v3::address::AddressView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::address::AddressView::default())
  }

  // socket_options: repeated message envoy.config.core.v3.SocketOption
  pub fn socket_options(self) -> ::protobuf::RepeatedView<'msg, crate::xds::generated::envoy::config::core::v3::socket_option::SocketOption> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        3
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::core::v3::socket_option::SocketOption>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // ignore_global_conn_limit: optional bool
  pub fn ignore_global_conn_limit(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        5, (false).into()
      ).try_into().unwrap()
    }
  }

  // allow_paths: repeated message envoy.type.matcher.v3.StringMatcher
  pub fn allow_paths(self) -> ::protobuf::RepeatedView<'msg, crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcher> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        6
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcher>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

}

// SAFETY:
// - `AdminView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for AdminView<'_> {}

// SAFETY:
// - `AdminView` is `Send` because while its alive a `AdminMut` cannot.
// - `AdminView` does not use thread-local data.
unsafe impl ::std::marker::Send for AdminView<'_> {}

impl<'msg> ::protobuf::AsView for AdminView<'msg> {
  type Proxied = Admin;
  fn as_view(&self) -> ::protobuf::View<'msg, Admin> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for AdminView<'msg> {
  fn into_view<'shorter>(self) -> AdminView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Admin> for AdminView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Admin {
    let mut dst = Admin::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Admin> for AdminMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Admin {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for Admin {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for AdminView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for AdminMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct AdminMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Admin>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for AdminMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for AdminMut<'msg> {
  type Message = Admin;
}

impl ::std::fmt::Debug for AdminMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Admin>> for AdminMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Admin>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> AdminMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Admin> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> Admin {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // access_log: repeated message envoy.config.accesslog.v3.AccessLog
  pub fn access_log(&self) -> ::protobuf::RepeatedView<'_, crate::xds::generated::envoy::config::accesslog::v3::accesslog::AccessLog> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        4
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::accesslog::v3::accesslog::AccessLog>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn access_log_mut(&mut self) -> ::protobuf::RepeatedMut<'_, crate::xds::generated::envoy::config::accesslog::v3::accesslog::AccessLog> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        4,
        self.inner.arena()
      ).expect("alloc should not fail");
      ::protobuf::RepeatedMut::from_inner(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::InnerRepeatedMut::new(
          raw_array, self.inner.arena(),
        ),
      )
    }
  }
  pub fn set_access_log(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<crate::xds::generated::envoy::config::accesslog::v3::accesslog::AccessLog>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        src);
    }
  }

  // access_log_path: optional string
  pub fn access_log_path(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_access_log_path(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // profile_path: optional string
  pub fn profile_path(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_profile_path(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // address: optional message envoy.config.core.v3.Address
  pub fn has_address(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_address(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn address_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::address::AddressView<'_>> {
    self.has_address().then(|| self.address())
  }
  pub fn address(&self) -> crate::xds::generated::envoy::config::core::v3::address::AddressView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::address::AddressView::default())
  }
  pub fn address_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::address::AddressMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         2, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_address(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::address::Address>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // socket_options: repeated message envoy.config.core.v3.SocketOption
  pub fn socket_options(&self) -> ::protobuf::RepeatedView<'_, crate::xds::generated::envoy::config::core::v3::socket_option::SocketOption> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        3
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::core::v3::socket_option::SocketOption>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn socket_options_mut(&mut self) -> ::protobuf::RepeatedMut<'_, crate::xds::generated::envoy::config::core::v3::socket_option::SocketOption> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        3,
        self.inner.arena()
      ).expect("alloc should not fail");
      ::protobuf::RepeatedMut::from_inner(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::InnerRepeatedMut::new(
          raw_array, self.inner.arena(),
        ),
      )
    }
  }
  pub fn set_socket_options(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<crate::xds::generated::envoy::config::core::v3::socket_option::SocketOption>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        src);
    }
  }

  // ignore_global_conn_limit: optional bool
  pub fn ignore_global_conn_limit(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        5, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_ignore_global_conn_limit(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        5, val.into()
      )
    }
  }

  // allow_paths: repeated message envoy.type.matcher.v3.StringMatcher
  pub fn allow_paths(&self) -> ::protobuf::RepeatedView<'_, crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcher> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        6
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcher>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn allow_paths_mut(&mut self) -> ::protobuf::RepeatedMut<'_, crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcher> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        6,
        self.inner.arena()
      ).expect("alloc should not fail");
      ::protobuf::RepeatedMut::from_inner(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::InnerRepeatedMut::new(
          raw_array, self.inner.arena(),
        ),
      )
    }
  }
  pub fn set_allow_paths(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcher>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        src);
    }
  }

}

// SAFETY:
// - `AdminMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for AdminMut<'_> {}

// SAFETY:
// - `AdminMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for AdminMut<'_> {}

impl<'msg> ::protobuf::AsView for AdminMut<'msg> {
  type Proxied = Admin;
  fn as_view(&self) -> ::protobuf::View<'_, Admin> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for AdminMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Admin>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for AdminMut<'msg> {
  type MutProxied = Admin;
  fn as_mut(&mut self) -> AdminMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for AdminMut<'msg> {
  fn into_mut<'shorter>(self) -> AdminMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Admin {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Admin> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> AdminView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> AdminMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // access_log: repeated message envoy.config.accesslog.v3.AccessLog
  pub fn access_log(&self) -> ::protobuf::RepeatedView<'_, crate::xds::generated::envoy::config::accesslog::v3::accesslog::AccessLog> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        4
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::accesslog::v3::accesslog::AccessLog>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn access_log_mut(&mut self) -> ::protobuf::RepeatedMut<'_, crate::xds::generated::envoy::config::accesslog::v3::accesslog::AccessLog> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        4,
        self.inner.arena()
      ).expect("alloc should not fail");
      ::protobuf::RepeatedMut::from_inner(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::InnerRepeatedMut::new(
          raw_array, self.inner.arena(),
        ),
      )
    }
  }
  pub fn set_access_log(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<crate::xds::generated::envoy::config::accesslog::v3::accesslog::AccessLog>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        src);
    }
  }

  // access_log_path: optional string
  pub fn access_log_path(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_access_log_path(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // profile_path: optional string
  pub fn profile_path(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_profile_path(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // address: optional message envoy.config.core.v3.Address
  pub fn has_address(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_address(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn address_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::address::AddressView<'_>> {
    self.has_address().then(|| self.address())
  }
  pub fn address(&self) -> crate::xds::generated::envoy::config::core::v3::address::AddressView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::address::AddressView::default())
  }
  pub fn address_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::address::AddressMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         2, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_address(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::address::Address>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // socket_options: repeated message envoy.config.core.v3.SocketOption
  pub fn socket_options(&self) -> ::protobuf::RepeatedView<'_, crate::xds::generated::envoy::config::core::v3::socket_option::SocketOption> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        3
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::config::core::v3::socket_option::SocketOption>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn socket_options_mut(&mut self) -> ::protobuf::RepeatedMut<'_, crate::xds::generated::envoy::config::core::v3::socket_option::SocketOption> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        3,
        self.inner.arena()
      ).expect("alloc should not fail");
      ::protobuf::RepeatedMut::from_inner(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::InnerRepeatedMut::new(
          raw_array, self.inner.arena(),
        ),
      )
    }
  }
  pub fn set_socket_options(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<crate::xds::generated::envoy::config::core::v3::socket_option::SocketOption>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        src);
    }
  }

  // ignore_global_conn_limit: optional bool
  pub fn ignore_global_conn_limit(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        5, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_ignore_global_conn_limit(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        5, val.into()
      )
    }
  }

  // allow_paths: repeated message envoy.type.matcher.v3.StringMatcher
  pub fn allow_paths(&self) -> ::protobuf::RepeatedView<'_, crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcher> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        6
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcher>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn allow_paths_mut(&mut self) -> ::protobuf::RepeatedMut<'_, crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcher> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        6,
        self.inner.arena()
      ).expect("alloc should not fail");
      ::protobuf::RepeatedMut::from_inner(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::InnerRepeatedMut::new(
          raw_array, self.inner.arena(),
        ),
      )
    }
  }
  pub fn set_allow_paths(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcher>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        src);
    }
  }

}  // impl Admin

impl ::std::ops::Drop for Admin {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Admin {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Admin {
  type Proxied = Self;
  fn as_view(&self) -> AdminView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Admin {
  type MutProxied = Self;
  fn as_mut(&mut self) -> AdminMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Admin {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__bootstrap__v3__Admin_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1X1X3GG/PG");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__bootstrap__v3__Admin_msg_init.0, &[<crate::xds::generated::envoy::config::core::v3::address::Address as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::core::v3::socket_option::SocketOption as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::accesslog::v3::accesslog::AccessLog as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::r#type::matcher::v3::string::StringMatcher as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__bootstrap__v3__Admin_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Admin {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Admin {
  type Msg = Admin;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Admin> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Admin {
  type Msg = Admin;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Admin> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for AdminMut<'_> {
  type Msg = Admin;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Admin> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for AdminMut<'_> {
  type Msg = Admin;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Admin> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for AdminView<'_> {
  type Msg = Admin;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Admin> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for AdminMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__bootstrap__v3__ClusterManager_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct ClusterManager {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<ClusterManager>
}

impl ::protobuf::Message for ClusterManager {
  type MessageView<'msg> = ClusterManagerView<'msg>;
  type MessageMut<'msg> = ClusterManagerMut<'msg>;
}

impl ::std::default::Default for ClusterManager {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for ClusterManager {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `ClusterManager` is `Sync` because it does not implement interior mutability.
//    Neither does `ClusterManagerMut`.
unsafe impl ::std::marker::Sync for ClusterManager {}

// SAFETY:
// - `ClusterManager` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for ClusterManager {}

impl ::protobuf::Proxied for ClusterManager {
  type View<'msg> = ClusterManagerView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for ClusterManager {}

impl ::protobuf::MutProxied for ClusterManager {
  type Mut<'msg> = ClusterManagerMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ClusterManagerView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ClusterManager>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ClusterManagerView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ClusterManagerView<'msg> {
  type Message = ClusterManager;
}

impl ::std::fmt::Debug for ClusterManagerView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ClusterManagerView<'_> {
  fn default() -> ClusterManagerView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, ClusterManager>> for ClusterManagerView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ClusterManager>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ClusterManagerView<'msg> {

  pub fn to_owned(&self) -> ClusterManager {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // local_cluster_name: optional string
  pub fn local_cluster_name(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // outlier_detection: optional message envoy.config.bootstrap.v3.ClusterManager.OutlierDetection
  pub fn has_outlier_detection(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn outlier_detection_opt(self) -> ::std::option::Option<super::cluster_manager::OutlierDetectionView<'msg>> {
    self.has_outlier_detection().then(|| self.outlier_detection())
  }
  pub fn outlier_detection(self) -> super::cluster_manager::OutlierDetectionView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::cluster_manager::OutlierDetectionView::default())
  }

  // upstream_bind_config: optional message envoy.config.core.v3.BindConfig
  pub fn has_upstream_bind_config(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn upstream_bind_config_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::address::BindConfigView<'msg>> {
    self.has_upstream_bind_config().then(|| self.upstream_bind_config())
  }
  pub fn upstream_bind_config(self) -> crate::xds::generated::envoy::config::core::v3::address::BindConfigView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::address::BindConfigView::default())
  }

  // load_stats_config: optional message envoy.config.core.v3.ApiConfigSource
  pub fn has_load_stats_config(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn load_stats_config_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::config_source::ApiConfigSourceView<'msg>> {
    self.has_load_stats_config().then(|| self.load_stats_config())
  }
  pub fn load_stats_config(self) -> crate::xds::generated::envoy::config::core::v3::config_source::ApiConfigSourceView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::config_source::ApiConfigSourceView::default())
  }

  // enable_deferred_cluster_creation: optional bool
  pub fn enable_deferred_cluster_creation(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        4, (false).into()
      ).try_into().unwrap()
    }
  }

}

// SAFETY:
// - `ClusterManagerView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for ClusterManagerView<'_> {}

// SAFETY:
// - `ClusterManagerView` is `Send` because while its alive a `ClusterManagerMut` cannot.
// - `ClusterManagerView` does not use thread-local data.
unsafe impl ::std::marker::Send for ClusterManagerView<'_> {}

impl<'msg> ::protobuf::AsView for ClusterManagerView<'msg> {
  type Proxied = ClusterManager;
  fn as_view(&self) -> ::protobuf::View<'msg, ClusterManager> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ClusterManagerView<'msg> {
  fn into_view<'shorter>(self) -> ClusterManagerView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<ClusterManager> for ClusterManagerView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ClusterManager {
    let mut dst = ClusterManager::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<ClusterManager> for ClusterManagerMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ClusterManager {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for ClusterManager {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ClusterManagerView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ClusterManagerMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ClusterManagerMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ClusterManager>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ClusterManagerMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ClusterManagerMut<'msg> {
  type Message = ClusterManager;
}

impl ::std::fmt::Debug for ClusterManagerMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, ClusterManager>> for ClusterManagerMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ClusterManager>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ClusterManagerMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, ClusterManager> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> ClusterManager {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // local_cluster_name: optional string
  pub fn local_cluster_name(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_local_cluster_name(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // outlier_detection: optional message envoy.config.bootstrap.v3.ClusterManager.OutlierDetection
  pub fn has_outlier_detection(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_outlier_detection(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn outlier_detection_opt(&self) -> ::std::option::Option<super::cluster_manager::OutlierDetectionView<'_>> {
    self.has_outlier_detection().then(|| self.outlier_detection())
  }
  pub fn outlier_detection(&self) -> super::cluster_manager::OutlierDetectionView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::cluster_manager::OutlierDetectionView::default())
  }
  pub fn outlier_detection_mut(&mut self) -> super::cluster_manager::OutlierDetectionMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         1, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_outlier_detection(&mut self,
    val: impl ::protobuf::IntoProxied<super::cluster_manager::OutlierDetection>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // upstream_bind_config: optional message envoy.config.core.v3.BindConfig
  pub fn has_upstream_bind_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_upstream_bind_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn upstream_bind_config_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::address::BindConfigView<'_>> {
    self.has_upstream_bind_config().then(|| self.upstream_bind_config())
  }
  pub fn upstream_bind_config(&self) -> crate::xds::generated::envoy::config::core::v3::address::BindConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::address::BindConfigView::default())
  }
  pub fn upstream_bind_config_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::address::BindConfigMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         2, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_upstream_bind_config(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::address::BindConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // load_stats_config: optional message envoy.config.core.v3.ApiConfigSource
  pub fn has_load_stats_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_load_stats_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn load_stats_config_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::config_source::ApiConfigSourceView<'_>> {
    self.has_load_stats_config().then(|| self.load_stats_config())
  }
  pub fn load_stats_config(&self) -> crate::xds::generated::envoy::config::core::v3::config_source::ApiConfigSourceView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::config_source::ApiConfigSourceView::default())
  }
  pub fn load_stats_config_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::config_source::ApiConfigSourceMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         3, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_load_stats_config(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::config_source::ApiConfigSource>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // enable_deferred_cluster_creation: optional bool
  pub fn enable_deferred_cluster_creation(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        4, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_enable_deferred_cluster_creation(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        4, val.into()
      )
    }
  }

}

// SAFETY:
// - `ClusterManagerMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for ClusterManagerMut<'_> {}

// SAFETY:
// - `ClusterManagerMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for ClusterManagerMut<'_> {}

impl<'msg> ::protobuf::AsView for ClusterManagerMut<'msg> {
  type Proxied = ClusterManager;
  fn as_view(&self) -> ::protobuf::View<'_, ClusterManager> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ClusterManagerMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, ClusterManager>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for ClusterManagerMut<'msg> {
  type MutProxied = ClusterManager;
  fn as_mut(&mut self) -> ClusterManagerMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ClusterManagerMut<'msg> {
  fn into_mut<'shorter>(self) -> ClusterManagerMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl ClusterManager {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, ClusterManager> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ClusterManagerView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ClusterManagerMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // local_cluster_name: optional string
  pub fn local_cluster_name(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_local_cluster_name(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // outlier_detection: optional message envoy.config.bootstrap.v3.ClusterManager.OutlierDetection
  pub fn has_outlier_detection(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_outlier_detection(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn outlier_detection_opt(&self) -> ::std::option::Option<super::cluster_manager::OutlierDetectionView<'_>> {
    self.has_outlier_detection().then(|| self.outlier_detection())
  }
  pub fn outlier_detection(&self) -> super::cluster_manager::OutlierDetectionView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::cluster_manager::OutlierDetectionView::default())
  }
  pub fn outlier_detection_mut(&mut self) -> super::cluster_manager::OutlierDetectionMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         1, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_outlier_detection(&mut self,
    val: impl ::protobuf::IntoProxied<super::cluster_manager::OutlierDetection>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // upstream_bind_config: optional message envoy.config.core.v3.BindConfig
  pub fn has_upstream_bind_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_upstream_bind_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn upstream_bind_config_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::address::BindConfigView<'_>> {
    self.has_upstream_bind_config().then(|| self.upstream_bind_config())
  }
  pub fn upstream_bind_config(&self) -> crate::xds::generated::envoy::config::core::v3::address::BindConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::address::BindConfigView::default())
  }
  pub fn upstream_bind_config_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::address::BindConfigMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         2, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_upstream_bind_config(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::address::BindConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // load_stats_config: optional message envoy.config.core.v3.ApiConfigSource
  pub fn has_load_stats_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_load_stats_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn load_stats_config_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::config_source::ApiConfigSourceView<'_>> {
    self.has_load_stats_config().then(|| self.load_stats_config())
  }
  pub fn load_stats_config(&self) -> crate::xds::generated::envoy::config::core::v3::config_source::ApiConfigSourceView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::config_source::ApiConfigSourceView::default())
  }
  pub fn load_stats_config_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::config_source::ApiConfigSourceMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         3, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_load_stats_config(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::config_source::ApiConfigSource>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // enable_deferred_cluster_creation: optional bool
  pub fn enable_deferred_cluster_creation(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        4, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_enable_deferred_cluster_creation(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        4, val.into()
      )
    }
  }

}  // impl ClusterManager

impl ::std::ops::Drop for ClusterManager {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for ClusterManager {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for ClusterManager {
  type Proxied = Self;
  fn as_view(&self) -> ClusterManagerView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for ClusterManager {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ClusterManagerMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for ClusterManager {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__bootstrap__v3__ClusterManager_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1X333/P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__bootstrap__v3__ClusterManager_msg_init.0, &[<super::cluster_manager::OutlierDetection as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::core::v3::address::BindConfig as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::config::core::v3::config_source::ApiConfigSource as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__bootstrap__v3__ClusterManager_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ClusterManager {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ClusterManager {
  type Msg = ClusterManager;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ClusterManager> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ClusterManager {
  type Msg = ClusterManager;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ClusterManager> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ClusterManagerMut<'_> {
  type Msg = ClusterManager;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ClusterManager> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ClusterManagerMut<'_> {
  type Msg = ClusterManager;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ClusterManager> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ClusterManagerView<'_> {
  type Msg = ClusterManager;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ClusterManager> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ClusterManagerMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod cluster_manager {// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__bootstrap__v3__ClusterManager__OutlierDetection_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct OutlierDetection {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<OutlierDetection>
}

impl ::protobuf::Message for OutlierDetection {
  type MessageView<'msg> = OutlierDetectionView<'msg>;
  type MessageMut<'msg> = OutlierDetectionMut<'msg>;
}

impl ::std::default::Default for OutlierDetection {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for OutlierDetection {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `OutlierDetection` is `Sync` because it does not implement interior mutability.
//    Neither does `OutlierDetectionMut`.
unsafe impl ::std::marker::Sync for OutlierDetection {}

// SAFETY:
// - `OutlierDetection` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for OutlierDetection {}

impl ::protobuf::Proxied for OutlierDetection {
  type View<'msg> = OutlierDetectionView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for OutlierDetection {}

impl ::protobuf::MutProxied for OutlierDetection {
  type Mut<'msg> = OutlierDetectionMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct OutlierDetectionView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, OutlierDetection>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for OutlierDetectionView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for OutlierDetectionView<'msg> {
  type Message = OutlierDetection;
}

impl ::std::fmt::Debug for OutlierDetectionView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for OutlierDetectionView<'_> {
  fn default() -> OutlierDetectionView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, OutlierDetection>> for OutlierDetectionView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, OutlierDetection>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> OutlierDetectionView<'msg> {

  pub fn to_owned(&self) -> OutlierDetection {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // event_log_path: optional string
  pub fn event_log_path(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // event_service: optional message envoy.config.core.v3.EventServiceConfig
  pub fn has_event_service(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn event_service_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::event_service_config::EventServiceConfigView<'msg>> {
    self.has_event_service().then(|| self.event_service())
  }
  pub fn event_service(self) -> crate::xds::generated::envoy::config::core::v3::event_service_config::EventServiceConfigView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::event_service_config::EventServiceConfigView::default())
  }

}

// SAFETY:
// - `OutlierDetectionView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for OutlierDetectionView<'_> {}

// SAFETY:
// - `OutlierDetectionView` is `Send` because while its alive a `OutlierDetectionMut` cannot.
// - `OutlierDetectionView` does not use thread-local data.
unsafe impl ::std::marker::Send for OutlierDetectionView<'_> {}

impl<'msg> ::protobuf::AsView for OutlierDetectionView<'msg> {
  type Proxied = OutlierDetection;
  fn as_view(&self) -> ::protobuf::View<'msg, OutlierDetection> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for OutlierDetectionView<'msg> {
  fn into_view<'shorter>(self) -> OutlierDetectionView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<OutlierDetection> for OutlierDetectionView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> OutlierDetection {
    let mut dst = OutlierDetection::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<OutlierDetection> for OutlierDetectionMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> OutlierDetection {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for OutlierDetection {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for OutlierDetectionView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for OutlierDetectionMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct OutlierDetectionMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, OutlierDetection>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for OutlierDetectionMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for OutlierDetectionMut<'msg> {
  type Message = OutlierDetection;
}

impl ::std::fmt::Debug for OutlierDetectionMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, OutlierDetection>> for OutlierDetectionMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, OutlierDetection>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> OutlierDetectionMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, OutlierDetection> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> OutlierDetection {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // event_log_path: optional string
  pub fn event_log_path(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_event_log_path(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // event_service: optional message envoy.config.core.v3.EventServiceConfig
  pub fn has_event_service(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_event_service(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn event_service_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::event_service_config::EventServiceConfigView<'_>> {
    self.has_event_service().then(|| self.event_service())
  }
  pub fn event_service(&self) -> crate::xds::generated::envoy::config::core::v3::event_service_config::EventServiceConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::event_service_config::EventServiceConfigView::default())
  }
  pub fn event_service_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::event_service_config::EventServiceConfigMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         1, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_event_service(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::event_service_config::EventServiceConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

}

// SAFETY:
// - `OutlierDetectionMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for OutlierDetectionMut<'_> {}

// SAFETY:
// - `OutlierDetectionMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for OutlierDetectionMut<'_> {}

impl<'msg> ::protobuf::AsView for OutlierDetectionMut<'msg> {
  type Proxied = OutlierDetection;
  fn as_view(&self) -> ::protobuf::View<'_, OutlierDetection> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for OutlierDetectionMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, OutlierDetection>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for OutlierDetectionMut<'msg> {
  type MutProxied = OutlierDetection;
  fn as_mut(&mut self) -> OutlierDetectionMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for OutlierDetectionMut<'msg> {
  fn into_mut<'shorter>(self) -> OutlierDetectionMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl OutlierDetection {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, OutlierDetection> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> OutlierDetectionView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> OutlierDetectionMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // event_log_path: optional string
  pub fn event_log_path(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_event_log_path(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // event_service: optional message envoy.config.core.v3.EventServiceConfig
  pub fn has_event_service(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_event_service(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn event_service_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::event_service_config::EventServiceConfigView<'_>> {
    self.has_event_service().then(|| self.event_service())
  }
  pub fn event_service(&self) -> crate::xds::generated::envoy::config::core::v3::event_service_config::EventServiceConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::event_service_config::EventServiceConfigView::default())
  }
  pub fn event_service_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::event_service_config::EventServiceConfigMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         1, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_event_service(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::event_service_config::EventServiceConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

}  // impl OutlierDetection

impl ::std::ops::Drop for OutlierDetection {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for OutlierDetection {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for OutlierDetection {
  type Proxied = Self;
  fn as_view(&self) -> OutlierDetectionView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for OutlierDetection {
  type MutProxied = Self;
  fn as_mut(&mut self) -> OutlierDetectionMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for OutlierDetection {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::cluster_manager::envoy__config__bootstrap__v3__ClusterManager__OutlierDetection_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1X3");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::cluster_manager::envoy__config__bootstrap__v3__ClusterManager__OutlierDetection_msg_init.0, &[<crate::xds::generated::envoy::config::core::v3::event_service_config::EventServiceConfig as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::cluster_manager::envoy__config__bootstrap__v3__ClusterManager__OutlierDetection_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for OutlierDetection {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for OutlierDetection {
  type Msg = OutlierDetection;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<OutlierDetection> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for OutlierDetection {
  type Msg = OutlierDetection;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<OutlierDetection> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for OutlierDetectionMut<'_> {
  type Msg = OutlierDetection;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<OutlierDetection> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for OutlierDetectionMut<'_> {
  type Msg = OutlierDetection;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<OutlierDetection> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for OutlierDetectionView<'_> {
  type Msg = OutlierDetection;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<OutlierDetection> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for OutlierDetectionMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



}  // pub mod cluster_manager


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__bootstrap__v3__Watchdogs_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct Watchdogs {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Watchdogs>
}

impl ::protobuf::Message for Watchdogs {
  type MessageView<'msg> = WatchdogsView<'msg>;
  type MessageMut<'msg> = WatchdogsMut<'msg>;
}

impl ::std::default::Default for Watchdogs {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for Watchdogs {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `Watchdogs` is `Sync` because it does not implement interior mutability.
//    Neither does `WatchdogsMut`.
unsafe impl ::std::marker::Sync for Watchdogs {}

// SAFETY:
// - `Watchdogs` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for Watchdogs {}

impl ::protobuf::Proxied for Watchdogs {
  type View<'msg> = WatchdogsView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Watchdogs {}

impl ::protobuf::MutProxied for Watchdogs {
  type Mut<'msg> = WatchdogsMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct WatchdogsView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Watchdogs>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for WatchdogsView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for WatchdogsView<'msg> {
  type Message = Watchdogs;
}

impl ::std::fmt::Debug for WatchdogsView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for WatchdogsView<'_> {
  fn default() -> WatchdogsView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Watchdogs>> for WatchdogsView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Watchdogs>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> WatchdogsView<'msg> {

  pub fn to_owned(&self) -> Watchdogs {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // main_thread_watchdog: optional message envoy.config.bootstrap.v3.Watchdog
  pub fn has_main_thread_watchdog(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn main_thread_watchdog_opt(self) -> ::std::option::Option<super::WatchdogView<'msg>> {
    self.has_main_thread_watchdog().then(|| self.main_thread_watchdog())
  }
  pub fn main_thread_watchdog(self) -> super::WatchdogView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::WatchdogView::default())
  }

  // worker_watchdog: optional message envoy.config.bootstrap.v3.Watchdog
  pub fn has_worker_watchdog(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn worker_watchdog_opt(self) -> ::std::option::Option<super::WatchdogView<'msg>> {
    self.has_worker_watchdog().then(|| self.worker_watchdog())
  }
  pub fn worker_watchdog(self) -> super::WatchdogView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::WatchdogView::default())
  }

}

// SAFETY:
// - `WatchdogsView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for WatchdogsView<'_> {}

// SAFETY:
// - `WatchdogsView` is `Send` because while its alive a `WatchdogsMut` cannot.
// - `WatchdogsView` does not use thread-local data.
unsafe impl ::std::marker::Send for WatchdogsView<'_> {}

impl<'msg> ::protobuf::AsView for WatchdogsView<'msg> {
  type Proxied = Watchdogs;
  fn as_view(&self) -> ::protobuf::View<'msg, Watchdogs> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for WatchdogsView<'msg> {
  fn into_view<'shorter>(self) -> WatchdogsView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Watchdogs> for WatchdogsView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Watchdogs {
    let mut dst = Watchdogs::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Watchdogs> for WatchdogsMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Watchdogs {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for Watchdogs {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for WatchdogsView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for WatchdogsMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct WatchdogsMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Watchdogs>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for WatchdogsMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for WatchdogsMut<'msg> {
  type Message = Watchdogs;
}

impl ::std::fmt::Debug for WatchdogsMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Watchdogs>> for WatchdogsMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Watchdogs>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> WatchdogsMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Watchdogs> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> Watchdogs {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // main_thread_watchdog: optional message envoy.config.bootstrap.v3.Watchdog
  pub fn has_main_thread_watchdog(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_main_thread_watchdog(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn main_thread_watchdog_opt(&self) -> ::std::option::Option<super::WatchdogView<'_>> {
    self.has_main_thread_watchdog().then(|| self.main_thread_watchdog())
  }
  pub fn main_thread_watchdog(&self) -> super::WatchdogView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::WatchdogView::default())
  }
  pub fn main_thread_watchdog_mut(&mut self) -> super::WatchdogMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         0, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_main_thread_watchdog(&mut self,
    val: impl ::protobuf::IntoProxied<super::Watchdog>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // worker_watchdog: optional message envoy.config.bootstrap.v3.Watchdog
  pub fn has_worker_watchdog(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_worker_watchdog(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn worker_watchdog_opt(&self) -> ::std::option::Option<super::WatchdogView<'_>> {
    self.has_worker_watchdog().then(|| self.worker_watchdog())
  }
  pub fn worker_watchdog(&self) -> super::WatchdogView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::WatchdogView::default())
  }
  pub fn worker_watchdog_mut(&mut self) -> super::WatchdogMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         1, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_worker_watchdog(&mut self,
    val: impl ::protobuf::IntoProxied<super::Watchdog>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

}

// SAFETY:
// - `WatchdogsMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for WatchdogsMut<'_> {}

// SAFETY:
// - `WatchdogsMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for WatchdogsMut<'_> {}

impl<'msg> ::protobuf::AsView for WatchdogsMut<'msg> {
  type Proxied = Watchdogs;
  fn as_view(&self) -> ::protobuf::View<'_, Watchdogs> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for WatchdogsMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Watchdogs>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for WatchdogsMut<'msg> {
  type MutProxied = Watchdogs;
  fn as_mut(&mut self) -> WatchdogsMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for WatchdogsMut<'msg> {
  fn into_mut<'shorter>(self) -> WatchdogsMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Watchdogs {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Watchdogs> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> WatchdogsView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> WatchdogsMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // main_thread_watchdog: optional message envoy.config.bootstrap.v3.Watchdog
  pub fn has_main_thread_watchdog(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_main_thread_watchdog(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn main_thread_watchdog_opt(&self) -> ::std::option::Option<super::WatchdogView<'_>> {
    self.has_main_thread_watchdog().then(|| self.main_thread_watchdog())
  }
  pub fn main_thread_watchdog(&self) -> super::WatchdogView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::WatchdogView::default())
  }
  pub fn main_thread_watchdog_mut(&mut self) -> super::WatchdogMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         0, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_main_thread_watchdog(&mut self,
    val: impl ::protobuf::IntoProxied<super::Watchdog>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // worker_watchdog: optional message envoy.config.bootstrap.v3.Watchdog
  pub fn has_worker_watchdog(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_worker_watchdog(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn worker_watchdog_opt(&self) -> ::std::option::Option<super::WatchdogView<'_>> {
    self.has_worker_watchdog().then(|| self.worker_watchdog())
  }
  pub fn worker_watchdog(&self) -> super::WatchdogView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::WatchdogView::default())
  }
  pub fn worker_watchdog_mut(&mut self) -> super::WatchdogMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         1, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_worker_watchdog(&mut self,
    val: impl ::protobuf::IntoProxied<super::Watchdog>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

}  // impl Watchdogs

impl ::std::ops::Drop for Watchdogs {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Watchdogs {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Watchdogs {
  type Proxied = Self;
  fn as_view(&self) -> WatchdogsView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Watchdogs {
  type MutProxied = Self;
  fn as_mut(&mut self) -> WatchdogsMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Watchdogs {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__bootstrap__v3__Watchdogs_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$33");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__bootstrap__v3__Watchdogs_msg_init.0, &[<super::Watchdog as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::Watchdog as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__bootstrap__v3__Watchdogs_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Watchdogs {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Watchdogs {
  type Msg = Watchdogs;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Watchdogs> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Watchdogs {
  type Msg = Watchdogs;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Watchdogs> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for WatchdogsMut<'_> {
  type Msg = Watchdogs;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Watchdogs> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for WatchdogsMut<'_> {
  type Msg = Watchdogs;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Watchdogs> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for WatchdogsView<'_> {
  type Msg = Watchdogs;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Watchdogs> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for WatchdogsMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__bootstrap__v3__Watchdog_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct Watchdog {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Watchdog>
}

impl ::protobuf::Message for Watchdog {
  type MessageView<'msg> = WatchdogView<'msg>;
  type MessageMut<'msg> = WatchdogMut<'msg>;
}

impl ::std::default::Default for Watchdog {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for Watchdog {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `Watchdog` is `Sync` because it does not implement interior mutability.
//    Neither does `WatchdogMut`.
unsafe impl ::std::marker::Sync for Watchdog {}

// SAFETY:
// - `Watchdog` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for Watchdog {}

impl ::protobuf::Proxied for Watchdog {
  type View<'msg> = WatchdogView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Watchdog {}

impl ::protobuf::MutProxied for Watchdog {
  type Mut<'msg> = WatchdogMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct WatchdogView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Watchdog>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for WatchdogView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for WatchdogView<'msg> {
  type Message = Watchdog;
}

impl ::std::fmt::Debug for WatchdogView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for WatchdogView<'_> {
  fn default() -> WatchdogView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Watchdog>> for WatchdogView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Watchdog>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> WatchdogView<'msg> {

  pub fn to_owned(&self) -> Watchdog {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // actions: repeated message envoy.config.bootstrap.v3.Watchdog.WatchdogAction
  pub fn actions(self) -> ::protobuf::RepeatedView<'msg, super::watchdog::WatchdogAction> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        6
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::watchdog::WatchdogAction>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // miss_timeout: optional message google.protobuf.Duration
  pub fn has_miss_timeout(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn miss_timeout_opt(self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'msg>> {
    self.has_miss_timeout().then(|| self.miss_timeout())
  }
  pub fn miss_timeout(self) -> ::protobuf_well_known_types::DurationView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }

  // megamiss_timeout: optional message google.protobuf.Duration
  pub fn has_megamiss_timeout(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn megamiss_timeout_opt(self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'msg>> {
    self.has_megamiss_timeout().then(|| self.megamiss_timeout())
  }
  pub fn megamiss_timeout(self) -> ::protobuf_well_known_types::DurationView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }

  // kill_timeout: optional message google.protobuf.Duration
  pub fn has_kill_timeout(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn kill_timeout_opt(self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'msg>> {
    self.has_kill_timeout().then(|| self.kill_timeout())
  }
  pub fn kill_timeout(self) -> ::protobuf_well_known_types::DurationView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }

  // max_kill_timeout_jitter: optional message google.protobuf.Duration
  pub fn has_max_kill_timeout_jitter(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn max_kill_timeout_jitter_opt(self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'msg>> {
    self.has_max_kill_timeout_jitter().then(|| self.max_kill_timeout_jitter())
  }
  pub fn max_kill_timeout_jitter(self) -> ::protobuf_well_known_types::DurationView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }

  // multikill_timeout: optional message google.protobuf.Duration
  pub fn has_multikill_timeout(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn multikill_timeout_opt(self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'msg>> {
    self.has_multikill_timeout().then(|| self.multikill_timeout())
  }
  pub fn multikill_timeout(self) -> ::protobuf_well_known_types::DurationView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }

  // multikill_threshold: optional message envoy.type.v3.Percent
  pub fn has_multikill_threshold(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn multikill_threshold_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::v3::percent::PercentView<'msg>> {
    self.has_multikill_threshold().then(|| self.multikill_threshold())
  }
  pub fn multikill_threshold(self) -> crate::xds::generated::envoy::r#type::v3::percent::PercentView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::v3::percent::PercentView::default())
  }

}

// SAFETY:
// - `WatchdogView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for WatchdogView<'_> {}

// SAFETY:
// - `WatchdogView` is `Send` because while its alive a `WatchdogMut` cannot.
// - `WatchdogView` does not use thread-local data.
unsafe impl ::std::marker::Send for WatchdogView<'_> {}

impl<'msg> ::protobuf::AsView for WatchdogView<'msg> {
  type Proxied = Watchdog;
  fn as_view(&self) -> ::protobuf::View<'msg, Watchdog> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for WatchdogView<'msg> {
  fn into_view<'shorter>(self) -> WatchdogView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Watchdog> for WatchdogView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Watchdog {
    let mut dst = Watchdog::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Watchdog> for WatchdogMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Watchdog {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for Watchdog {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for WatchdogView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for WatchdogMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct WatchdogMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Watchdog>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for WatchdogMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for WatchdogMut<'msg> {
  type Message = Watchdog;
}

impl ::std::fmt::Debug for WatchdogMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Watchdog>> for WatchdogMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Watchdog>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> WatchdogMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Watchdog> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> Watchdog {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // actions: repeated message envoy.config.bootstrap.v3.Watchdog.WatchdogAction
  pub fn actions(&self) -> ::protobuf::RepeatedView<'_, super::watchdog::WatchdogAction> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        6
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::watchdog::WatchdogAction>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn actions_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::watchdog::WatchdogAction> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        6,
        self.inner.arena()
      ).expect("alloc should not fail");
      ::protobuf::RepeatedMut::from_inner(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::InnerRepeatedMut::new(
          raw_array, self.inner.arena(),
        ),
      )
    }
  }
  pub fn set_actions(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::watchdog::WatchdogAction>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        src);
    }
  }

  // miss_timeout: optional message google.protobuf.Duration
  pub fn has_miss_timeout(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_miss_timeout(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn miss_timeout_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_miss_timeout().then(|| self.miss_timeout())
  }
  pub fn miss_timeout(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn miss_timeout_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         0, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_miss_timeout(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // megamiss_timeout: optional message google.protobuf.Duration
  pub fn has_megamiss_timeout(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_megamiss_timeout(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn megamiss_timeout_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_megamiss_timeout().then(|| self.megamiss_timeout())
  }
  pub fn megamiss_timeout(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn megamiss_timeout_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         1, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_megamiss_timeout(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // kill_timeout: optional message google.protobuf.Duration
  pub fn has_kill_timeout(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_kill_timeout(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn kill_timeout_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_kill_timeout().then(|| self.kill_timeout())
  }
  pub fn kill_timeout(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn kill_timeout_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         2, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_kill_timeout(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // max_kill_timeout_jitter: optional message google.protobuf.Duration
  pub fn has_max_kill_timeout_jitter(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn clear_max_kill_timeout_jitter(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        5
      );
    }
  }
  pub fn max_kill_timeout_jitter_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_max_kill_timeout_jitter().then(|| self.max_kill_timeout_jitter())
  }
  pub fn max_kill_timeout_jitter(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn max_kill_timeout_jitter_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         5, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_max_kill_timeout_jitter(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val
      );
    }
  }

  // multikill_timeout: optional message google.protobuf.Duration
  pub fn has_multikill_timeout(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_multikill_timeout(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn multikill_timeout_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_multikill_timeout().then(|| self.multikill_timeout())
  }
  pub fn multikill_timeout(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn multikill_timeout_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         3, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_multikill_timeout(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // multikill_threshold: optional message envoy.type.v3.Percent
  pub fn has_multikill_threshold(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_multikill_threshold(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn multikill_threshold_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::v3::percent::PercentView<'_>> {
    self.has_multikill_threshold().then(|| self.multikill_threshold())
  }
  pub fn multikill_threshold(&self) -> crate::xds::generated::envoy::r#type::v3::percent::PercentView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::v3::percent::PercentView::default())
  }
  pub fn multikill_threshold_mut(&mut self) -> crate::xds::generated::envoy::r#type::v3::percent::PercentMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         4, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_multikill_threshold(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::r#type::v3::percent::Percent>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val
      );
    }
  }

}

// SAFETY:
// - `WatchdogMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for WatchdogMut<'_> {}

// SAFETY:
// - `WatchdogMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for WatchdogMut<'_> {}

impl<'msg> ::protobuf::AsView for WatchdogMut<'msg> {
  type Proxied = Watchdog;
  fn as_view(&self) -> ::protobuf::View<'_, Watchdog> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for WatchdogMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Watchdog>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for WatchdogMut<'msg> {
  type MutProxied = Watchdog;
  fn as_mut(&mut self) -> WatchdogMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for WatchdogMut<'msg> {
  fn into_mut<'shorter>(self) -> WatchdogMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Watchdog {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Watchdog> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> WatchdogView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> WatchdogMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // actions: repeated message envoy.config.bootstrap.v3.Watchdog.WatchdogAction
  pub fn actions(&self) -> ::protobuf::RepeatedView<'_, super::watchdog::WatchdogAction> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        6
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::watchdog::WatchdogAction>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn actions_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::watchdog::WatchdogAction> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        6,
        self.inner.arena()
      ).expect("alloc should not fail");
      ::protobuf::RepeatedMut::from_inner(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::InnerRepeatedMut::new(
          raw_array, self.inner.arena(),
        ),
      )
    }
  }
  pub fn set_actions(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::watchdog::WatchdogAction>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        src);
    }
  }

  // miss_timeout: optional message google.protobuf.Duration
  pub fn has_miss_timeout(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_miss_timeout(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn miss_timeout_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_miss_timeout().then(|| self.miss_timeout())
  }
  pub fn miss_timeout(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn miss_timeout_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         0, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_miss_timeout(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // megamiss_timeout: optional message google.protobuf.Duration
  pub fn has_megamiss_timeout(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_megamiss_timeout(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn megamiss_timeout_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_megamiss_timeout().then(|| self.megamiss_timeout())
  }
  pub fn megamiss_timeout(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn megamiss_timeout_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         1, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_megamiss_timeout(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // kill_timeout: optional message google.protobuf.Duration
  pub fn has_kill_timeout(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_kill_timeout(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn kill_timeout_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_kill_timeout().then(|| self.kill_timeout())
  }
  pub fn kill_timeout(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn kill_timeout_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         2, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_kill_timeout(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // max_kill_timeout_jitter: optional message google.protobuf.Duration
  pub fn has_max_kill_timeout_jitter(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn clear_max_kill_timeout_jitter(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        5
      );
    }
  }
  pub fn max_kill_timeout_jitter_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_max_kill_timeout_jitter().then(|| self.max_kill_timeout_jitter())
  }
  pub fn max_kill_timeout_jitter(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn max_kill_timeout_jitter_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         5, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_max_kill_timeout_jitter(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val
      );
    }
  }

  // multikill_timeout: optional message google.protobuf.Duration
  pub fn has_multikill_timeout(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_multikill_timeout(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn multikill_timeout_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_multikill_timeout().then(|| self.multikill_timeout())
  }
  pub fn multikill_timeout(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn multikill_timeout_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         3, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_multikill_timeout(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // multikill_threshold: optional message envoy.type.v3.Percent
  pub fn has_multikill_threshold(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_multikill_threshold(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn multikill_threshold_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::r#type::v3::percent::PercentView<'_>> {
    self.has_multikill_threshold().then(|| self.multikill_threshold())
  }
  pub fn multikill_threshold(&self) -> crate::xds::generated::envoy::r#type::v3::percent::PercentView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::r#type::v3::percent::PercentView::default())
  }
  pub fn multikill_threshold_mut(&mut self) -> crate::xds::generated::envoy::r#type::v3::percent::PercentMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         4, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_multikill_threshold(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::r#type::v3::percent::Percent>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val
      );
    }
  }

}  // impl Watchdog

impl ::std::ops::Drop for Watchdog {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Watchdog {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Watchdog {
  type Proxied = Self;
  fn as_view(&self) -> WatchdogView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Watchdog {
  type MutProxied = Self;
  fn as_mut(&mut self) -> WatchdogMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Watchdog {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__bootstrap__v3__Watchdog_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$333333G");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__bootstrap__v3__Watchdog_msg_init.0, &[<::protobuf_well_known_types::Duration as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::Duration as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::Duration as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::Duration as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <crate::xds::generated::envoy::r#type::v3::percent::Percent as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <::protobuf_well_known_types::Duration as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::watchdog::WatchdogAction as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__bootstrap__v3__Watchdog_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Watchdog {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Watchdog {
  type Msg = Watchdog;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Watchdog> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Watchdog {
  type Msg = Watchdog;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Watchdog> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for WatchdogMut<'_> {
  type Msg = Watchdog;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Watchdog> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for WatchdogMut<'_> {
  type Msg = Watchdog;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Watchdog> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for WatchdogView<'_> {
  type Msg = Watchdog;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Watchdog> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for WatchdogMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod watchdog {// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__bootstrap__v3__Watchdog__WatchdogAction_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct WatchdogAction {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<WatchdogAction>
}

impl ::protobuf::Message for WatchdogAction {
  type MessageView<'msg> = WatchdogActionView<'msg>;
  type MessageMut<'msg> = WatchdogActionMut<'msg>;
}

impl ::std::default::Default for WatchdogAction {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for WatchdogAction {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `WatchdogAction` is `Sync` because it does not implement interior mutability.
//    Neither does `WatchdogActionMut`.
unsafe impl ::std::marker::Sync for WatchdogAction {}

// SAFETY:
// - `WatchdogAction` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for WatchdogAction {}

impl ::protobuf::Proxied for WatchdogAction {
  type View<'msg> = WatchdogActionView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for WatchdogAction {}

impl ::protobuf::MutProxied for WatchdogAction {
  type Mut<'msg> = WatchdogActionMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct WatchdogActionView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, WatchdogAction>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for WatchdogActionView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for WatchdogActionView<'msg> {
  type Message = WatchdogAction;
}

impl ::std::fmt::Debug for WatchdogActionView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for WatchdogActionView<'_> {
  fn default() -> WatchdogActionView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, WatchdogAction>> for WatchdogActionView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, WatchdogAction>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> WatchdogActionView<'msg> {

  pub fn to_owned(&self) -> WatchdogAction {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // config: optional message envoy.config.core.v3.TypedExtensionConfig
  pub fn has_config(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn config_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'msg>> {
    self.has_config().then(|| self.config())
  }
  pub fn config(self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView::default())
  }

  // event: optional enum envoy.config.bootstrap.v3.Watchdog.WatchdogAction.WatchdogEvent
  pub fn event(self) -> super::super::watchdog::watchdog_action::WatchdogEvent {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        1, (super::super::watchdog::watchdog_action::WatchdogEvent::Unknown).into()
      ).try_into().unwrap()
    }
  }

}

// SAFETY:
// - `WatchdogActionView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for WatchdogActionView<'_> {}

// SAFETY:
// - `WatchdogActionView` is `Send` because while its alive a `WatchdogActionMut` cannot.
// - `WatchdogActionView` does not use thread-local data.
unsafe impl ::std::marker::Send for WatchdogActionView<'_> {}

impl<'msg> ::protobuf::AsView for WatchdogActionView<'msg> {
  type Proxied = WatchdogAction;
  fn as_view(&self) -> ::protobuf::View<'msg, WatchdogAction> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for WatchdogActionView<'msg> {
  fn into_view<'shorter>(self) -> WatchdogActionView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<WatchdogAction> for WatchdogActionView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> WatchdogAction {
    let mut dst = WatchdogAction::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<WatchdogAction> for WatchdogActionMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> WatchdogAction {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for WatchdogAction {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for WatchdogActionView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for WatchdogActionMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct WatchdogActionMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, WatchdogAction>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for WatchdogActionMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for WatchdogActionMut<'msg> {
  type Message = WatchdogAction;
}

impl ::std::fmt::Debug for WatchdogActionMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, WatchdogAction>> for WatchdogActionMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, WatchdogAction>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> WatchdogActionMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, WatchdogAction> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> WatchdogAction {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // config: optional message envoy.config.core.v3.TypedExtensionConfig
  pub fn has_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn config_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'_>> {
    self.has_config().then(|| self.config())
  }
  pub fn config(&self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView::default())
  }
  pub fn config_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         0, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_config(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // event: optional enum envoy.config.bootstrap.v3.Watchdog.WatchdogAction.WatchdogEvent
  pub fn event(&self) -> super::super::watchdog::watchdog_action::WatchdogEvent {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        1, (super::super::watchdog::watchdog_action::WatchdogEvent::Unknown).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_event(&mut self, val: super::super::watchdog::watchdog_action::WatchdogEvent) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        1, val.into()
      )
    }
  }

}

// SAFETY:
// - `WatchdogActionMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for WatchdogActionMut<'_> {}

// SAFETY:
// - `WatchdogActionMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for WatchdogActionMut<'_> {}

impl<'msg> ::protobuf::AsView for WatchdogActionMut<'msg> {
  type Proxied = WatchdogAction;
  fn as_view(&self) -> ::protobuf::View<'_, WatchdogAction> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for WatchdogActionMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, WatchdogAction>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for WatchdogActionMut<'msg> {
  type MutProxied = WatchdogAction;
  fn as_mut(&mut self) -> WatchdogActionMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for WatchdogActionMut<'msg> {
  fn into_mut<'shorter>(self) -> WatchdogActionMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl WatchdogAction {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, WatchdogAction> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> WatchdogActionView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> WatchdogActionMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // config: optional message envoy.config.core.v3.TypedExtensionConfig
  pub fn has_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn config_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'_>> {
    self.has_config().then(|| self.config())
  }
  pub fn config(&self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView::default())
  }
  pub fn config_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         0, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_config(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // event: optional enum envoy.config.bootstrap.v3.Watchdog.WatchdogAction.WatchdogEvent
  pub fn event(&self) -> super::super::watchdog::watchdog_action::WatchdogEvent {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        1, (super::super::watchdog::watchdog_action::WatchdogEvent::Unknown).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_event(&mut self, val: super::super::watchdog::watchdog_action::WatchdogEvent) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        1, val.into()
      )
    }
  }

}  // impl WatchdogAction

impl ::std::ops::Drop for WatchdogAction {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for WatchdogAction {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for WatchdogAction {
  type Proxied = Self;
  fn as_view(&self) -> WatchdogActionView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for WatchdogAction {
  type MutProxied = Self;
  fn as_mut(&mut self) -> WatchdogActionMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for WatchdogAction {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::watchdog::envoy__config__bootstrap__v3__Watchdog__WatchdogAction_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$3.P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::watchdog::envoy__config__bootstrap__v3__Watchdog__WatchdogAction_msg_init.0, &[<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::watchdog::envoy__config__bootstrap__v3__Watchdog__WatchdogAction_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for WatchdogAction {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for WatchdogAction {
  type Msg = WatchdogAction;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<WatchdogAction> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for WatchdogAction {
  type Msg = WatchdogAction;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<WatchdogAction> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for WatchdogActionMut<'_> {
  type Msg = WatchdogAction;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<WatchdogAction> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for WatchdogActionMut<'_> {
  type Msg = WatchdogAction;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<WatchdogAction> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for WatchdogActionView<'_> {
  type Msg = WatchdogAction;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<WatchdogAction> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for WatchdogActionMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod watchdog_action {
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct WatchdogEvent(i32);

#[allow(non_upper_case_globals)]
impl WatchdogEvent {
  pub const Unknown: WatchdogEvent = WatchdogEvent(0);
  pub const Kill: WatchdogEvent = WatchdogEvent(1);
  pub const Multikill: WatchdogEvent = WatchdogEvent(2);
  pub const Megamiss: WatchdogEvent = WatchdogEvent(3);
  pub const Miss: WatchdogEvent = WatchdogEvent(4);

  fn constant_name(&self) -> ::std::option::Option<&'static str> {
    #[allow(unreachable_patterns)] // In the case of aliases, just emit them all and let the first one match.
    Some(match self.0 {
      0 => "Unknown",
      1 => "Kill",
      2 => "Multikill",
      3 => "Megamiss",
      4 => "Miss",
      _ => return None
    })
  }
}

impl ::std::convert::From<WatchdogEvent> for i32 {
  fn from(val: WatchdogEvent) -> i32 {
    val.0
  }
}

impl ::std::convert::From<i32> for WatchdogEvent {
  fn from(val: i32) -> WatchdogEvent {
    Self(val)
  }
}

impl ::std::default::Default for WatchdogEvent {
  fn default() -> Self {
    Self(0)
  }
}

impl ::std::fmt::Debug for WatchdogEvent {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    if let Some(constant_name) = self.constant_name() {
      write!(f, "WatchdogEvent::{}", constant_name)
    } else {
      write!(f, "WatchdogEvent::from({})", self.0)
    }
  }
}

impl ::protobuf::IntoProxied<i32> for WatchdogEvent {
  fn into_proxied(self, _: ::protobuf::__internal::Private) -> i32 {
    self.0
  }
}

impl ::protobuf::__internal::SealedInternal for WatchdogEvent {}

impl ::protobuf::Proxied for WatchdogEvent {
  type View<'a> = WatchdogEvent;
}

impl ::protobuf::AsView for WatchdogEvent {
  type Proxied = WatchdogEvent;

  fn as_view(&self) -> WatchdogEvent {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for WatchdogEvent {
  fn into_view<'shorter>(self) -> WatchdogEvent where 'msg: 'shorter {
    self
  }
}

// SAFETY: this is an enum type
unsafe impl ::protobuf::__internal::Enum for WatchdogEvent {
  const NAME: &'static str = "WatchdogEvent";

  fn is_known(value: i32) -> bool {
    matches!(value, 0|1|2|3|4)
  }
}

impl ::protobuf::__internal::EntityType for WatchdogEvent {
    type Tag = ::protobuf::__internal::entity_tag::EnumTag;
}


}  // pub mod watchdog_action


}  // pub mod watchdog


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__bootstrap__v3__FatalAction_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct FatalAction {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<FatalAction>
}

impl ::protobuf::Message for FatalAction {
  type MessageView<'msg> = FatalActionView<'msg>;
  type MessageMut<'msg> = FatalActionMut<'msg>;
}

impl ::std::default::Default for FatalAction {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for FatalAction {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `FatalAction` is `Sync` because it does not implement interior mutability.
//    Neither does `FatalActionMut`.
unsafe impl ::std::marker::Sync for FatalAction {}

// SAFETY:
// - `FatalAction` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for FatalAction {}

impl ::protobuf::Proxied for FatalAction {
  type View<'msg> = FatalActionView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for FatalAction {}

impl ::protobuf::MutProxied for FatalAction {
  type Mut<'msg> = FatalActionMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct FatalActionView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, FatalAction>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for FatalActionView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for FatalActionView<'msg> {
  type Message = FatalAction;
}

impl ::std::fmt::Debug for FatalActionView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for FatalActionView<'_> {
  fn default() -> FatalActionView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, FatalAction>> for FatalActionView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, FatalAction>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> FatalActionView<'msg> {

  pub fn to_owned(&self) -> FatalAction {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // config: optional message envoy.config.core.v3.TypedExtensionConfig
  pub fn has_config(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn config_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'msg>> {
    self.has_config().then(|| self.config())
  }
  pub fn config(self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView::default())
  }

}

// SAFETY:
// - `FatalActionView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for FatalActionView<'_> {}

// SAFETY:
// - `FatalActionView` is `Send` because while its alive a `FatalActionMut` cannot.
// - `FatalActionView` does not use thread-local data.
unsafe impl ::std::marker::Send for FatalActionView<'_> {}

impl<'msg> ::protobuf::AsView for FatalActionView<'msg> {
  type Proxied = FatalAction;
  fn as_view(&self) -> ::protobuf::View<'msg, FatalAction> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for FatalActionView<'msg> {
  fn into_view<'shorter>(self) -> FatalActionView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<FatalAction> for FatalActionView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> FatalAction {
    let mut dst = FatalAction::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<FatalAction> for FatalActionMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> FatalAction {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for FatalAction {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for FatalActionView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for FatalActionMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct FatalActionMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, FatalAction>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for FatalActionMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for FatalActionMut<'msg> {
  type Message = FatalAction;
}

impl ::std::fmt::Debug for FatalActionMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, FatalAction>> for FatalActionMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, FatalAction>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> FatalActionMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, FatalAction> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> FatalAction {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // config: optional message envoy.config.core.v3.TypedExtensionConfig
  pub fn has_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn config_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'_>> {
    self.has_config().then(|| self.config())
  }
  pub fn config(&self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView::default())
  }
  pub fn config_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         0, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_config(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

}

// SAFETY:
// - `FatalActionMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for FatalActionMut<'_> {}

// SAFETY:
// - `FatalActionMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for FatalActionMut<'_> {}

impl<'msg> ::protobuf::AsView for FatalActionMut<'msg> {
  type Proxied = FatalAction;
  fn as_view(&self) -> ::protobuf::View<'_, FatalAction> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for FatalActionMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, FatalAction>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for FatalActionMut<'msg> {
  type MutProxied = FatalAction;
  fn as_mut(&mut self) -> FatalActionMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for FatalActionMut<'msg> {
  fn into_mut<'shorter>(self) -> FatalActionMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl FatalAction {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, FatalAction> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> FatalActionView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> FatalActionMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // config: optional message envoy.config.core.v3.TypedExtensionConfig
  pub fn has_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn config_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'_>> {
    self.has_config().then(|| self.config())
  }
  pub fn config(&self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigView::default())
  }
  pub fn config_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfigMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         0, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_config(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

}  // impl FatalAction

impl ::std::ops::Drop for FatalAction {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for FatalAction {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for FatalAction {
  type Proxied = Self;
  fn as_view(&self) -> FatalActionView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for FatalAction {
  type MutProxied = Self;
  fn as_mut(&mut self) -> FatalActionMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for FatalAction {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__bootstrap__v3__FatalAction_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$3");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__bootstrap__v3__FatalAction_msg_init.0, &[<crate::xds::generated::envoy::config::core::v3::extension::TypedExtensionConfig as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__bootstrap__v3__FatalAction_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for FatalAction {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for FatalAction {
  type Msg = FatalAction;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<FatalAction> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for FatalAction {
  type Msg = FatalAction;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<FatalAction> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for FatalActionMut<'_> {
  type Msg = FatalAction;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<FatalAction> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for FatalActionMut<'_> {
  type Msg = FatalAction;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<FatalAction> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for FatalActionView<'_> {
  type Msg = FatalAction;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<FatalAction> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for FatalActionMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__bootstrap__v3__Runtime_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct Runtime {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Runtime>
}

impl ::protobuf::Message for Runtime {
  type MessageView<'msg> = RuntimeView<'msg>;
  type MessageMut<'msg> = RuntimeMut<'msg>;
}

impl ::std::default::Default for Runtime {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for Runtime {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `Runtime` is `Sync` because it does not implement interior mutability.
//    Neither does `RuntimeMut`.
unsafe impl ::std::marker::Sync for Runtime {}

// SAFETY:
// - `Runtime` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for Runtime {}

impl ::protobuf::Proxied for Runtime {
  type View<'msg> = RuntimeView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Runtime {}

impl ::protobuf::MutProxied for Runtime {
  type Mut<'msg> = RuntimeMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct RuntimeView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Runtime>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for RuntimeView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for RuntimeView<'msg> {
  type Message = Runtime;
}

impl ::std::fmt::Debug for RuntimeView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for RuntimeView<'_> {
  fn default() -> RuntimeView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Runtime>> for RuntimeView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Runtime>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> RuntimeView<'msg> {

  pub fn to_owned(&self) -> Runtime {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // symlink_root: optional string
  pub fn symlink_root(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // subdirectory: optional string
  pub fn subdirectory(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // override_subdirectory: optional string
  pub fn override_subdirectory(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // base: optional message google.protobuf.Struct
  pub fn has_base(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn base_opt(self) -> ::std::option::Option<::protobuf_well_known_types::StructView<'msg>> {
    self.has_base().then(|| self.base())
  }
  pub fn base(self) -> ::protobuf_well_known_types::StructView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::StructView::default())
  }

}

// SAFETY:
// - `RuntimeView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for RuntimeView<'_> {}

// SAFETY:
// - `RuntimeView` is `Send` because while its alive a `RuntimeMut` cannot.
// - `RuntimeView` does not use thread-local data.
unsafe impl ::std::marker::Send for RuntimeView<'_> {}

impl<'msg> ::protobuf::AsView for RuntimeView<'msg> {
  type Proxied = Runtime;
  fn as_view(&self) -> ::protobuf::View<'msg, Runtime> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for RuntimeView<'msg> {
  fn into_view<'shorter>(self) -> RuntimeView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Runtime> for RuntimeView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Runtime {
    let mut dst = Runtime::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Runtime> for RuntimeMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Runtime {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for Runtime {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for RuntimeView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for RuntimeMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct RuntimeMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Runtime>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for RuntimeMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for RuntimeMut<'msg> {
  type Message = Runtime;
}

impl ::std::fmt::Debug for RuntimeMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Runtime>> for RuntimeMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Runtime>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> RuntimeMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Runtime> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> Runtime {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // symlink_root: optional string
  pub fn symlink_root(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_symlink_root(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // subdirectory: optional string
  pub fn subdirectory(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_subdirectory(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // override_subdirectory: optional string
  pub fn override_subdirectory(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_override_subdirectory(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val);
    }
  }

  // base: optional message google.protobuf.Struct
  pub fn has_base(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_base(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn base_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::StructView<'_>> {
    self.has_base().then(|| self.base())
  }
  pub fn base(&self) -> ::protobuf_well_known_types::StructView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::StructView::default())
  }
  pub fn base_mut(&mut self) -> ::protobuf_well_known_types::StructMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         3, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_base(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Struct>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

}

// SAFETY:
// - `RuntimeMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for RuntimeMut<'_> {}

// SAFETY:
// - `RuntimeMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for RuntimeMut<'_> {}

impl<'msg> ::protobuf::AsView for RuntimeMut<'msg> {
  type Proxied = Runtime;
  fn as_view(&self) -> ::protobuf::View<'_, Runtime> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for RuntimeMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Runtime>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for RuntimeMut<'msg> {
  type MutProxied = Runtime;
  fn as_mut(&mut self) -> RuntimeMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for RuntimeMut<'msg> {
  fn into_mut<'shorter>(self) -> RuntimeMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Runtime {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Runtime> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> RuntimeView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> RuntimeMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // symlink_root: optional string
  pub fn symlink_root(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_symlink_root(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // subdirectory: optional string
  pub fn subdirectory(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_subdirectory(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // override_subdirectory: optional string
  pub fn override_subdirectory(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_override_subdirectory(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val);
    }
  }

  // base: optional message google.protobuf.Struct
  pub fn has_base(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_base(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn base_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::StructView<'_>> {
    self.has_base().then(|| self.base())
  }
  pub fn base(&self) -> ::protobuf_well_known_types::StructView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::StructView::default())
  }
  pub fn base_mut(&mut self) -> ::protobuf_well_known_types::StructMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         3, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_base(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Struct>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

}  // impl Runtime

impl ::std::ops::Drop for Runtime {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Runtime {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Runtime {
  type Proxied = Self;
  fn as_view(&self) -> RuntimeView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Runtime {
  type MutProxied = Self;
  fn as_mut(&mut self) -> RuntimeMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Runtime {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__bootstrap__v3__Runtime_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1X1X1X3");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__bootstrap__v3__Runtime_msg_init.0, &[<::protobuf_well_known_types::Struct as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__bootstrap__v3__Runtime_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Runtime {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Runtime {
  type Msg = Runtime;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Runtime> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Runtime {
  type Msg = Runtime;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Runtime> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for RuntimeMut<'_> {
  type Msg = Runtime;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Runtime> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RuntimeMut<'_> {
  type Msg = Runtime;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Runtime> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RuntimeView<'_> {
  type Msg = Runtime;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Runtime> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for RuntimeMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__bootstrap__v3__RuntimeLayer_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct RuntimeLayer {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<RuntimeLayer>
}

impl ::protobuf::Message for RuntimeLayer {
  type MessageView<'msg> = RuntimeLayerView<'msg>;
  type MessageMut<'msg> = RuntimeLayerMut<'msg>;
}

impl ::std::default::Default for RuntimeLayer {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for RuntimeLayer {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `RuntimeLayer` is `Sync` because it does not implement interior mutability.
//    Neither does `RuntimeLayerMut`.
unsafe impl ::std::marker::Sync for RuntimeLayer {}

// SAFETY:
// - `RuntimeLayer` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for RuntimeLayer {}

impl ::protobuf::Proxied for RuntimeLayer {
  type View<'msg> = RuntimeLayerView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for RuntimeLayer {}

impl ::protobuf::MutProxied for RuntimeLayer {
  type Mut<'msg> = RuntimeLayerMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct RuntimeLayerView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, RuntimeLayer>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for RuntimeLayerView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for RuntimeLayerView<'msg> {
  type Message = RuntimeLayer;
}

impl ::std::fmt::Debug for RuntimeLayerView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for RuntimeLayerView<'_> {
  fn default() -> RuntimeLayerView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, RuntimeLayer>> for RuntimeLayerView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, RuntimeLayer>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> RuntimeLayerView<'msg> {

  pub fn to_owned(&self) -> RuntimeLayer {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // name: optional string
  pub fn name(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // static_layer: optional message google.protobuf.Struct
  pub fn has_static_layer(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn static_layer_opt(self) -> ::std::option::Option<::protobuf_well_known_types::StructView<'msg>> {
    self.has_static_layer().then(|| self.static_layer())
  }
  pub fn static_layer(self) -> ::protobuf_well_known_types::StructView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::StructView::default())
  }

  // disk_layer: optional message envoy.config.bootstrap.v3.RuntimeLayer.DiskLayer
  pub fn has_disk_layer(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn disk_layer_opt(self) -> ::std::option::Option<super::runtime_layer::DiskLayerView<'msg>> {
    self.has_disk_layer().then(|| self.disk_layer())
  }
  pub fn disk_layer(self) -> super::runtime_layer::DiskLayerView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::runtime_layer::DiskLayerView::default())
  }

  // admin_layer: optional message envoy.config.bootstrap.v3.RuntimeLayer.AdminLayer
  pub fn has_admin_layer(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn admin_layer_opt(self) -> ::std::option::Option<super::runtime_layer::AdminLayerView<'msg>> {
    self.has_admin_layer().then(|| self.admin_layer())
  }
  pub fn admin_layer(self) -> super::runtime_layer::AdminLayerView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::runtime_layer::AdminLayerView::default())
  }

  // rtds_layer: optional message envoy.config.bootstrap.v3.RuntimeLayer.RtdsLayer
  pub fn has_rtds_layer(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn rtds_layer_opt(self) -> ::std::option::Option<super::runtime_layer::RtdsLayerView<'msg>> {
    self.has_rtds_layer().then(|| self.rtds_layer())
  }
  pub fn rtds_layer(self) -> super::runtime_layer::RtdsLayerView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::runtime_layer::RtdsLayerView::default())
  }

  pub fn layer_specifier(self) -> super::runtime_layer::LayerSpecifierOneof<'msg> {
    match self.layer_specifier_case() {
      super::runtime_layer::LayerSpecifierCase::StaticLayer =>
          super::runtime_layer::LayerSpecifierOneof::StaticLayer(self.static_layer()),
      super::runtime_layer::LayerSpecifierCase::DiskLayer =>
          super::runtime_layer::LayerSpecifierOneof::DiskLayer(self.disk_layer()),
      super::runtime_layer::LayerSpecifierCase::AdminLayer =>
          super::runtime_layer::LayerSpecifierOneof::AdminLayer(self.admin_layer()),
      super::runtime_layer::LayerSpecifierCase::RtdsLayer =>
          super::runtime_layer::LayerSpecifierOneof::RtdsLayer(self.rtds_layer()),
      _ => super::runtime_layer::LayerSpecifierOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn layer_specifier_case(self) -> super::runtime_layer::LayerSpecifierCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(1);
      super::runtime_layer::LayerSpecifierCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `RuntimeLayerView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for RuntimeLayerView<'_> {}

// SAFETY:
// - `RuntimeLayerView` is `Send` because while its alive a `RuntimeLayerMut` cannot.
// - `RuntimeLayerView` does not use thread-local data.
unsafe impl ::std::marker::Send for RuntimeLayerView<'_> {}

impl<'msg> ::protobuf::AsView for RuntimeLayerView<'msg> {
  type Proxied = RuntimeLayer;
  fn as_view(&self) -> ::protobuf::View<'msg, RuntimeLayer> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for RuntimeLayerView<'msg> {
  fn into_view<'shorter>(self) -> RuntimeLayerView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<RuntimeLayer> for RuntimeLayerView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> RuntimeLayer {
    let mut dst = RuntimeLayer::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<RuntimeLayer> for RuntimeLayerMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> RuntimeLayer {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for RuntimeLayer {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for RuntimeLayerView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for RuntimeLayerMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct RuntimeLayerMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, RuntimeLayer>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for RuntimeLayerMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for RuntimeLayerMut<'msg> {
  type Message = RuntimeLayer;
}

impl ::std::fmt::Debug for RuntimeLayerMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, RuntimeLayer>> for RuntimeLayerMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, RuntimeLayer>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> RuntimeLayerMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, RuntimeLayer> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> RuntimeLayer {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // name: optional string
  pub fn name(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_name(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // static_layer: optional message google.protobuf.Struct
  pub fn has_static_layer(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_static_layer(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn static_layer_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::StructView<'_>> {
    self.has_static_layer().then(|| self.static_layer())
  }
  pub fn static_layer(&self) -> ::protobuf_well_known_types::StructView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::StructView::default())
  }
  pub fn static_layer_mut(&mut self) -> ::protobuf_well_known_types::StructMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         1, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_static_layer(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Struct>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // disk_layer: optional message envoy.config.bootstrap.v3.RuntimeLayer.DiskLayer
  pub fn has_disk_layer(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_disk_layer(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn disk_layer_opt(&self) -> ::std::option::Option<super::runtime_layer::DiskLayerView<'_>> {
    self.has_disk_layer().then(|| self.disk_layer())
  }
  pub fn disk_layer(&self) -> super::runtime_layer::DiskLayerView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::runtime_layer::DiskLayerView::default())
  }
  pub fn disk_layer_mut(&mut self) -> super::runtime_layer::DiskLayerMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         2, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_disk_layer(&mut self,
    val: impl ::protobuf::IntoProxied<super::runtime_layer::DiskLayer>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // admin_layer: optional message envoy.config.bootstrap.v3.RuntimeLayer.AdminLayer
  pub fn has_admin_layer(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_admin_layer(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn admin_layer_opt(&self) -> ::std::option::Option<super::runtime_layer::AdminLayerView<'_>> {
    self.has_admin_layer().then(|| self.admin_layer())
  }
  pub fn admin_layer(&self) -> super::runtime_layer::AdminLayerView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::runtime_layer::AdminLayerView::default())
  }
  pub fn admin_layer_mut(&mut self) -> super::runtime_layer::AdminLayerMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         3, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_admin_layer(&mut self,
    val: impl ::protobuf::IntoProxied<super::runtime_layer::AdminLayer>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // rtds_layer: optional message envoy.config.bootstrap.v3.RuntimeLayer.RtdsLayer
  pub fn has_rtds_layer(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_rtds_layer(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn rtds_layer_opt(&self) -> ::std::option::Option<super::runtime_layer::RtdsLayerView<'_>> {
    self.has_rtds_layer().then(|| self.rtds_layer())
  }
  pub fn rtds_layer(&self) -> super::runtime_layer::RtdsLayerView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::runtime_layer::RtdsLayerView::default())
  }
  pub fn rtds_layer_mut(&mut self) -> super::runtime_layer::RtdsLayerMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         4, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_rtds_layer(&mut self,
    val: impl ::protobuf::IntoProxied<super::runtime_layer::RtdsLayer>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val
      );
    }
  }

  pub fn layer_specifier(&self) -> super::runtime_layer::LayerSpecifierOneof<'_> {
    match &self.layer_specifier_case() {
      super::runtime_layer::LayerSpecifierCase::StaticLayer =>
          super::runtime_layer::LayerSpecifierOneof::StaticLayer(self.static_layer()),
      super::runtime_layer::LayerSpecifierCase::DiskLayer =>
          super::runtime_layer::LayerSpecifierOneof::DiskLayer(self.disk_layer()),
      super::runtime_layer::LayerSpecifierCase::AdminLayer =>
          super::runtime_layer::LayerSpecifierOneof::AdminLayer(self.admin_layer()),
      super::runtime_layer::LayerSpecifierCase::RtdsLayer =>
          super::runtime_layer::LayerSpecifierOneof::RtdsLayer(self.rtds_layer()),
      _ => super::runtime_layer::LayerSpecifierOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn layer_specifier_case(&self) -> super::runtime_layer::LayerSpecifierCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(1);
      super::runtime_layer::LayerSpecifierCase::try_from(field_num).unwrap_unchecked()
    }
  }
}

// SAFETY:
// - `RuntimeLayerMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for RuntimeLayerMut<'_> {}

// SAFETY:
// - `RuntimeLayerMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for RuntimeLayerMut<'_> {}

impl<'msg> ::protobuf::AsView for RuntimeLayerMut<'msg> {
  type Proxied = RuntimeLayer;
  fn as_view(&self) -> ::protobuf::View<'_, RuntimeLayer> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for RuntimeLayerMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, RuntimeLayer>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for RuntimeLayerMut<'msg> {
  type MutProxied = RuntimeLayer;
  fn as_mut(&mut self) -> RuntimeLayerMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for RuntimeLayerMut<'msg> {
  fn into_mut<'shorter>(self) -> RuntimeLayerMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl RuntimeLayer {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, RuntimeLayer> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> RuntimeLayerView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> RuntimeLayerMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // name: optional string
  pub fn name(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_name(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // static_layer: optional message google.protobuf.Struct
  pub fn has_static_layer(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_static_layer(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn static_layer_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::StructView<'_>> {
    self.has_static_layer().then(|| self.static_layer())
  }
  pub fn static_layer(&self) -> ::protobuf_well_known_types::StructView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::StructView::default())
  }
  pub fn static_layer_mut(&mut self) -> ::protobuf_well_known_types::StructMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         1, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_static_layer(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Struct>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // disk_layer: optional message envoy.config.bootstrap.v3.RuntimeLayer.DiskLayer
  pub fn has_disk_layer(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_disk_layer(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn disk_layer_opt(&self) -> ::std::option::Option<super::runtime_layer::DiskLayerView<'_>> {
    self.has_disk_layer().then(|| self.disk_layer())
  }
  pub fn disk_layer(&self) -> super::runtime_layer::DiskLayerView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::runtime_layer::DiskLayerView::default())
  }
  pub fn disk_layer_mut(&mut self) -> super::runtime_layer::DiskLayerMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         2, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_disk_layer(&mut self,
    val: impl ::protobuf::IntoProxied<super::runtime_layer::DiskLayer>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // admin_layer: optional message envoy.config.bootstrap.v3.RuntimeLayer.AdminLayer
  pub fn has_admin_layer(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_admin_layer(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn admin_layer_opt(&self) -> ::std::option::Option<super::runtime_layer::AdminLayerView<'_>> {
    self.has_admin_layer().then(|| self.admin_layer())
  }
  pub fn admin_layer(&self) -> super::runtime_layer::AdminLayerView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::runtime_layer::AdminLayerView::default())
  }
  pub fn admin_layer_mut(&mut self) -> super::runtime_layer::AdminLayerMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         3, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_admin_layer(&mut self,
    val: impl ::protobuf::IntoProxied<super::runtime_layer::AdminLayer>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // rtds_layer: optional message envoy.config.bootstrap.v3.RuntimeLayer.RtdsLayer
  pub fn has_rtds_layer(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_rtds_layer(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn rtds_layer_opt(&self) -> ::std::option::Option<super::runtime_layer::RtdsLayerView<'_>> {
    self.has_rtds_layer().then(|| self.rtds_layer())
  }
  pub fn rtds_layer(&self) -> super::runtime_layer::RtdsLayerView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::runtime_layer::RtdsLayerView::default())
  }
  pub fn rtds_layer_mut(&mut self) -> super::runtime_layer::RtdsLayerMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         4, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_rtds_layer(&mut self,
    val: impl ::protobuf::IntoProxied<super::runtime_layer::RtdsLayer>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val
      );
    }
  }

  pub fn layer_specifier(&self) -> super::runtime_layer::LayerSpecifierOneof<'_> {
    match &self.layer_specifier_case() {
      super::runtime_layer::LayerSpecifierCase::StaticLayer =>
          super::runtime_layer::LayerSpecifierOneof::StaticLayer(self.static_layer()),
      super::runtime_layer::LayerSpecifierCase::DiskLayer =>
          super::runtime_layer::LayerSpecifierOneof::DiskLayer(self.disk_layer()),
      super::runtime_layer::LayerSpecifierCase::AdminLayer =>
          super::runtime_layer::LayerSpecifierOneof::AdminLayer(self.admin_layer()),
      super::runtime_layer::LayerSpecifierCase::RtdsLayer =>
          super::runtime_layer::LayerSpecifierOneof::RtdsLayer(self.rtds_layer()),
      _ => super::runtime_layer::LayerSpecifierOneof::not_set(std::marker::PhantomData)
    }
  }

  pub fn layer_specifier_case(&self) -> super::runtime_layer::LayerSpecifierCase {
    unsafe {
      let field_num = <Self as ::protobuf::__internal::runtime::UpbGetMessagePtr>::get_ptr(
          &self, ::protobuf::__internal::Private)
          .which_oneof_field_number_by_index(1);
      super::runtime_layer::LayerSpecifierCase::try_from(field_num).unwrap_unchecked()
    }
  }
}  // impl RuntimeLayer

impl ::std::ops::Drop for RuntimeLayer {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for RuntimeLayer {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for RuntimeLayer {
  type Proxied = Self;
  fn as_view(&self) -> RuntimeLayerView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for RuntimeLayer {
  type MutProxied = Self;
  fn as_mut(&mut self) -> RuntimeLayerMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for RuntimeLayer {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__bootstrap__v3__RuntimeLayer_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1X3333^#|$|%|&");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__bootstrap__v3__RuntimeLayer_msg_init.0, &[<::protobuf_well_known_types::Struct as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::runtime_layer::DiskLayer as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::runtime_layer::AdminLayer as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::runtime_layer::RtdsLayer as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__bootstrap__v3__RuntimeLayer_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for RuntimeLayer {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for RuntimeLayer {
  type Msg = RuntimeLayer;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RuntimeLayer> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RuntimeLayer {
  type Msg = RuntimeLayer;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RuntimeLayer> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for RuntimeLayerMut<'_> {
  type Msg = RuntimeLayer;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RuntimeLayer> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RuntimeLayerMut<'_> {
  type Msg = RuntimeLayer;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RuntimeLayer> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RuntimeLayerView<'_> {
  type Msg = RuntimeLayer;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RuntimeLayer> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for RuntimeLayerMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod runtime_layer {// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__bootstrap__v3__RuntimeLayer__DiskLayer_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct DiskLayer {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<DiskLayer>
}

impl ::protobuf::Message for DiskLayer {
  type MessageView<'msg> = DiskLayerView<'msg>;
  type MessageMut<'msg> = DiskLayerMut<'msg>;
}

impl ::std::default::Default for DiskLayer {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for DiskLayer {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `DiskLayer` is `Sync` because it does not implement interior mutability.
//    Neither does `DiskLayerMut`.
unsafe impl ::std::marker::Sync for DiskLayer {}

// SAFETY:
// - `DiskLayer` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for DiskLayer {}

impl ::protobuf::Proxied for DiskLayer {
  type View<'msg> = DiskLayerView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for DiskLayer {}

impl ::protobuf::MutProxied for DiskLayer {
  type Mut<'msg> = DiskLayerMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct DiskLayerView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, DiskLayer>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for DiskLayerView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for DiskLayerView<'msg> {
  type Message = DiskLayer;
}

impl ::std::fmt::Debug for DiskLayerView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for DiskLayerView<'_> {
  fn default() -> DiskLayerView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, DiskLayer>> for DiskLayerView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, DiskLayer>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> DiskLayerView<'msg> {

  pub fn to_owned(&self) -> DiskLayer {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // symlink_root: optional string
  pub fn symlink_root(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // subdirectory: optional string
  pub fn subdirectory(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // append_service_cluster: optional bool
  pub fn append_service_cluster(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        1, (false).into()
      ).try_into().unwrap()
    }
  }

}

// SAFETY:
// - `DiskLayerView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for DiskLayerView<'_> {}

// SAFETY:
// - `DiskLayerView` is `Send` because while its alive a `DiskLayerMut` cannot.
// - `DiskLayerView` does not use thread-local data.
unsafe impl ::std::marker::Send for DiskLayerView<'_> {}

impl<'msg> ::protobuf::AsView for DiskLayerView<'msg> {
  type Proxied = DiskLayer;
  fn as_view(&self) -> ::protobuf::View<'msg, DiskLayer> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for DiskLayerView<'msg> {
  fn into_view<'shorter>(self) -> DiskLayerView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<DiskLayer> for DiskLayerView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> DiskLayer {
    let mut dst = DiskLayer::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<DiskLayer> for DiskLayerMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> DiskLayer {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for DiskLayer {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for DiskLayerView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for DiskLayerMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct DiskLayerMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, DiskLayer>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for DiskLayerMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for DiskLayerMut<'msg> {
  type Message = DiskLayer;
}

impl ::std::fmt::Debug for DiskLayerMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, DiskLayer>> for DiskLayerMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, DiskLayer>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> DiskLayerMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, DiskLayer> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> DiskLayer {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // symlink_root: optional string
  pub fn symlink_root(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_symlink_root(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // subdirectory: optional string
  pub fn subdirectory(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_subdirectory(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val);
    }
  }

  // append_service_cluster: optional bool
  pub fn append_service_cluster(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        1, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_append_service_cluster(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        1, val.into()
      )
    }
  }

}

// SAFETY:
// - `DiskLayerMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for DiskLayerMut<'_> {}

// SAFETY:
// - `DiskLayerMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for DiskLayerMut<'_> {}

impl<'msg> ::protobuf::AsView for DiskLayerMut<'msg> {
  type Proxied = DiskLayer;
  fn as_view(&self) -> ::protobuf::View<'_, DiskLayer> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for DiskLayerMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, DiskLayer>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for DiskLayerMut<'msg> {
  type MutProxied = DiskLayer;
  fn as_mut(&mut self) -> DiskLayerMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for DiskLayerMut<'msg> {
  fn into_mut<'shorter>(self) -> DiskLayerMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl DiskLayer {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, DiskLayer> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> DiskLayerView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> DiskLayerMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // symlink_root: optional string
  pub fn symlink_root(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_symlink_root(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // subdirectory: optional string
  pub fn subdirectory(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_subdirectory(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val);
    }
  }

  // append_service_cluster: optional bool
  pub fn append_service_cluster(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        1, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_append_service_cluster(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        1, val.into()
      )
    }
  }

}  // impl DiskLayer

impl ::std::ops::Drop for DiskLayer {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for DiskLayer {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for DiskLayer {
  type Proxied = Self;
  fn as_view(&self) -> DiskLayerView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for DiskLayer {
  type MutProxied = Self;
  fn as_mut(&mut self) -> DiskLayerMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for DiskLayer {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::runtime_layer::envoy__config__bootstrap__v3__RuntimeLayer__DiskLayer_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1X/P1X");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::runtime_layer::envoy__config__bootstrap__v3__RuntimeLayer__DiskLayer_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::runtime_layer::envoy__config__bootstrap__v3__RuntimeLayer__DiskLayer_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for DiskLayer {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for DiskLayer {
  type Msg = DiskLayer;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DiskLayer> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DiskLayer {
  type Msg = DiskLayer;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DiskLayer> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for DiskLayerMut<'_> {
  type Msg = DiskLayer;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DiskLayer> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DiskLayerMut<'_> {
  type Msg = DiskLayer;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DiskLayer> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for DiskLayerView<'_> {
  type Msg = DiskLayer;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<DiskLayer> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for DiskLayerMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__bootstrap__v3__RuntimeLayer__AdminLayer_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct AdminLayer {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<AdminLayer>
}

impl ::protobuf::Message for AdminLayer {
  type MessageView<'msg> = AdminLayerView<'msg>;
  type MessageMut<'msg> = AdminLayerMut<'msg>;
}

impl ::std::default::Default for AdminLayer {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for AdminLayer {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `AdminLayer` is `Sync` because it does not implement interior mutability.
//    Neither does `AdminLayerMut`.
unsafe impl ::std::marker::Sync for AdminLayer {}

// SAFETY:
// - `AdminLayer` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for AdminLayer {}

impl ::protobuf::Proxied for AdminLayer {
  type View<'msg> = AdminLayerView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for AdminLayer {}

impl ::protobuf::MutProxied for AdminLayer {
  type Mut<'msg> = AdminLayerMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct AdminLayerView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, AdminLayer>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for AdminLayerView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for AdminLayerView<'msg> {
  type Message = AdminLayer;
}

impl ::std::fmt::Debug for AdminLayerView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for AdminLayerView<'_> {
  fn default() -> AdminLayerView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, AdminLayer>> for AdminLayerView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, AdminLayer>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> AdminLayerView<'msg> {

  pub fn to_owned(&self) -> AdminLayer {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

}

// SAFETY:
// - `AdminLayerView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for AdminLayerView<'_> {}

// SAFETY:
// - `AdminLayerView` is `Send` because while its alive a `AdminLayerMut` cannot.
// - `AdminLayerView` does not use thread-local data.
unsafe impl ::std::marker::Send for AdminLayerView<'_> {}

impl<'msg> ::protobuf::AsView for AdminLayerView<'msg> {
  type Proxied = AdminLayer;
  fn as_view(&self) -> ::protobuf::View<'msg, AdminLayer> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for AdminLayerView<'msg> {
  fn into_view<'shorter>(self) -> AdminLayerView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<AdminLayer> for AdminLayerView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> AdminLayer {
    let mut dst = AdminLayer::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<AdminLayer> for AdminLayerMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> AdminLayer {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for AdminLayer {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for AdminLayerView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for AdminLayerMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct AdminLayerMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, AdminLayer>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for AdminLayerMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for AdminLayerMut<'msg> {
  type Message = AdminLayer;
}

impl ::std::fmt::Debug for AdminLayerMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, AdminLayer>> for AdminLayerMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, AdminLayer>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> AdminLayerMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, AdminLayer> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> AdminLayer {
    ::protobuf::AsView::as_view(self).to_owned()
  }

}

// SAFETY:
// - `AdminLayerMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for AdminLayerMut<'_> {}

// SAFETY:
// - `AdminLayerMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for AdminLayerMut<'_> {}

impl<'msg> ::protobuf::AsView for AdminLayerMut<'msg> {
  type Proxied = AdminLayer;
  fn as_view(&self) -> ::protobuf::View<'_, AdminLayer> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for AdminLayerMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, AdminLayer>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for AdminLayerMut<'msg> {
  type MutProxied = AdminLayer;
  fn as_mut(&mut self) -> AdminLayerMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for AdminLayerMut<'msg> {
  fn into_mut<'shorter>(self) -> AdminLayerMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl AdminLayer {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, AdminLayer> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> AdminLayerView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> AdminLayerMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

}  // impl AdminLayer

impl ::std::ops::Drop for AdminLayer {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for AdminLayer {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for AdminLayer {
  type Proxied = Self;
  fn as_view(&self) -> AdminLayerView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for AdminLayer {
  type MutProxied = Self;
  fn as_mut(&mut self) -> AdminLayerMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for AdminLayer {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::runtime_layer::envoy__config__bootstrap__v3__RuntimeLayer__AdminLayer_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::runtime_layer::envoy__config__bootstrap__v3__RuntimeLayer__AdminLayer_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::runtime_layer::envoy__config__bootstrap__v3__RuntimeLayer__AdminLayer_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for AdminLayer {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for AdminLayer {
  type Msg = AdminLayer;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<AdminLayer> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for AdminLayer {
  type Msg = AdminLayer;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<AdminLayer> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for AdminLayerMut<'_> {
  type Msg = AdminLayer;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<AdminLayer> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for AdminLayerMut<'_> {
  type Msg = AdminLayer;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<AdminLayer> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for AdminLayerView<'_> {
  type Msg = AdminLayer;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<AdminLayer> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for AdminLayerMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__bootstrap__v3__RuntimeLayer__RtdsLayer_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct RtdsLayer {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<RtdsLayer>
}

impl ::protobuf::Message for RtdsLayer {
  type MessageView<'msg> = RtdsLayerView<'msg>;
  type MessageMut<'msg> = RtdsLayerMut<'msg>;
}

impl ::std::default::Default for RtdsLayer {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for RtdsLayer {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `RtdsLayer` is `Sync` because it does not implement interior mutability.
//    Neither does `RtdsLayerMut`.
unsafe impl ::std::marker::Sync for RtdsLayer {}

// SAFETY:
// - `RtdsLayer` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for RtdsLayer {}

impl ::protobuf::Proxied for RtdsLayer {
  type View<'msg> = RtdsLayerView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for RtdsLayer {}

impl ::protobuf::MutProxied for RtdsLayer {
  type Mut<'msg> = RtdsLayerMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct RtdsLayerView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, RtdsLayer>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for RtdsLayerView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for RtdsLayerView<'msg> {
  type Message = RtdsLayer;
}

impl ::std::fmt::Debug for RtdsLayerView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for RtdsLayerView<'_> {
  fn default() -> RtdsLayerView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, RtdsLayer>> for RtdsLayerView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, RtdsLayer>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> RtdsLayerView<'msg> {

  pub fn to_owned(&self) -> RtdsLayer {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // name: optional string
  pub fn name(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // rtds_config: optional message envoy.config.core.v3.ConfigSource
  pub fn has_rtds_config(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn rtds_config_opt(self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::config_source::ConfigSourceView<'msg>> {
    self.has_rtds_config().then(|| self.rtds_config())
  }
  pub fn rtds_config(self) -> crate::xds::generated::envoy::config::core::v3::config_source::ConfigSourceView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::config_source::ConfigSourceView::default())
  }

}

// SAFETY:
// - `RtdsLayerView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for RtdsLayerView<'_> {}

// SAFETY:
// - `RtdsLayerView` is `Send` because while its alive a `RtdsLayerMut` cannot.
// - `RtdsLayerView` does not use thread-local data.
unsafe impl ::std::marker::Send for RtdsLayerView<'_> {}

impl<'msg> ::protobuf::AsView for RtdsLayerView<'msg> {
  type Proxied = RtdsLayer;
  fn as_view(&self) -> ::protobuf::View<'msg, RtdsLayer> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for RtdsLayerView<'msg> {
  fn into_view<'shorter>(self) -> RtdsLayerView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<RtdsLayer> for RtdsLayerView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> RtdsLayer {
    let mut dst = RtdsLayer::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<RtdsLayer> for RtdsLayerMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> RtdsLayer {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for RtdsLayer {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for RtdsLayerView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for RtdsLayerMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct RtdsLayerMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, RtdsLayer>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for RtdsLayerMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for RtdsLayerMut<'msg> {
  type Message = RtdsLayer;
}

impl ::std::fmt::Debug for RtdsLayerMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, RtdsLayer>> for RtdsLayerMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, RtdsLayer>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> RtdsLayerMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, RtdsLayer> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> RtdsLayer {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // name: optional string
  pub fn name(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_name(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // rtds_config: optional message envoy.config.core.v3.ConfigSource
  pub fn has_rtds_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_rtds_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn rtds_config_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::config_source::ConfigSourceView<'_>> {
    self.has_rtds_config().then(|| self.rtds_config())
  }
  pub fn rtds_config(&self) -> crate::xds::generated::envoy::config::core::v3::config_source::ConfigSourceView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::config_source::ConfigSourceView::default())
  }
  pub fn rtds_config_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::config_source::ConfigSourceMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         1, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_rtds_config(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::config_source::ConfigSource>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

}

// SAFETY:
// - `RtdsLayerMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for RtdsLayerMut<'_> {}

// SAFETY:
// - `RtdsLayerMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for RtdsLayerMut<'_> {}

impl<'msg> ::protobuf::AsView for RtdsLayerMut<'msg> {
  type Proxied = RtdsLayer;
  fn as_view(&self) -> ::protobuf::View<'_, RtdsLayer> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for RtdsLayerMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, RtdsLayer>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for RtdsLayerMut<'msg> {
  type MutProxied = RtdsLayer;
  fn as_mut(&mut self) -> RtdsLayerMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for RtdsLayerMut<'msg> {
  fn into_mut<'shorter>(self) -> RtdsLayerMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl RtdsLayer {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, RtdsLayer> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> RtdsLayerView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> RtdsLayerMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // name: optional string
  pub fn name(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_name(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // rtds_config: optional message envoy.config.core.v3.ConfigSource
  pub fn has_rtds_config(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_rtds_config(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn rtds_config_opt(&self) -> ::std::option::Option<crate::xds::generated::envoy::config::core::v3::config_source::ConfigSourceView<'_>> {
    self.has_rtds_config().then(|| self.rtds_config())
  }
  pub fn rtds_config(&self) -> crate::xds::generated::envoy::config::core::v3::config_source::ConfigSourceView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(crate::xds::generated::envoy::config::core::v3::config_source::ConfigSourceView::default())
  }
  pub fn rtds_config_mut(&mut self) -> crate::xds::generated::envoy::config::core::v3::config_source::ConfigSourceMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         1, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_rtds_config(&mut self,
    val: impl ::protobuf::IntoProxied<crate::xds::generated::envoy::config::core::v3::config_source::ConfigSource>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

}  // impl RtdsLayer

impl ::std::ops::Drop for RtdsLayer {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for RtdsLayer {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for RtdsLayer {
  type Proxied = Self;
  fn as_view(&self) -> RtdsLayerView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for RtdsLayer {
  type MutProxied = Self;
  fn as_mut(&mut self) -> RtdsLayerMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for RtdsLayer {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::runtime_layer::envoy__config__bootstrap__v3__RuntimeLayer__RtdsLayer_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1X3");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::runtime_layer::envoy__config__bootstrap__v3__RuntimeLayer__RtdsLayer_msg_init.0, &[<crate::xds::generated::envoy::config::core::v3::config_source::ConfigSource as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::runtime_layer::envoy__config__bootstrap__v3__RuntimeLayer__RtdsLayer_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for RtdsLayer {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for RtdsLayer {
  type Msg = RtdsLayer;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RtdsLayer> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RtdsLayer {
  type Msg = RtdsLayer;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RtdsLayer> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for RtdsLayerMut<'_> {
  type Msg = RtdsLayer;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RtdsLayer> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RtdsLayerMut<'_> {
  type Msg = RtdsLayer;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RtdsLayer> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for RtdsLayerView<'_> {
  type Msg = RtdsLayer;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<RtdsLayer> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for RtdsLayerMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
#[repr(u32)]
pub enum LayerSpecifierOneof<'msg> {
  StaticLayer(::protobuf::View<'msg, ::protobuf_well_known_types::Struct>) = 2,
  DiskLayer(::protobuf::View<'msg, super::super::runtime_layer::DiskLayer>) = 3,
  AdminLayer(::protobuf::View<'msg, super::super::runtime_layer::AdminLayer>) = 4,
  RtdsLayer(::protobuf::View<'msg, super::super::runtime_layer::RtdsLayer>) = 5,

  not_set(std::marker::PhantomData<&'msg ()>) = 0
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
#[allow(dead_code)]
pub enum LayerSpecifierCase {
  StaticLayer = 2,
  DiskLayer = 3,
  AdminLayer = 4,
  RtdsLayer = 5,

  not_set = 0
}

impl LayerSpecifierCase {
  #[allow(dead_code)]
  pub(crate) fn try_from(v: u32) -> ::std::option::Option<LayerSpecifierCase> {
    match v {
      0 => Some(LayerSpecifierCase::not_set),
      2 => Some(LayerSpecifierCase::StaticLayer),
      3 => Some(LayerSpecifierCase::DiskLayer),
      4 => Some(LayerSpecifierCase::AdminLayer),
      5 => Some(LayerSpecifierCase::RtdsLayer),
      _ => None
    }
  }
}
}  // pub mod runtime_layer


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__bootstrap__v3__LayeredRuntime_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct LayeredRuntime {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<LayeredRuntime>
}

impl ::protobuf::Message for LayeredRuntime {
  type MessageView<'msg> = LayeredRuntimeView<'msg>;
  type MessageMut<'msg> = LayeredRuntimeMut<'msg>;
}

impl ::std::default::Default for LayeredRuntime {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for LayeredRuntime {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `LayeredRuntime` is `Sync` because it does not implement interior mutability.
//    Neither does `LayeredRuntimeMut`.
unsafe impl ::std::marker::Sync for LayeredRuntime {}

// SAFETY:
// - `LayeredRuntime` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for LayeredRuntime {}

impl ::protobuf::Proxied for LayeredRuntime {
  type View<'msg> = LayeredRuntimeView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for LayeredRuntime {}

impl ::protobuf::MutProxied for LayeredRuntime {
  type Mut<'msg> = LayeredRuntimeMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct LayeredRuntimeView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, LayeredRuntime>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for LayeredRuntimeView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for LayeredRuntimeView<'msg> {
  type Message = LayeredRuntime;
}

impl ::std::fmt::Debug for LayeredRuntimeView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for LayeredRuntimeView<'_> {
  fn default() -> LayeredRuntimeView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, LayeredRuntime>> for LayeredRuntimeView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, LayeredRuntime>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> LayeredRuntimeView<'msg> {

  pub fn to_owned(&self) -> LayeredRuntime {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // layers: repeated message envoy.config.bootstrap.v3.RuntimeLayer
  pub fn layers(self) -> ::protobuf::RepeatedView<'msg, super::RuntimeLayer> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::RuntimeLayer>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

}

// SAFETY:
// - `LayeredRuntimeView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for LayeredRuntimeView<'_> {}

// SAFETY:
// - `LayeredRuntimeView` is `Send` because while its alive a `LayeredRuntimeMut` cannot.
// - `LayeredRuntimeView` does not use thread-local data.
unsafe impl ::std::marker::Send for LayeredRuntimeView<'_> {}

impl<'msg> ::protobuf::AsView for LayeredRuntimeView<'msg> {
  type Proxied = LayeredRuntime;
  fn as_view(&self) -> ::protobuf::View<'msg, LayeredRuntime> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for LayeredRuntimeView<'msg> {
  fn into_view<'shorter>(self) -> LayeredRuntimeView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<LayeredRuntime> for LayeredRuntimeView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> LayeredRuntime {
    let mut dst = LayeredRuntime::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<LayeredRuntime> for LayeredRuntimeMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> LayeredRuntime {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for LayeredRuntime {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for LayeredRuntimeView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for LayeredRuntimeMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct LayeredRuntimeMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, LayeredRuntime>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for LayeredRuntimeMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for LayeredRuntimeMut<'msg> {
  type Message = LayeredRuntime;
}

impl ::std::fmt::Debug for LayeredRuntimeMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, LayeredRuntime>> for LayeredRuntimeMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, LayeredRuntime>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> LayeredRuntimeMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, LayeredRuntime> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> LayeredRuntime {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // layers: repeated message envoy.config.bootstrap.v3.RuntimeLayer
  pub fn layers(&self) -> ::protobuf::RepeatedView<'_, super::RuntimeLayer> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::RuntimeLayer>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn layers_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::RuntimeLayer> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        0,
        self.inner.arena()
      ).expect("alloc should not fail");
      ::protobuf::RepeatedMut::from_inner(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::InnerRepeatedMut::new(
          raw_array, self.inner.arena(),
        ),
      )
    }
  }
  pub fn set_layers(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::RuntimeLayer>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

}

// SAFETY:
// - `LayeredRuntimeMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for LayeredRuntimeMut<'_> {}

// SAFETY:
// - `LayeredRuntimeMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for LayeredRuntimeMut<'_> {}

impl<'msg> ::protobuf::AsView for LayeredRuntimeMut<'msg> {
  type Proxied = LayeredRuntime;
  fn as_view(&self) -> ::protobuf::View<'_, LayeredRuntime> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for LayeredRuntimeMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, LayeredRuntime>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for LayeredRuntimeMut<'msg> {
  type MutProxied = LayeredRuntime;
  fn as_mut(&mut self) -> LayeredRuntimeMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for LayeredRuntimeMut<'msg> {
  fn into_mut<'shorter>(self) -> LayeredRuntimeMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl LayeredRuntime {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, LayeredRuntime> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> LayeredRuntimeView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> LayeredRuntimeMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // layers: repeated message envoy.config.bootstrap.v3.RuntimeLayer
  pub fn layers(&self) -> ::protobuf::RepeatedView<'_, super::RuntimeLayer> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::RuntimeLayer>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn layers_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::RuntimeLayer> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        0,
        self.inner.arena()
      ).expect("alloc should not fail");
      ::protobuf::RepeatedMut::from_inner(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::InnerRepeatedMut::new(
          raw_array, self.inner.arena(),
        ),
      )
    }
  }
  pub fn set_layers(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::RuntimeLayer>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

}  // impl LayeredRuntime

impl ::std::ops::Drop for LayeredRuntime {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for LayeredRuntime {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for LayeredRuntime {
  type Proxied = Self;
  fn as_view(&self) -> LayeredRuntimeView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for LayeredRuntime {
  type MutProxied = Self;
  fn as_mut(&mut self) -> LayeredRuntimeMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for LayeredRuntime {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__bootstrap__v3__LayeredRuntime_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$G");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__bootstrap__v3__LayeredRuntime_msg_init.0, &[<super::RuntimeLayer as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__bootstrap__v3__LayeredRuntime_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for LayeredRuntime {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for LayeredRuntime {
  type Msg = LayeredRuntime;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<LayeredRuntime> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for LayeredRuntime {
  type Msg = LayeredRuntime;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<LayeredRuntime> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for LayeredRuntimeMut<'_> {
  type Msg = LayeredRuntime;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<LayeredRuntime> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for LayeredRuntimeMut<'_> {
  type Msg = LayeredRuntime;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<LayeredRuntime> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for LayeredRuntimeView<'_> {
  type Msg = LayeredRuntime;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<LayeredRuntime> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for LayeredRuntimeMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__bootstrap__v3__CustomInlineHeader_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct CustomInlineHeader {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<CustomInlineHeader>
}

impl ::protobuf::Message for CustomInlineHeader {
  type MessageView<'msg> = CustomInlineHeaderView<'msg>;
  type MessageMut<'msg> = CustomInlineHeaderMut<'msg>;
}

impl ::std::default::Default for CustomInlineHeader {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for CustomInlineHeader {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `CustomInlineHeader` is `Sync` because it does not implement interior mutability.
//    Neither does `CustomInlineHeaderMut`.
unsafe impl ::std::marker::Sync for CustomInlineHeader {}

// SAFETY:
// - `CustomInlineHeader` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for CustomInlineHeader {}

impl ::protobuf::Proxied for CustomInlineHeader {
  type View<'msg> = CustomInlineHeaderView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for CustomInlineHeader {}

impl ::protobuf::MutProxied for CustomInlineHeader {
  type Mut<'msg> = CustomInlineHeaderMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct CustomInlineHeaderView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, CustomInlineHeader>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for CustomInlineHeaderView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for CustomInlineHeaderView<'msg> {
  type Message = CustomInlineHeader;
}

impl ::std::fmt::Debug for CustomInlineHeaderView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for CustomInlineHeaderView<'_> {
  fn default() -> CustomInlineHeaderView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, CustomInlineHeader>> for CustomInlineHeaderView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, CustomInlineHeader>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> CustomInlineHeaderView<'msg> {

  pub fn to_owned(&self) -> CustomInlineHeader {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // inline_header_name: optional string
  pub fn inline_header_name(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // inline_header_type: optional enum envoy.config.bootstrap.v3.CustomInlineHeader.InlineHeaderType
  pub fn inline_header_type(self) -> super::custom_inline_header::InlineHeaderType {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        1, (super::custom_inline_header::InlineHeaderType::RequestHeader).into()
      ).try_into().unwrap()
    }
  }

}

// SAFETY:
// - `CustomInlineHeaderView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for CustomInlineHeaderView<'_> {}

// SAFETY:
// - `CustomInlineHeaderView` is `Send` because while its alive a `CustomInlineHeaderMut` cannot.
// - `CustomInlineHeaderView` does not use thread-local data.
unsafe impl ::std::marker::Send for CustomInlineHeaderView<'_> {}

impl<'msg> ::protobuf::AsView for CustomInlineHeaderView<'msg> {
  type Proxied = CustomInlineHeader;
  fn as_view(&self) -> ::protobuf::View<'msg, CustomInlineHeader> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for CustomInlineHeaderView<'msg> {
  fn into_view<'shorter>(self) -> CustomInlineHeaderView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<CustomInlineHeader> for CustomInlineHeaderView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> CustomInlineHeader {
    let mut dst = CustomInlineHeader::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<CustomInlineHeader> for CustomInlineHeaderMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> CustomInlineHeader {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for CustomInlineHeader {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for CustomInlineHeaderView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for CustomInlineHeaderMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct CustomInlineHeaderMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, CustomInlineHeader>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for CustomInlineHeaderMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for CustomInlineHeaderMut<'msg> {
  type Message = CustomInlineHeader;
}

impl ::std::fmt::Debug for CustomInlineHeaderMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, CustomInlineHeader>> for CustomInlineHeaderMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, CustomInlineHeader>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> CustomInlineHeaderMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, CustomInlineHeader> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> CustomInlineHeader {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // inline_header_name: optional string
  pub fn inline_header_name(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_inline_header_name(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // inline_header_type: optional enum envoy.config.bootstrap.v3.CustomInlineHeader.InlineHeaderType
  pub fn inline_header_type(&self) -> super::custom_inline_header::InlineHeaderType {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        1, (super::custom_inline_header::InlineHeaderType::RequestHeader).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_inline_header_type(&mut self, val: super::custom_inline_header::InlineHeaderType) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        1, val.into()
      )
    }
  }

}

// SAFETY:
// - `CustomInlineHeaderMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for CustomInlineHeaderMut<'_> {}

// SAFETY:
// - `CustomInlineHeaderMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for CustomInlineHeaderMut<'_> {}

impl<'msg> ::protobuf::AsView for CustomInlineHeaderMut<'msg> {
  type Proxied = CustomInlineHeader;
  fn as_view(&self) -> ::protobuf::View<'_, CustomInlineHeader> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for CustomInlineHeaderMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, CustomInlineHeader>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for CustomInlineHeaderMut<'msg> {
  type MutProxied = CustomInlineHeader;
  fn as_mut(&mut self) -> CustomInlineHeaderMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for CustomInlineHeaderMut<'msg> {
  fn into_mut<'shorter>(self) -> CustomInlineHeaderMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl CustomInlineHeader {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, CustomInlineHeader> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> CustomInlineHeaderView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> CustomInlineHeaderMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // inline_header_name: optional string
  pub fn inline_header_name(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_inline_header_name(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // inline_header_type: optional enum envoy.config.bootstrap.v3.CustomInlineHeader.InlineHeaderType
  pub fn inline_header_type(&self) -> super::custom_inline_header::InlineHeaderType {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        1, (super::custom_inline_header::InlineHeaderType::RequestHeader).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_inline_header_type(&mut self, val: super::custom_inline_header::InlineHeaderType) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        1, val.into()
      )
    }
  }

}  // impl CustomInlineHeader

impl ::std::ops::Drop for CustomInlineHeader {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for CustomInlineHeader {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for CustomInlineHeader {
  type Proxied = Self;
  fn as_view(&self) -> CustomInlineHeaderView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for CustomInlineHeader {
  type MutProxied = Self;
  fn as_mut(&mut self) -> CustomInlineHeaderMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for CustomInlineHeader {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__bootstrap__v3__CustomInlineHeader_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1X.P");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__bootstrap__v3__CustomInlineHeader_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__bootstrap__v3__CustomInlineHeader_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for CustomInlineHeader {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for CustomInlineHeader {
  type Msg = CustomInlineHeader;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CustomInlineHeader> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for CustomInlineHeader {
  type Msg = CustomInlineHeader;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CustomInlineHeader> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for CustomInlineHeaderMut<'_> {
  type Msg = CustomInlineHeader;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CustomInlineHeader> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for CustomInlineHeaderMut<'_> {
  type Msg = CustomInlineHeader;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CustomInlineHeader> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for CustomInlineHeaderView<'_> {
  type Msg = CustomInlineHeader;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CustomInlineHeader> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for CustomInlineHeaderMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod custom_inline_header {
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct InlineHeaderType(i32);

#[allow(non_upper_case_globals)]
impl InlineHeaderType {
  pub const RequestHeader: InlineHeaderType = InlineHeaderType(0);
  pub const RequestTrailer: InlineHeaderType = InlineHeaderType(1);
  pub const ResponseHeader: InlineHeaderType = InlineHeaderType(2);
  pub const ResponseTrailer: InlineHeaderType = InlineHeaderType(3);

  fn constant_name(&self) -> ::std::option::Option<&'static str> {
    #[allow(unreachable_patterns)] // In the case of aliases, just emit them all and let the first one match.
    Some(match self.0 {
      0 => "RequestHeader",
      1 => "RequestTrailer",
      2 => "ResponseHeader",
      3 => "ResponseTrailer",
      _ => return None
    })
  }
}

impl ::std::convert::From<InlineHeaderType> for i32 {
  fn from(val: InlineHeaderType) -> i32 {
    val.0
  }
}

impl ::std::convert::From<i32> for InlineHeaderType {
  fn from(val: i32) -> InlineHeaderType {
    Self(val)
  }
}

impl ::std::default::Default for InlineHeaderType {
  fn default() -> Self {
    Self(0)
  }
}

impl ::std::fmt::Debug for InlineHeaderType {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    if let Some(constant_name) = self.constant_name() {
      write!(f, "InlineHeaderType::{}", constant_name)
    } else {
      write!(f, "InlineHeaderType::from({})", self.0)
    }
  }
}

impl ::protobuf::IntoProxied<i32> for InlineHeaderType {
  fn into_proxied(self, _: ::protobuf::__internal::Private) -> i32 {
    self.0
  }
}

impl ::protobuf::__internal::SealedInternal for InlineHeaderType {}

impl ::protobuf::Proxied for InlineHeaderType {
  type View<'a> = InlineHeaderType;
}

impl ::protobuf::AsView for InlineHeaderType {
  type Proxied = InlineHeaderType;

  fn as_view(&self) -> InlineHeaderType {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for InlineHeaderType {
  fn into_view<'shorter>(self) -> InlineHeaderType where 'msg: 'shorter {
    self
  }
}

// SAFETY: this is an enum type
unsafe impl ::protobuf::__internal::Enum for InlineHeaderType {
  const NAME: &'static str = "InlineHeaderType";

  fn is_known(value: i32) -> bool {
    matches!(value, 0|1|2|3)
  }
}

impl ::protobuf::__internal::EntityType for InlineHeaderType {
    type Tag = ::protobuf::__internal::entity_tag::EnumTag;
}


}  // pub mod custom_inline_header


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut envoy__config__bootstrap__v3__MemoryAllocatorManager_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct MemoryAllocatorManager {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<MemoryAllocatorManager>
}

impl ::protobuf::Message for MemoryAllocatorManager {
  type MessageView<'msg> = MemoryAllocatorManagerView<'msg>;
  type MessageMut<'msg> = MemoryAllocatorManagerMut<'msg>;
}

impl ::std::default::Default for MemoryAllocatorManager {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for MemoryAllocatorManager {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `MemoryAllocatorManager` is `Sync` because it does not implement interior mutability.
//    Neither does `MemoryAllocatorManagerMut`.
unsafe impl ::std::marker::Sync for MemoryAllocatorManager {}

// SAFETY:
// - `MemoryAllocatorManager` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for MemoryAllocatorManager {}

impl ::protobuf::Proxied for MemoryAllocatorManager {
  type View<'msg> = MemoryAllocatorManagerView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for MemoryAllocatorManager {}

impl ::protobuf::MutProxied for MemoryAllocatorManager {
  type Mut<'msg> = MemoryAllocatorManagerMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct MemoryAllocatorManagerView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, MemoryAllocatorManager>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for MemoryAllocatorManagerView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for MemoryAllocatorManagerView<'msg> {
  type Message = MemoryAllocatorManager;
}

impl ::std::fmt::Debug for MemoryAllocatorManagerView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for MemoryAllocatorManagerView<'_> {
  fn default() -> MemoryAllocatorManagerView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, MemoryAllocatorManager>> for MemoryAllocatorManagerView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, MemoryAllocatorManager>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> MemoryAllocatorManagerView<'msg> {

  pub fn to_owned(&self) -> MemoryAllocatorManager {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // bytes_to_release: optional uint64
  pub fn bytes_to_release(self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        0, (0u64).into()
      ).try_into().unwrap()
    }
  }

  // memory_release_interval: optional message google.protobuf.Duration
  pub fn has_memory_release_interval(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn memory_release_interval_opt(self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'msg>> {
    self.has_memory_release_interval().then(|| self.memory_release_interval())
  }
  pub fn memory_release_interval(self) -> ::protobuf_well_known_types::DurationView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }

}

// SAFETY:
// - `MemoryAllocatorManagerView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for MemoryAllocatorManagerView<'_> {}

// SAFETY:
// - `MemoryAllocatorManagerView` is `Send` because while its alive a `MemoryAllocatorManagerMut` cannot.
// - `MemoryAllocatorManagerView` does not use thread-local data.
unsafe impl ::std::marker::Send for MemoryAllocatorManagerView<'_> {}

impl<'msg> ::protobuf::AsView for MemoryAllocatorManagerView<'msg> {
  type Proxied = MemoryAllocatorManager;
  fn as_view(&self) -> ::protobuf::View<'msg, MemoryAllocatorManager> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for MemoryAllocatorManagerView<'msg> {
  fn into_view<'shorter>(self) -> MemoryAllocatorManagerView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<MemoryAllocatorManager> for MemoryAllocatorManagerView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> MemoryAllocatorManager {
    let mut dst = MemoryAllocatorManager::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<MemoryAllocatorManager> for MemoryAllocatorManagerMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> MemoryAllocatorManager {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for MemoryAllocatorManager {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for MemoryAllocatorManagerView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for MemoryAllocatorManagerMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct MemoryAllocatorManagerMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, MemoryAllocatorManager>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for MemoryAllocatorManagerMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for MemoryAllocatorManagerMut<'msg> {
  type Message = MemoryAllocatorManager;
}

impl ::std::fmt::Debug for MemoryAllocatorManagerMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, MemoryAllocatorManager>> for MemoryAllocatorManagerMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, MemoryAllocatorManager>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> MemoryAllocatorManagerMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, MemoryAllocatorManager> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> MemoryAllocatorManager {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // bytes_to_release: optional uint64
  pub fn bytes_to_release(&self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        0, (0u64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_bytes_to_release(&mut self, val: u64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u64_at_index(
        0, val.into()
      )
    }
  }

  // memory_release_interval: optional message google.protobuf.Duration
  pub fn has_memory_release_interval(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_memory_release_interval(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn memory_release_interval_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_memory_release_interval().then(|| self.memory_release_interval())
  }
  pub fn memory_release_interval(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn memory_release_interval_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         1, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_memory_release_interval(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

}

// SAFETY:
// - `MemoryAllocatorManagerMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for MemoryAllocatorManagerMut<'_> {}

// SAFETY:
// - `MemoryAllocatorManagerMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for MemoryAllocatorManagerMut<'_> {}

impl<'msg> ::protobuf::AsView for MemoryAllocatorManagerMut<'msg> {
  type Proxied = MemoryAllocatorManager;
  fn as_view(&self) -> ::protobuf::View<'_, MemoryAllocatorManager> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for MemoryAllocatorManagerMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, MemoryAllocatorManager>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for MemoryAllocatorManagerMut<'msg> {
  type MutProxied = MemoryAllocatorManager;
  fn as_mut(&mut self) -> MemoryAllocatorManagerMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for MemoryAllocatorManagerMut<'msg> {
  fn into_mut<'shorter>(self) -> MemoryAllocatorManagerMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl MemoryAllocatorManager {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, MemoryAllocatorManager> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> MemoryAllocatorManagerView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> MemoryAllocatorManagerMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // bytes_to_release: optional uint64
  pub fn bytes_to_release(&self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        0, (0u64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_bytes_to_release(&mut self, val: u64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u64_at_index(
        0, val.into()
      )
    }
  }

  // memory_release_interval: optional message google.protobuf.Duration
  pub fn has_memory_release_interval(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_memory_release_interval(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn memory_release_interval_opt(&self) -> ::std::option::Option<::protobuf_well_known_types::DurationView<'_>> {
    self.has_memory_release_interval().then(|| self.memory_release_interval())
  }
  pub fn memory_release_interval(&self) -> ::protobuf_well_known_types::DurationView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(::protobuf_well_known_types::DurationView::default())
  }
  pub fn memory_release_interval_mut(&mut self) -> ::protobuf_well_known_types::DurationMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         1, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_memory_release_interval(&mut self,
    val: impl ::protobuf::IntoProxied<::protobuf_well_known_types::Duration>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

}  // impl MemoryAllocatorManager

impl ::std::ops::Drop for MemoryAllocatorManager {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for MemoryAllocatorManager {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for MemoryAllocatorManager {
  type Proxied = Self;
  fn as_view(&self) -> MemoryAllocatorManagerView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for MemoryAllocatorManager {
  type MutProxied = Self;
  fn as_mut(&mut self) -> MemoryAllocatorManagerMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for MemoryAllocatorManager {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::envoy__config__bootstrap__v3__MemoryAllocatorManager_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$,P3");
        ::protobuf::__internal::runtime::link_mini_table(
            super::envoy__config__bootstrap__v3__MemoryAllocatorManager_msg_init.0, &[<::protobuf_well_known_types::Duration as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::envoy__config__bootstrap__v3__MemoryAllocatorManager_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for MemoryAllocatorManager {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for MemoryAllocatorManager {
  type Msg = MemoryAllocatorManager;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MemoryAllocatorManager> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for MemoryAllocatorManager {
  type Msg = MemoryAllocatorManager;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MemoryAllocatorManager> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for MemoryAllocatorManagerMut<'_> {
  type Msg = MemoryAllocatorManager;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MemoryAllocatorManager> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for MemoryAllocatorManagerMut<'_> {
  type Msg = MemoryAllocatorManager;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MemoryAllocatorManager> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for MemoryAllocatorManagerView<'_> {
  type Msg = MemoryAllocatorManager;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<MemoryAllocatorManager> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for MemoryAllocatorManagerMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



