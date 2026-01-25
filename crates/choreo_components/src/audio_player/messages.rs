#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AudioPlayerPositionChangedEvent {
    pub position_seconds: f64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CloseAudioFileCommand;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OpenAudioFileCommand {
    pub file_path: String,
}
