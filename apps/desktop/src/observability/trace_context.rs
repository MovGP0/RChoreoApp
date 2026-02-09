#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct TraceContext {
    pub trace_id_hex: Option<String>,
    pub span_id_hex: Option<String>,
}
