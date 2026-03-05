use choreo_components::observability::is_tracing_enabled;
use choreo_components::observability::set_tracing_enabled;
use opentelemetry::KeyValue;
use opentelemetry::global;
use opentelemetry::trace::{Span, SpanKind, Tracer};
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::Resource;
use opentelemetry_sdk::trace::SdkTracerProvider;
use opentelemetry_semantic_conventions::resource::SERVICE_NAME;

use super::TraceContext;

pub(crate) struct OTelGuard {
    provider: Option<SdkTracerProvider>,
}

impl Drop for OTelGuard {
    fn drop(&mut self) {
        if let Some(provider) = self.provider.take() {
            let _ = provider.shutdown();
        }
    }
}

pub(crate) fn init_debug_otel() -> OTelGuard {
    set_tracing_enabled(false);

    let Some(endpoint) = resolve_traces_endpoint() else {
        eprintln!(
            "otel init: disabled (missing OTEL_EXPORTER_OTLP_ENDPOINT or OTEL_EXPORTER_OTLP_TRACES_ENDPOINT)"
        );
        return OTelGuard { provider: None };
    };

    match build_provider(&endpoint) {
        Some(provider) => {
            global::set_tracer_provider(provider.clone());
            set_tracing_enabled(true);
            eprintln!("otel init: enabled, endpoint={endpoint}");
            OTelGuard {
                provider: Some(provider),
            }
        }
        None => OTelGuard { provider: None },
    }
}

pub(crate) fn capture_trace_context(action_name: &str) -> Option<TraceContext> {
    if !is_tracing_enabled() {
        return None;
    }

    let tracer = global::tracer("rchoreo_desktop");
    let mut span = tracer
        .span_builder(format!("user.{action_name}"))
        .with_kind(SpanKind::Internal)
        .start(&tracer);
    let span_context = span.span_context();
    let trace_context = TraceContext {
        trace_id_hex: Some(span_context.trace_id().to_string()),
        span_id_hex: Some(span_context.span_id().to_string()),
    };
    span.end();
    Some(trace_context)
}

fn build_provider(endpoint: &str) -> Option<SdkTracerProvider> {
    let service_name = std::env::var("OTEL_SERVICE_NAME")
        .ok()
        .filter(|value| !value.trim().is_empty())
        .unwrap_or_else(|| "rchoreo_desktop_debug".to_string());
    let resource = Resource::builder()
        .with_attribute(KeyValue::new(SERVICE_NAME, service_name))
        .build();

    let exporter = match opentelemetry_otlp::SpanExporter::builder()
        .with_http()
        .with_endpoint(endpoint.to_string())
        .with_protocol(opentelemetry_otlp::Protocol::HttpBinary)
        .build()
    {
        Ok(exporter) => exporter,
        Err(error) => {
            eprintln!("otel init: failed to build exporter: {error}");
            return None;
        }
    };

    Some(
        SdkTracerProvider::builder()
            .with_simple_exporter(exporter)
            .with_resource(resource)
            .build(),
    )
}

fn env_var(name: &str) -> Option<String> {
    std::env::var(name)
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
}

fn resolve_traces_endpoint() -> Option<String> {
    resolve_traces_endpoint_from(env_var)
}

fn resolve_traces_endpoint_from(read_env: impl Fn(&str) -> Option<String>) -> Option<String> {
    if let Some(endpoint) = read_env("OTEL_EXPORTER_OTLP_TRACES_ENDPOINT") {
        return Some(endpoint);
    }

    let endpoint = read_env("OTEL_EXPORTER_OTLP_ENDPOINT")?;
    if endpoint.ends_with('/') {
        return Some(format!("{endpoint}v1/traces"));
    }

    Some(format!("{endpoint}/v1/traces"))
}

#[cfg(test)]
mod tests {
    use super::resolve_traces_endpoint_from;

    #[test]
    fn resolve_traces_endpoint_prefers_traces_specific_value() {
        let endpoint = resolve_traces_endpoint_from(|name| match name {
            "OTEL_EXPORTER_OTLP_TRACES_ENDPOINT" => {
                Some("http://collector:4318/custom".to_string())
            }
            "OTEL_EXPORTER_OTLP_ENDPOINT" => Some("http://collector:4318".to_string()),
            _ => None,
        });

        assert_eq!(endpoint.as_deref(), Some("http://collector:4318/custom"));
    }

    #[test]
    fn resolve_traces_endpoint_appends_v1_traces_without_double_slash() {
        let trailing_slash = resolve_traces_endpoint_from(|name| {
            (name == "OTEL_EXPORTER_OTLP_ENDPOINT").then(|| "http://collector:4318/".to_string())
        });
        let no_trailing_slash = resolve_traces_endpoint_from(|name| {
            (name == "OTEL_EXPORTER_OTLP_ENDPOINT").then(|| "http://collector:4318".to_string())
        });

        assert_eq!(
            trailing_slash.as_deref(),
            Some("http://collector:4318/v1/traces")
        );
        assert_eq!(
            no_trailing_slash.as_deref(),
            Some("http://collector:4318/v1/traces")
        );
    }

    #[test]
    fn resolve_traces_endpoint_returns_none_when_env_is_missing() {
        let endpoint = resolve_traces_endpoint_from(|_| None);

        assert!(endpoint.is_none());
    }
}
