use std::net::TcpStream;
use std::net::ToSocketAddrs;
use std::time::Duration;

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

    if !is_endpoint_reachable(&endpoint) {
        eprintln!("otel init: disabled (endpoint unreachable: {endpoint})");
        return OTelGuard { provider: None };
    }

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

    resolve_traces_endpoint()?;

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
    if let Some(endpoint) = env_var("OTEL_EXPORTER_OTLP_TRACES_ENDPOINT") {
        return Some(endpoint);
    }

    let endpoint = env_var("OTEL_EXPORTER_OTLP_ENDPOINT")?;
    if endpoint.ends_with('/') {
        return Some(format!("{endpoint}v1/traces"));
    }

    Some(format!("{endpoint}/v1/traces"))
}

fn is_endpoint_reachable(endpoint: &str) -> bool {
    let Some((host, port)) = parse_endpoint_host_port(endpoint) else {
        eprintln!("otel init: disabled (invalid OTLP endpoint URL: {endpoint})");
        return false;
    };

    let Ok(addresses) = (host.as_str(), port).to_socket_addrs() else {
        return false;
    };

    for address in addresses {
        if TcpStream::connect_timeout(&address, Duration::from_millis(75)).is_ok() {
            return true;
        }
    }

    false
}

fn parse_endpoint_host_port(endpoint: &str) -> Option<(String, u16)> {
    let (scheme, rest) = endpoint.split_once("://")?;
    let default_port = match scheme {
        "http" => 80,
        "https" => 443,
        _ => return None,
    };

    let authority = rest.split('/').next()?.trim();
    if authority.is_empty() {
        return None;
    }

    if authority.starts_with('[') {
        let bracket_end = authority.find(']')?;
        let host = authority[..=bracket_end].to_string();
        let suffix = &authority[(bracket_end + 1)..];
        let port = if let Some(port_text) = suffix.strip_prefix(':') {
            port_text.parse::<u16>().ok()?
        } else {
            default_port
        };
        return Some((host, port));
    }

    if let Some((host, port_text)) = authority.rsplit_once(':')
        && let Ok(port) = port_text.parse::<u16>()
    {
        return Some((host.to_string(), port));
    }

    Some((authority.to_string(), default_port))
}
