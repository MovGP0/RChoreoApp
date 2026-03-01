#[derive(Debug, Clone, PartialEq)]
pub enum TimestampStateMachineAction {
    Initialize,
    DragStarted { is_playing: bool },
    PreviewPositionChanged { position: f64 },
    SeekCommitted { position: f64, now_seconds: f64 },
    ActorPositionSampled { position: f64, now_seconds: f64 },
    SetPlaybackFromPlayer { has_player: bool, is_playing: bool },
    CompleteSeekCommit,
    SetIsAdjustingSpeed { value: bool },
}
