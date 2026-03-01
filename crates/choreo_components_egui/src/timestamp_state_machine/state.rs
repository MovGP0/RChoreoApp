#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlaybackPhase {
    NoMedia,
    ReadyPaused,
    ReadyPlaying,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OwnershipPhase {
    ActorDriven,
    UserPreview,
    SeekCommitPending,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TimestampStateMachineState {
    pub playback_phase: PlaybackPhase,
    pub ownership_phase: OwnershipPhase,
    pub was_playing_before_seek: bool,
    pub pending_seek_position: Option<f64>,
    pub pending_seek_started_at_seconds: Option<f64>,
    pub preview_position: Option<f64>,
    pub is_adjusting_speed: bool,
    pub pause_playback_requested: bool,
    pub resume_playback_requested: bool,
    pub last_actor_sample_accepted: bool,
}

impl Default for TimestampStateMachineState {
    fn default() -> Self {
        Self {
            playback_phase: PlaybackPhase::ReadyPaused,
            ownership_phase: OwnershipPhase::ActorDriven,
            was_playing_before_seek: false,
            pending_seek_position: None,
            pending_seek_started_at_seconds: None,
            preview_position: None,
            is_adjusting_speed: false,
            pause_playback_requested: false,
            resume_playback_requested: false,
            last_actor_sample_accepted: false,
        }
    }
}
