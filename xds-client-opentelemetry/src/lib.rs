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

//! OpenTelemetry [`MetricsRecorder`] implementation for `xds-client`.
//!
//! [`OtelMetricsRecorder`] adapts the framework-agnostic [`MetricsRecorder`]
//! trait onto an [`opentelemetry::metrics::Meter`], so the gRFC A78 xDS client
//! metrics flow into whatever OpenTelemetry SDK the application has configured.
//!
//! This lives in a dedicated crate (rather than behind a feature on
//! `xds-client`) so the OpenTelemetry version it depends on can move
//! independently of the core `xds-client` release cadence, and so applications
//! that use a different telemetry framework never compile OpenTelemetry at all.
//!
//! # Example
//!
//! ```ignore
//! use std::sync::Arc;
//! use xds_client::{MetricsRecorder, XdsClient};
//! use xds_client_opentelemetry::OtelMetricsRecorder;
//!
//! let meter = opentelemetry::global::meter("grpc-xds");
//! let recorder: Arc<dyn MetricsRecorder> = Arc::new(OtelMetricsRecorder::new(meter));
//! let client = XdsClient::builder(config, transport, codec, runtime)
//!     .with_metrics_recorder(recorder)
//!     .build();
//! ```

use std::collections::HashMap;
use std::sync::Arc;

use opentelemetry::metrics::{Counter, Gauge, Histogram, Meter, UpDownCounter};

use xds_client::metrics::instruments;
use xds_client::{Instrument, InstrumentKind, KeyValue, MetricsRecorder, StringValue, Value};

/// An OpenTelemetry instrument, cached per metric descriptor.
#[derive(Debug)]
enum CachedInstrument {
    Counter(Counter<u64>),
    UpDownCounter(UpDownCounter<i64>),
    Histogram(Histogram<f64>),
    Gauge(Gauge<i64>),
}

/// A [`MetricsRecorder`] backed by an OpenTelemetry [`Meter`].
///
/// Every instrument in [`instruments::ALL`] is created
/// up front in [`new`](Self::new) and stored in an immutable map keyed
/// by the address of its `&'static Instrument` descriptor. Recording is therefore
/// a lock-free shared read of that map (mirroring grpc-go's stats-plugin design),
/// adding no synchronization to the xDS update path. Measurements for an
/// instrument that was not pre-registered are silently dropped.
///
/// The metric name, description, and unit are taken from the [`Instrument`]
/// descriptor, so backends observe the canonical gRFC A78 metadata.
#[derive(Debug)]
pub struct OtelMetricsRecorder {
    instruments: HashMap<usize, CachedInstrument>,
}

impl OtelMetricsRecorder {
    /// Create a recorder that emits measurements through `meter`.
    ///
    /// All instruments in [`instruments::ALL`] are eagerly
    /// created on the `meter`, so no instruments are built on the recording path.
    pub fn new(meter: Meter) -> Self {
        let mut instruments = HashMap::with_capacity(instruments::ALL.len());
        for &instrument in instruments::ALL {
            instruments.insert(key(instrument), build_instrument(&meter, instrument));
        }
        Self { instruments }
    }

    /// Look up the cached instrument for a descriptor, if it was registered.
    fn get(&self, instrument: &'static Instrument) -> Option<&CachedInstrument> {
        self.instruments.get(&key(instrument))
    }
}

/// Stable per-program cache key derived from the descriptor address.
fn key(instrument: &'static Instrument) -> usize {
    instrument as *const Instrument as usize
}

/// Build the OpenTelemetry instrument matching a descriptor's [`InstrumentKind`].
fn build_instrument(meter: &Meter, instrument: &'static Instrument) -> CachedInstrument {
    match instrument.kind {
        InstrumentKind::Counter => CachedInstrument::Counter(
            meter
                .u64_counter(instrument.name)
                .with_description(instrument.description)
                .with_unit(instrument.unit)
                .build(),
        ),
        InstrumentKind::UpDownCounter => CachedInstrument::UpDownCounter(
            meter
                .i64_up_down_counter(instrument.name)
                .with_description(instrument.description)
                .with_unit(instrument.unit)
                .build(),
        ),
        InstrumentKind::Histogram => CachedInstrument::Histogram(
            meter
                .f64_histogram(instrument.name)
                .with_description(instrument.description)
                .with_unit(instrument.unit)
                .build(),
        ),
        InstrumentKind::Gauge => CachedInstrument::Gauge(
            meter
                .i64_gauge(instrument.name)
                .with_description(instrument.description)
                .with_unit(instrument.unit)
                .build(),
        ),
    }
}

