#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct TraceContext {
    pub trace_id_hex: Option<String>,
    pub span_id_hex: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SpanAttributeValue {
    String(String),
    Bool(bool),
    F64(f64),
}

#[derive(Debug, Clone, PartialEq)]
pub struct SpanAttribute {
    pub key: String,
    pub value: SpanAttributeValue,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SpanRecord {
    pub name: String,
    pub trace_context: Option<TraceContext>,
    pub attributes: Vec<SpanAttribute>,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ObservabilityState {
    pub tracing_enabled: bool,
    pub active_span: Option<SpanRecord>,
    pub completed_spans: Vec<SpanRecord>,
}
