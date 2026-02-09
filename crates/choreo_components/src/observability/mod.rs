#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct TraceContext {
    pub trace_id_hex: Option<String>,
    pub span_id_hex: Option<String>,
}

pub struct SpanGuard {
    #[cfg(feature = "otel")]
    span: Option<opentelemetry::global::BoxedSpan>,
}

impl SpanGuard {
    pub fn set_string_attribute(&mut self, _key: &'static str, _value: String) {
        #[cfg(feature = "otel")]
        if let Some(span) = self.span.as_mut() {
            use opentelemetry::KeyValue;
            use opentelemetry::trace::Span;

            span.set_attribute(KeyValue::new(_key, _value));
        }
    }

    pub fn set_bool_attribute(&mut self, _key: &'static str, _value: bool) {
        #[cfg(feature = "otel")]
        if let Some(span) = self.span.as_mut() {
            use opentelemetry::KeyValue;
            use opentelemetry::trace::Span;

            span.set_attribute(KeyValue::new(_key, _value));
        }
    }
}

impl Drop for SpanGuard {
    fn drop(&mut self) {
        #[cfg(feature = "otel")]
        if let Some(span) = self.span.as_mut() {
            use opentelemetry::trace::Span;

            span.end();
        }
    }
}

#[cfg(feature = "otel")]
pub fn start_internal_span(name: &'static str, trace_context: Option<&TraceContext>) -> SpanGuard {
    use opentelemetry::global;
    use opentelemetry::trace::{SpanKind, Tracer};

    let tracer = global::tracer("choreo_components");
    let span = if let Some(parent) = build_parent_context(trace_context) {
        tracer
            .span_builder(name)
            .with_kind(SpanKind::Internal)
            .start_with_context(&tracer, &parent)
    } else {
        tracer
            .span_builder(name)
            .with_kind(SpanKind::Internal)
            .start(&tracer)
    };

    SpanGuard { span: Some(span) }
}

#[cfg(feature = "otel")]
fn build_parent_context(trace_context: Option<&TraceContext>) -> Option<opentelemetry::Context> {
    use opentelemetry::Context;
    use opentelemetry::trace::{
        SpanContext, SpanId, TraceContextExt, TraceFlags, TraceId, TraceState,
    };

    let context = trace_context?;
    let trace_id = TraceId::from_hex(context.trace_id_hex.as_deref()?).ok()?;
    let span_id = SpanId::from_hex(context.span_id_hex.as_deref()?).ok()?;
    let span_context = SpanContext::new(
        trace_id,
        span_id,
        TraceFlags::SAMPLED,
        true,
        TraceState::default(),
    );

    Some(Context::new().with_remote_span_context(span_context))
}

#[cfg(not(feature = "otel"))]
pub fn start_internal_span(
    _name: &'static str,
    _trace_context: Option<&TraceContext>,
) -> SpanGuard {
    SpanGuard {}
}