impl MetricsRecorder for OtelMetricsRecorder {
    fn add_counter_u64(&self, instrument: &'static Instrument, value: u64, attrs: &[KeyValue]) {
        if let Some(CachedInstrument::Counter(c)) = self.get(instrument) {
            c.add(value, &to_otel_attrs(attrs));
        }
    }

    fn add_up_down_counter_i64(
        &self,
        instrument: &'static Instrument,
        value: i64,
        attrs: &[KeyValue],
    ) {
        if let Some(CachedInstrument::UpDownCounter(c)) = self.get(instrument) {
            c.add(value, &to_otel_attrs(attrs));
        }
    }

    fn record_histogram_f64(
        &self,
        instrument: &'static Instrument,
        value: f64,
        attrs: &[KeyValue],
    ) {
        if let Some(CachedInstrument::Histogram(h)) = self.get(instrument) {
            h.record(value, &to_otel_attrs(attrs));
        }
    }

    fn record_gauge_i64(&self, instrument: &'static Instrument, value: i64, attrs: &[KeyValue]) {
        if let Some(CachedInstrument::Gauge(g)) = self.get(instrument) {
            g.record(value, &to_otel_attrs(attrs));
        }
    }
}

/// Convert the crate's attribute slice into OpenTelemetry key/value pairs.
fn to_otel_attrs(attrs: &[KeyValue]) -> Vec<opentelemetry::KeyValue> {
    attrs.iter().map(to_otel_key_value).collect()
}

fn to_otel_key_value(kv: &KeyValue) -> opentelemetry::KeyValue {
    let value = match &kv.value {
        Value::Bool(b) => opentelemetry::Value::Bool(*b),
        Value::Int(i) => opentelemetry::Value::I64(*i),
        Value::F64(f) => opentelemetry::Value::F64(*f),
        Value::Str(s) => opentelemetry::Value::String(to_otel_string(s)),
    };
    opentelemetry::KeyValue::new(kv.key, value)
}

