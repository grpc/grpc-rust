//! Metrics extension point for xds-client.
//!
//! Defines a framework-agnostic [`MetricsRecorder`] trait that backends implement
//! to receive metric measurements emitted by the client. Modeled after gRFC A79's
//! `MetricsRecorder` abstraction.
//!
//! No bundled implementation is provided in this crate today; consumers wanting
//! to ship metrics to a real backend must implement [`MetricsRecorder`]
//! themselves. A bundled OpenTelemetry implementation behind an `otel` Cargo
//! feature is planned but not yet implemented.
//!
//! # Example
//!
//! ```ignore
//! use std::sync::Arc;
//! use xds_client::metrics::MetricsRecorder;
//!
//! struct MyRecorder;
//! impl MetricsRecorder for MyRecorder { /* ... */ }
//!
//! let recorder: Arc<dyn MetricsRecorder> = Arc::new(MyRecorder);
//! let client = XdsClient::builder(config, transport, codec, runtime)
//!     .with_metrics_recorder(recorder)
//!     .build();
//! ```

use std::borrow::Cow;
use std::fmt;
use std::sync::Arc;

/// Static descriptor for a metric instrument.
///
/// One `Instrument` is declared per metric as a `pub static` constant. Call
/// sites reference instruments by `&'static Instrument`. Backend implementations
/// may use the instrument address as a cache key (`instrument as *const _`).
#[derive(Debug)]
pub struct Instrument {
    /// Metric name (e.g. `"grpc.xds_client.connected"`).
    pub name: &'static str,
    /// Human-readable description.
    pub description: &'static str,
    /// OpenTelemetry unit notation (e.g. `"s"`, `"By"`, `"{bool}"`, `"{resource}"`).
    pub unit: &'static str,
    /// The kind of instrument.
    pub kind: InstrumentKind,
}

/// The kind of metric instrument.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InstrumentKind {
    /// Monotonic `u64` counter.
    Counter,
    /// Bidirectional `i64` counter, used for gauges emitted as deltas
    /// (e.g. `grpc.xds_client.resources`).
    UpDownCounter,
    /// Distribution of `f64` values.
    Histogram,
    /// Last-value `i64` gauge (push model).
    Gauge,
}

/// An attribute key/value pair attached to a single measurement.
#[derive(Debug, Clone)]
pub struct KeyValue {
    /// Attribute name. Keys are `'static` because the set of attribute keys
    /// per metric is fixed.
    pub key: &'static str,
    /// Attribute value.
    pub value: Value,
}

impl KeyValue {
    /// Construct a string-valued attribute.
    ///
    /// Accepts any value convertible to [`StringValue`]: `&'static str`,
    /// `String`, `Box<str>`, `Arc<str>`, or `Cow<'static, str>`. The choice
    /// affects allocation cost — see [`StringValue`].
    pub fn str(key: &'static str, value: impl Into<StringValue>) -> Self {
        Self {
            key,
            value: Value::Str(value.into()),
        }
    }

    /// Construct a boolean-valued attribute.
    pub fn bool(key: &'static str, value: bool) -> Self {
        Self {
            key,
            value: Value::Bool(value),
        }
    }

    /// Construct an integer-valued attribute.
    pub fn int(key: &'static str, value: i64) -> Self {
        Self {
            key,
            value: Value::Int(value),
        }
    }

    /// Construct an f64-valued attribute.
    pub fn f64(key: &'static str, value: f64) -> Self {
        Self {
            key,
            value: Value::F64(value),
        }
    }
}

/// A typed attribute value.
#[derive(Debug, Clone)]
pub enum Value {
    /// Boolean value.
    Bool(bool),
    /// Signed 64-bit integer value.
    Int(i64),
    /// 64-bit floating-point value.
    F64(f64),
    /// String value. See [`StringValue`] for ownership modes.
    Str(StringValue),
}

/// String attribute value with three ownership modes.
///
/// Mirrors the `OtelString` design from the OpenTelemetry Rust SDK: a single
/// non-lifetime-parameterized type that supports static borrows, owned strings,
/// and refcounted strings. This keeps [`MetricsRecorder`] trait object-safe
/// while letting callers pick the cheapest representation for the value at
/// hand:
///
/// - [`Static`](Self::Static) — for compile-time-known values
///   (e.g. cache_state labels like `"acked"`). Zero allocation.
/// - [`Owned`](Self::Owned) — for runtime-built strings the recorder will own.
///   One heap allocation per value.
/// - [`RefCounted`](Self::RefCounted) — for runtime values shared across many
///   emissions (e.g. the channel target stored on the worker). Cloning is one
///   atomic op, no allocation.
///
/// `From` impls cover the common conversions, so call sites usually need only
/// `KeyValue::str(KEY, value)` with whatever string-like type they have.
#[derive(Debug, Clone)]
pub enum StringValue {
    /// Compile-time known string. Zero-cost.
    Static(&'static str),
    /// Owned runtime-built string.
    Owned(Box<str>),
    /// Reference-counted string. Cheap to clone (atomic op only).
    RefCounted(Arc<str>),
}

impl StringValue {
    /// Borrow the string content regardless of variant.
    pub fn as_str(&self) -> &str {
        match self {
            StringValue::Static(s) => s,
            StringValue::Owned(s) => s,
            StringValue::RefCounted(s) => s,
        }
    }
}

impl AsRef<str> for StringValue {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for StringValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl From<&'static str> for StringValue {
    fn from(s: &'static str) -> Self {
        StringValue::Static(s)
    }
}

impl From<String> for StringValue {
    fn from(s: String) -> Self {
        StringValue::Owned(s.into_boxed_str())
    }
}

impl From<Box<str>> for StringValue {
    fn from(s: Box<str>) -> Self {
        StringValue::Owned(s)
    }
}

impl From<Arc<str>> for StringValue {
    fn from(s: Arc<str>) -> Self {
        StringValue::RefCounted(s)
    }
}

impl From<Cow<'static, str>> for StringValue {
    fn from(c: Cow<'static, str>) -> Self {
        match c {
            Cow::Borrowed(s) => StringValue::Static(s),
            Cow::Owned(s) => StringValue::Owned(s.into_boxed_str()),
        }
    }
}

