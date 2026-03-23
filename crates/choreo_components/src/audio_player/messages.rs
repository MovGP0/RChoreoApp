use crate::observability::TraceContext;

#[derive(Debug, Clone, PartialEq)]
pub struct AudioPlayerPositionChangedEvent {
    pub position_seconds: f64,
    pub trace_context: Option<TraceContext>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CloseAudioFileCommand {
    pub trace_context: Option<TraceContext>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OpenAudioFileCommand {
    pub file_path: String,
    pub trace_context: Option<TraceContext>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LinkSceneToPositionCommand {
    pub trace_context: Option<TraceContext>,
}
