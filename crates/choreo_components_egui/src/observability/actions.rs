use super::state::TraceContext;

#[derive(Debug, Clone, PartialEq)]
pub enum ObservabilityAction {
    Initialize,
    SetTracingEnabled {
        enabled: bool,
    },
    StartSpan {
        name: String,
        trace_context: Option<TraceContext>,
    },
    EndActiveSpan,
    SetStringAttribute {
        key: String,
        value: String,
    },
    SetBoolAttribute {
        key: String,
        value: bool,
    },
    SetF64Attribute {
        key: String,
        value: f64,
    },
}
