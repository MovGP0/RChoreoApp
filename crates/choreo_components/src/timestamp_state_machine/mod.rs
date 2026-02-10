use std::time::{Duration, Instant};

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

#[derive(Debug, Clone, Copy)]
pub enum TimestampEvent {
    DragStarted { is_playing: bool },
    PreviewPositionChanged { position: f64 },
    SeekCommitted { position: f64 },
    ActorPositionSampled { position: f64 },
}

#[derive(Debug, Clone, Copy)]
pub struct TimestampStateSnapshot {
    pub playback_phase: PlaybackPhase,
    pub ownership_phase: OwnershipPhase,
    pub pending_seek_position: Option<f64>,
    pub pending_seek_age_seconds: Option<f64>,
    pub was_playing_before_seek: bool,
    pub is_adjusting_speed: bool,
}

pub struct TimestampOwnershipStateMachine {
    playback_phase: PlaybackPhase,
    ownership_phase: OwnershipPhase,
    was_playing_before_seek: bool,
    pending_seek_position: Option<f64>,
    pending_seek_started_at: Option<Instant>,
    preview_position: Option<f64>,
    is_adjusting_speed: bool,
}

impl Default for TimestampOwnershipStateMachine {
    fn default() -> Self {
        Self::new()
    }
}

impl TimestampOwnershipStateMachine {
    const POSITION_SYNC_TOLERANCE_SECONDS: f64 = 0.2;
    const POSITION_SYNC_TIMEOUT: Duration = Duration::from_millis(1500);

    pub fn new() -> Self {
        Self {
            playback_phase: PlaybackPhase::ReadyPaused,
            ownership_phase: OwnershipPhase::ActorDriven,
            was_playing_before_seek: false,
            pending_seek_position: None,
            pending_seek_started_at: None,
            preview_position: None,
            is_adjusting_speed: false,
        }
    }

    pub fn apply_event(&mut self, event: TimestampEvent) -> bool {
        match event {
            TimestampEvent::DragStarted { is_playing } => {
                self.ownership_phase = OwnershipPhase::UserPreview;
                self.was_playing_before_seek = is_playing;
                self.pending_seek_position = None;
                self.pending_seek_started_at = None;
                self.preview_position = None;
                is_playing
            }
            TimestampEvent::PreviewPositionChanged { position } => {
                self.preview_position = Some(position);
                false
            }
            TimestampEvent::SeekCommitted { position } => {
                self.ownership_phase = OwnershipPhase::SeekCommitPending;
                self.pending_seek_position = Some(position);
                self.pending_seek_started_at = Some(Instant::now());
                self.preview_position = Some(position);
                false
            }
            TimestampEvent::ActorPositionSampled { position } => {
                self.should_accept_actor_position(position)
            }
        }
    }

    pub fn set_playback_from_player(&mut self, has_player: bool, is_playing: bool) {
        self.playback_phase = if !has_player {
            PlaybackPhase::NoMedia
        } else if is_playing {
            PlaybackPhase::ReadyPlaying
        } else {
            PlaybackPhase::ReadyPaused
        };
    }

    pub fn complete_seek_commit(&mut self) -> bool {
        let was_playing = self.was_playing_before_seek;
        self.ownership_phase = OwnershipPhase::ActorDriven;
        self.was_playing_before_seek = false;
        was_playing
    }

    pub fn is_user_previewing(&self) -> bool {
        self.ownership_phase == OwnershipPhase::UserPreview
    }

    pub fn snapshot(&self) -> TimestampStateSnapshot {
        TimestampStateSnapshot {
            playback_phase: self.playback_phase,
            ownership_phase: self.ownership_phase,
            pending_seek_position: self.pending_seek_position,
            pending_seek_age_seconds: self
                .pending_seek_started_at
                .map(|started_at| started_at.elapsed().as_secs_f64()),
            was_playing_before_seek: self.was_playing_before_seek,
            is_adjusting_speed: self.is_adjusting_speed,
        }
    }

    pub fn set_is_adjusting_speed(&mut self, value: bool) {
        self.is_adjusting_speed = value;
    }

    pub fn is_adjusting_speed(&self) -> bool {
        self.is_adjusting_speed
    }

    fn should_accept_actor_position(&mut self, player_position: f64) -> bool {
        if self.ownership_phase == OwnershipPhase::UserPreview {
            return false;
        }

        let Some(target_position) = self.pending_seek_position else {
            self.ownership_phase = OwnershipPhase::ActorDriven;
            return true;
        };

        let has_reached_target =
            (player_position - target_position).abs() <= Self::POSITION_SYNC_TOLERANCE_SECONDS;
        let timed_out = self
            .pending_seek_started_at
            .is_some_and(|started_at| started_at.elapsed() >= Self::POSITION_SYNC_TIMEOUT);

        if has_reached_target || timed_out {
            self.pending_seek_position = None;
            self.pending_seek_started_at = None;
            self.ownership_phase = OwnershipPhase::ActorDriven;
            return true;
        }

        false
    }
}