/// Backend interface for recording metric measurements.
///
/// Implementors translate calls into measurements on their telemetry backend.
/// Implementations must be cheap and lock-free where possible since these calls
/// happen on hot paths.
pub trait MetricsRecorder: Send + Sync + 'static {
    /// Add a value to a monotonic counter.
    fn add_counter_u64(&self, instrument: &'static Instrument, value: u64, attrs: &[KeyValue]);

    /// Add a (possibly negative) delta to an up-down counter.
    fn add_up_down_counter_i64(
        &self,
        instrument: &'static Instrument,
        value: i64,
        attrs: &[KeyValue],
    );

    /// Record a value in a histogram.
    fn record_histogram_f64(&self, instrument: &'static Instrument, value: f64, attrs: &[KeyValue]);

    /// Record the current value of a push-model gauge.
    fn record_gauge_i64(&self, instrument: &'static Instrument, value: i64, attrs: &[KeyValue]);
}

/// Instrument descriptors for the gRFC A78 XdsClient metrics emitted by this crate.
pub mod instruments {
    use super::{Instrument, InstrumentKind};

    /// `grpc.xds_client.connected` — gauge indicating whether the client has an active ADS stream.
    pub static XDS_CLIENT_CONNECTED: Instrument = Instrument {
        name: "grpc.xds_client.connected",
        description: "Whether the xDS client currently has an active ADS stream to the xDS server.",
        unit: "{bool}",
        kind: InstrumentKind::Gauge,
    };

    /// `grpc.xds_client.server_failure` — counter of xDS server failure transitions.
    pub static XDS_CLIENT_SERVER_FAILURE: Instrument = Instrument {
        name: "grpc.xds_client.server_failure",
        description: "Number of times the xDS server transitioned from healthy to unhealthy.",
        unit: "{failure}",
        kind: InstrumentKind::Counter,
    };

    /// `grpc.xds_client.resource_updates_valid` — counter of resources received and successfully decoded.
    pub static XDS_CLIENT_RESOURCE_UPDATES_VALID: Instrument = Instrument {
        name: "grpc.xds_client.resource_updates_valid",
        description: "Number of resources received and successfully decoded.",
        unit: "{resource}",
        kind: InstrumentKind::Counter,
    };

    /// `grpc.xds_client.resource_updates_invalid` — counter of resources that failed codec-level validation.
    pub static XDS_CLIENT_RESOURCE_UPDATES_INVALID: Instrument = Instrument {
        name: "grpc.xds_client.resource_updates_invalid",
        description: "Number of resources received that failed codec-level validation.",
        unit: "{resource}",
        kind: InstrumentKind::Counter,
    };

    /// `grpc.xds_client.resources` — gauge of cached xDS resources, emitted as up-down-counter deltas.
    ///
    /// Use `cache_state` attribute values from [`super::cache_state`].
    pub static XDS_CLIENT_RESOURCES: Instrument = Instrument {
        name: "grpc.xds_client.resources",
        description: "Number of xDS resources currently cached, broken down by cache state.",
        unit: "{resource}",
        kind: InstrumentKind::UpDownCounter,
    };
}

/// Attribute keys used by the gRFC A78 XdsClient metrics.
pub mod attrs {
    /// `grpc.target` — the channel target (configured xDS URI).
    pub const GRPC_TARGET: &str = "grpc.target";
    /// `grpc.xds.server` — URI of the xDS server.
    pub const GRPC_XDS_SERVER: &str = "grpc.xds.server";
    /// `grpc.xds.authority` — xDS authority name (when bootstrap defines named authorities).
    pub const GRPC_XDS_AUTHORITY: &str = "grpc.xds.authority";
    /// `grpc.xds.cache_state` — cache state of a resource. Canonical values per
    /// gRFC A78: `requested`, `acked`, `nacked`, `does_not_exist`, `nacked_but_cached`.
    pub const GRPC_XDS_CACHE_STATE: &str = "grpc.xds.cache_state";
    /// `grpc.xds.resource_type` — type URL of the resource.
    pub const GRPC_XDS_RESOURCE_TYPE: &str = "grpc.xds.resource_type";
}