fn to_otel_string(s: &StringValue) -> opentelemetry::StringValue {
    match s {
        StringValue::Static(st) => (*st).into(),
        StringValue::Owned(o) => o.to_string().into(),
        StringValue::RefCounted(r) => Arc::clone(r).into(),
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::*;

    #[test]
    fn converts_each_value_variant() {
        let bool_kv = to_otel_key_value(&KeyValue::bool("k", true));
        assert_eq!(bool_kv.key.as_str(), "k");
        assert!(matches!(bool_kv.value, opentelemetry::Value::Bool(true)));

        let int_kv = to_otel_key_value(&KeyValue::int("k", 7));
        assert!(matches!(int_kv.value, opentelemetry::Value::I64(7)));

        let f64_kv = to_otel_key_value(&KeyValue::f64("k", 1.5));
        assert!(matches!(f64_kv.value, opentelemetry::Value::F64(v) if v == 1.5));

        let static_kv = to_otel_key_value(&KeyValue::str("k", "acked"));
        match static_kv.value {
            opentelemetry::Value::String(s) => assert_eq!(s.as_str(), "acked"),
            other => panic!("expected string, got {other:?}"),
        }

        let owned_kv = to_otel_key_value(&KeyValue::str("k", String::from("xds:///svc")));
        match owned_kv.value {
            opentelemetry::Value::String(s) => assert_eq!(s.as_str(), "xds:///svc"),
            other => panic!("expected string, got {other:?}"),
        }

        let arc_kv = to_otel_key_value(&KeyValue::str("k", Arc::<str>::from("server:443")));
        match arc_kv.value {
            opentelemetry::Value::String(s) => assert_eq!(s.as_str(), "server:443"),
            other => panic!("expected string, got {other:?}"),
        }
    }

    #[test]
    fn registers_every_instrument_up_front() {
        // The default global meter is a no-op provider; the recorder must still
        // eagerly register one cached instrument per descriptor in `ALL`. This
        // only checks the recorder's internal registration, not exported data.
        let recorder = OtelMetricsRecorder::new(opentelemetry::global::meter("test"));
        assert_eq!(recorder.instruments.len(), instruments::ALL.len());
        for &instrument in instruments::ALL {
            assert!(
                recorder.get(instrument).is_some(),
                "{} not registered",
                instrument.name
            );
        }
    }

    /// Drives the recorder through a *real* OpenTelemetry SDK with an in-memory
    /// exporter and asserts the exported instrument type, value, and attributes.
    /// A no-op global meter cannot catch a wrong instrument type (e.g. the
    /// `resources` gauge being exported as a Sum), so this uses a local
    /// `SdkMeterProvider` rather than the process-wide global provider.
    #[test]
    fn exports_resources_as_gauge_and_counter_as_sum() {
        use opentelemetry::metrics::MeterProvider as _;
        use opentelemetry_sdk::metrics::InMemoryMetricExporter;
        use opentelemetry_sdk::metrics::SdkMeterProvider;
        use opentelemetry_sdk::metrics::data::{AggregatedMetrics, MetricData};

        let exporter = InMemoryMetricExporter::default();
        let provider = SdkMeterProvider::builder()
            .with_periodic_exporter(exporter.clone())
            .build();
        let recorder = OtelMetricsRecorder::new(provider.meter("xds-client-test"));

        recorder.add_counter_u64(
            &instruments::XDS_CLIENT_SERVER_FAILURE,
            3,
            &[KeyValue::str("grpc.target", "xds:///svc")],
        );
        recorder.record_gauge_i64(
            &instruments::XDS_CLIENT_RESOURCES,
            2,
            &[
                KeyValue::str("grpc.target", "xds:///svc"),
                KeyValue::str("grpc.xds.cache_state", "acked"),
            ],
        );

        provider.force_flush().expect("flush metrics");
        let resource_metrics = exporter
            .get_finished_metrics()
            .expect("metrics are expected to be exported");

        let all: Vec<_> = resource_metrics
            .iter()
            .flat_map(|rm| rm.scope_metrics())
            .flat_map(|sm| sm.metrics())
            .collect();
        let metric_named = |name: &str| {
            all.iter()
                .copied()
                .find(|m| m.name() == name)
                .unwrap_or_else(|| panic!("metric {name} not exported"))
        };

        // The A78 counter is exported as a monotonic Sum carrying the value.
        let failures = metric_named("grpc.xds_client.server_failure");
        let AggregatedMetrics::U64(MetricData::Sum(sum)) = failures.data() else {
            panic!(
                "server_failure must be a u64 Sum, got {:?}",
                failures.data()
            );
        };
        assert!(sum.is_monotonic(), "counter must export as a monotonic Sum");
        let sum_points: Vec<_> = sum.data_points().collect();
        assert_eq!(sum_points.len(), 1);
        assert_eq!(sum_points[0].value(), 3);

        // The A78 `resources` metric must be exported as a Gauge (current value),
        // NOT a Sum/UpDownCounter, with the cache_state attribute preserved.
        let resources = metric_named("grpc.xds_client.resources");
        let AggregatedMetrics::I64(MetricData::Gauge(gauge)) = resources.data() else {
            panic!("resources must be an i64 Gauge, got {:?}", resources.data());
        };
        let gauge_points: Vec<_> = gauge.data_points().collect();
        assert_eq!(gauge_points.len(), 1);
        assert_eq!(gauge_points[0].value(), 2);
        let cache_state = gauge_points[0]
            .attributes()
            .find(|kv| kv.key.as_str() == "grpc.xds.cache_state")
            .map(|kv| kv.value.to_string());
        assert_eq!(cache_state.as_deref(), Some("acked"));
    }
}
