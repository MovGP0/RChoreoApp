use super::actions::TimestampStateMachineAction;
use super::state::{OwnershipPhase, PlaybackPhase, TimestampStateMachineState};

pub fn reduce(state: &mut TimestampStateMachineState, action: TimestampStateMachineAction) {
    match action {
        TimestampStateMachineAction::Initialize => {
            *state = TimestampStateMachineState::default();
        }
        TimestampStateMachineAction::DragStarted { is_playing } => {
            state.ownership_phase = OwnershipPhase::UserPreview;
            state.was_playing_before_seek = is_playing;
            state.pending_seek_position = None;
            state.pending_seek_started_at_seconds = None;
            state.preview_position = None;
            state.pause_playback_requested = is_playing;
            state.resume_playback_requested = false;
            state.last_actor_sample_accepted = false;
        }
        TimestampStateMachineAction::PreviewPositionChanged { position } => {
            state.preview_position = Some(position);
            state.pause_playback_requested = false;
            state.resume_playback_requested = false;
            state.last_actor_sample_accepted = false;
        }
        TimestampStateMachineAction::SeekCommitted {
            position,
            now_seconds,
        } => {
            state.ownership_phase = OwnershipPhase::SeekCommitPending;
            state.pending_seek_position = Some(position);
            state.pending_seek_started_at_seconds = Some(now_seconds);
            state.preview_position = Some(position);
            state.pause_playback_requested = false;
            state.resume_playback_requested = false;
            state.last_actor_sample_accepted = false;
        }
        TimestampStateMachineAction::ActorPositionSampled {
            position,
            now_seconds,
        } => {
            state.last_actor_sample_accepted = should_accept_actor_position(state, position, now_seconds);
            state.pause_playback_requested = false;
            state.resume_playback_requested = false;
        }
        TimestampStateMachineAction::SetPlaybackFromPlayer {
            has_player,
            is_playing,
        } => {
            state.playback_phase = if !has_player {
                PlaybackPhase::NoMedia
            } else if is_playing {
                PlaybackPhase::ReadyPlaying
            } else {
                PlaybackPhase::ReadyPaused
            };
        }
        TimestampStateMachineAction::CompleteSeekCommit => {
            state.ownership_phase = OwnershipPhase::ActorDriven;
            state.resume_playback_requested = state.was_playing_before_seek;
            state.was_playing_before_seek = false;
            state.pause_playback_requested = false;
            state.last_actor_sample_accepted = false;
        }
        TimestampStateMachineAction::SetIsAdjustingSpeed { value } => {
            state.is_adjusting_speed = value;
        }
    }
}

const POSITION_SYNC_TOLERANCE_SECONDS: f64 = 0.2;
const POSITION_SYNC_TIMEOUT_SECONDS: f64 = 1.5;

fn should_accept_actor_position(
    state: &mut TimestampStateMachineState,
    player_position: f64,
    now_seconds: f64,
) -> bool {
    if state.ownership_phase == OwnershipPhase::UserPreview {
        return false;
    }

    let Some(target_position) = state.pending_seek_position else {
        state.ownership_phase = OwnershipPhase::ActorDriven;
        return true;
    };

    let has_reached_target = (player_position - target_position).abs() <= POSITION_SYNC_TOLERANCE_SECONDS;
    let timed_out = state
        .pending_seek_started_at_seconds
        .map(|started_at| now_seconds - started_at >= POSITION_SYNC_TIMEOUT_SECONDS)
        .unwrap_or(false);

    if has_reached_target || timed_out {
        state.pending_seek_position = None;
        state.pending_seek_started_at_seconds = None;
        state.ownership_phase = OwnershipPhase::ActorDriven;
        return true;
    }

    false
}
