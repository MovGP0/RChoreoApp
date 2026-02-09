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
    if !has_otel_endpoint() {
        return OTelGuard { provider: None };
    }

    match build_provider() {
        Some(provider) => {
            global::set_tracer_provider(provider.clone());
            OTelGuard {
                provider: Some(provider),
            }
        }
        None => OTelGuard { provider: None },
    }
}

pub(crate) fn capture_trace_context(action_name: &str) -> Option<TraceContext> {
    if !has_otel_endpoint() {
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

fn build_provider() -> Option<SdkTracerProvider> {
    let service_name = std::env::var("OTEL_SERVICE_NAME")
        .ok()
        .filter(|value| !value.trim().is_empty())
        .unwrap_or_else(|| "rchoreo_desktop_debug".to_string());
    let resource = Resource::builder()
        .with_attribute(KeyValue::new(SERVICE_NAME, service_name))
        .build();

    let exporter = match opentelemetry_otlp::SpanExporter::builder()
        .with_http()
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

fn has_otel_endpoint() -> bool {
    has_env_var("OTEL_EXPORTER_OTLP_ENDPOINT") || has_env_var("OTEL_EXPORTER_OTLP_TRACES_ENDPOINT")
}

fn has_env_var(name: &str) -> bool {
    std::env::var(name)
        .ok()
        .is_some_and(|value| !value.trim().is_empty())
}
