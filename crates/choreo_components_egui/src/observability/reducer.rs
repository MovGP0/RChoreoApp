use super::actions::ObservabilityAction;
use super::state::{ObservabilityState, SpanAttribute, SpanAttributeValue, SpanRecord};

pub fn reduce(state: &mut ObservabilityState, action: ObservabilityAction) {
    match action {
        ObservabilityAction::Initialize => *state = ObservabilityState::default(),
        ObservabilityAction::SetTracingEnabled { enabled } => {
            state.tracing_enabled = enabled;
            if !enabled {
                state.active_span = None;
            }
        }
        ObservabilityAction::StartSpan {
            name,
            trace_context,
        } => {
            if state.tracing_enabled {
                state.active_span = Some(SpanRecord {
                    name,
                    trace_context,
                    attributes: Vec::new(),
                });
            }
        }
        ObservabilityAction::EndActiveSpan => {
            if let Some(span) = state.active_span.take() {
                state.completed_spans.push(span);
            }
        }
        ObservabilityAction::SetStringAttribute { key, value } => {
            push_active_attribute(state, key, SpanAttributeValue::String(value));
        }
        ObservabilityAction::SetBoolAttribute { key, value } => {
            push_active_attribute(state, key, SpanAttributeValue::Bool(value));
        }
        ObservabilityAction::SetF64Attribute { key, value } => {
            push_active_attribute(state, key, SpanAttributeValue::F64(value));
        }
    }
}

fn push_active_attribute(state: &mut ObservabilityState, key: String, value: SpanAttributeValue) {
    if let Some(span) = state.active_span.as_mut() {
        span.attributes.push(SpanAttribute { key, value });
    }
}
