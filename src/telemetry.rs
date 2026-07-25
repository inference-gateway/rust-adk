//! Telemetry bootstrap: bridges the `tracing` facade to OpenTelemetry.
//!
//! [`init`] always installs the `tracing_subscriber` fmt layer. When the
//! `telemetry` Cargo feature is compiled in **and** [`TelemetryConfig::traces_enabled`]
//! is true, it additionally installs a `tracing-opentelemetry` layer that
//! batches spans out to an OTLP collector over HTTP/protobuf.
//!
//! The returned [`TelemetryGuard`] owns the SDK tracer provider; dropping it
//! flushes buffered spans and shuts the exporter down. Bind it with
//! `let _guard = telemetry::init(...)?;` so it outlives `serve()`.
//!
//! [`TelemetryConfig::traces_enabled`]: crate::config::TelemetryConfig::traces_enabled

use crate::config::TelemetryConfig;
use anyhow::Result;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

/// RAII guard keeping the OTLP exporter alive. Drop flushes and shuts it
/// down. Inert (no fields to flush) when telemetry is compiled out or
/// disabled.
#[derive(Default)]
pub struct TelemetryGuard {
    #[cfg(feature = "telemetry")]
    provider: Option<opentelemetry_sdk::trace::SdkTracerProvider>,
}

impl Drop for TelemetryGuard {
    fn drop(&mut self) {
        #[cfg(feature = "telemetry")]
        if let Some(provider) = self.provider.take() {
            // Best-effort flush + shutdown; nothing actionable on failure.
            let _ = provider.shutdown();
        }
    }
}

/// Install the tracing subscriber and, when enabled, the OTLP span exporter.
///
/// `service_name` / `service_version` populate the OTel resource. Endpoint
/// precedence: `cfg.endpoint` (`A2A_TELEMETRY_ENDPOINT`) when set, else the
/// SDK's standard `OTEL_EXPORTER_OTLP_ENDPOINT` resolution (default
/// `http://localhost:4318`).
pub fn init(
    cfg: &TelemetryConfig,
    #[cfg_attr(not(feature = "telemetry"), allow(unused_variables))] service_name: &str,
    #[cfg_attr(not(feature = "telemetry"), allow(unused_variables))] service_version: &str,
) -> Result<TelemetryGuard> {
    let fmt_layer = tracing_subscriber::fmt::layer();
    let env_filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info"));

    #[cfg(feature = "telemetry")]
    if cfg.traces_enabled() {
        use opentelemetry::trace::TracerProvider as _;
        use opentelemetry_otlp::WithExportConfig as _;

        opentelemetry::global::set_text_map_propagator(
            opentelemetry_sdk::propagation::TraceContextPropagator::new(),
        );

        let mut exporter = opentelemetry_otlp::SpanExporter::builder()
            .with_http()
            .with_protocol(opentelemetry_otlp::Protocol::HttpBinary);
        if let Some(endpoint) = cfg.endpoint.as_deref().filter(|e| !e.is_empty()) {
            exporter = exporter.with_endpoint(endpoint);
        }
        let exporter = exporter.build()?;

        let resource = opentelemetry_sdk::Resource::builder()
            .with_service_name(service_name.to_string())
            .with_attribute(opentelemetry::KeyValue::new(
                "service.version",
                service_version.to_string(),
            ))
            .build();

        let provider = opentelemetry_sdk::trace::SdkTracerProvider::builder()
            .with_batch_exporter(exporter)
            .with_resource(resource)
            .build();
        let tracer = provider.tracer("inference-gateway-adk");
        opentelemetry::global::set_tracer_provider(provider.clone());

        let otel_layer = tracing_opentelemetry::layer().with_tracer(tracer);
        let _ = tracing_subscriber::registry()
            .with(env_filter)
            .with(fmt_layer)
            .with(otel_layer)
            .try_init();
        return Ok(TelemetryGuard {
            provider: Some(provider),
        });
    }

    #[cfg(not(feature = "telemetry"))]
    if cfg.traces_enabled() {
        tracing::warn!(
            "A2A_TELEMETRY_ENABLE=true but the `telemetry` Cargo feature is not compiled in; \
             spans will not be exported. Rebuild with `--features telemetry`."
        );
    }

    let _ = tracing_subscriber::registry()
        .with(env_filter)
        .with(fmt_layer)
        .try_init();
    Ok(TelemetryGuard::default())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::TelemetryConfig;

    #[test]
    fn init_disabled_returns_inert_guard_without_collector() {
        let cfg = TelemetryConfig::default();
        let guard = init(&cfg, "test-service", "0.0.0").expect("init should succeed");
        drop(guard);
    }
}
