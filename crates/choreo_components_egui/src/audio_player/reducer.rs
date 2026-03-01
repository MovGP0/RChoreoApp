use super::actions::AudioPlayerAction;
use super::state::AudioPlayerScene;
use super::state::AudioPlayerState;
use super::state::duration_label;
use super::state::speed_to_percent_text;

#[derive(Debug, Clone, PartialEq)]
pub enum AudioPlayerEffect {
    PositionChangedPublished {
        position_seconds: f64,
    },
}

pub fn reduce(state: &mut AudioPlayerState, action: AudioPlayerAction) -> Vec<AudioPlayerEffect> {
    match action {
        AudioPlayerAction::Initialize => {
            state.speed_label = speed_to_percent_text(state.speed);
            state.duration_label = duration_label(state.position, state.duration);
            state.tick_values = build_tick_values(state.duration, &state.scenes);
            state.can_link_scene_to_position =
                can_link_scene(state.position, state.selected_scene_id, &state.scenes);
            Vec::new()
        }
        AudioPlayerAction::TogglePlayPause => {
            if state.has_player {
                state.is_playing = !state.is_playing;
            }
            Vec::new()
        }
        AudioPlayerAction::Stop => {
            state.is_playing = false;
            state.position = 0.0;
            state.duration_label = duration_label(state.position, state.duration);
            Vec::new()
        }
        AudioPlayerAction::SeekToPosition { position } => {
            if !state.can_seek {
                return Vec::new();
            }
            state.position = position;
            state.duration_label = duration_label(state.position, state.duration);
            state.can_link_scene_to_position =
                can_link_scene(state.position, state.selected_scene_id, &state.scenes);
            Vec::new()
        }
        AudioPlayerAction::PositionDragStarted => {
            state.was_playing_before_drag = state.is_playing;
            state.is_user_dragging = true;
            if state.is_playing {
                state.is_playing = false;
            }
            Vec::new()
        }
        AudioPlayerAction::PositionPreviewChanged { position } => {
            if state.is_user_dragging {
                state.pending_seek_position = Some(position);
            }
            Vec::new()
        }
        AudioPlayerAction::PositionDragCompleted { position } => {
            if !state.can_seek {
                state.is_user_dragging = false;
                state.pending_seek_position = None;
                return Vec::new();
            }

            state.position = position;
            state.pending_seek_position = Some(position);
            state.is_playing = state.was_playing_before_drag;
            state.is_user_dragging = false;
            state.duration_label = duration_label(state.position, state.duration);
            state.can_link_scene_to_position =
                can_link_scene(state.position, state.selected_scene_id, &state.scenes);
            Vec::new()
        }
        AudioPlayerAction::PlayerPositionSampled { position } => {
            let Some(pending_seek_position) = state.pending_seek_position else {
                if !state.is_user_dragging {
                    state.position = position;
                    state.duration_label = duration_label(state.position, state.duration);
                    state.can_link_scene_to_position =
                        can_link_scene(state.position, state.selected_scene_id, &state.scenes);
                }
                return Vec::new();
            };

            if (pending_seek_position - position).abs() < 0.0001 {
                state.pending_seek_position = None;
                if !state.is_user_dragging {
                    state.position = position;
                    state.duration_label = duration_label(state.position, state.duration);
                    state.can_link_scene_to_position =
                        can_link_scene(state.position, state.selected_scene_id, &state.scenes);
                }
            }
            Vec::new()
        }
        AudioPlayerAction::SpeedChanged { speed } => {
            if state.is_adjusting_speed {
                return Vec::new();
            }

            let snapped = snap_speed(speed, state.minimum_speed, state.maximum_speed, 0.05);
            state.is_adjusting_speed = true;
            state.speed = snapped;
            state.speed_label = speed_to_percent_text(state.speed);
            state.is_adjusting_speed = false;
            Vec::new()
        }
        AudioPlayerAction::SetScenes {
            scenes,
            selected_scene_id,
            choreography_scenes,
        } => {
            state.scenes = scenes;
            state.selected_scene_id = selected_scene_id;
            state.choreography_scenes = choreography_scenes;
            Vec::new()
        }
        AudioPlayerAction::UpdateTicksAndLinkState => {
            state.tick_values = build_tick_values(state.duration, &state.scenes);
            state.can_link_scene_to_position =
                can_link_scene(state.position, state.selected_scene_id, &state.scenes);
            Vec::new()
        }
        AudioPlayerAction::LinkSceneToPosition => {
            let Some(selected_scene_id) = state.selected_scene_id else {
                return Vec::new();
            };

            let Some(linked_timestamp) =
                try_get_linked_timestamp(state.position, selected_scene_id, &state.scenes)
            else {
                return Vec::new();
            };

            if let Some(scene) = state
                .scenes
                .iter_mut()
                .find(|scene| scene.scene_id == selected_scene_id)
            {
                scene.timestamp = Some(linked_timestamp);
            }

            if let Some(scene) = state
                .choreography_scenes
                .iter_mut()
                .find(|scene| scene.scene_id == selected_scene_id)
            {
                scene.timestamp = Some(format_seconds(linked_timestamp));
            }

            state.tick_values = build_tick_values(state.duration, &state.scenes);
            state.can_link_scene_to_position =
                can_link_scene(state.position, state.selected_scene_id, &state.scenes);
            Vec::new()
        }
        AudioPlayerAction::OpenAudioFile {
            file_path,
            file_exists,
        } => {
            if file_path.trim().is_empty() {
                return Vec::new();
            }

            state.last_opened_audio_file_path = Some(file_path);
            state.has_stream_factory = true;
            state.has_player = file_exists;

            if !file_exists {
                state.can_seek = false;
                state.can_set_speed = false;
                state.duration = 0.0;
                state.position = 0.0;
                state.is_playing = false;
                state.duration_label = duration_label(state.position, state.duration);
            }

            Vec::new()
        }
        AudioPlayerAction::CloseAudioFile => {
            state.has_player = false;
            state.has_stream_factory = false;
            state.position = 0.0;
            state.duration = 0.0;
            state.is_playing = false;
            state.can_seek = false;
            state.can_set_speed = false;
            state.pending_seek_position = None;
            state.duration_label = duration_label(state.position, state.duration);
            state.can_link_scene_to_position = false;
            Vec::new()
        }
        AudioPlayerAction::PublishPositionIfChanged => {
            let should_publish = match state.last_published_position {
                Some(previous) => (previous - state.position).abs() > 0.0001,
                None => true,
            };
            if !should_publish {
                return Vec::new();
            }

            state.last_published_position = Some(state.position);
            vec![AudioPlayerEffect::PositionChangedPublished {
                position_seconds: state.position,
            }]
        }
    }
}

fn snap_speed(value: f64, minimum: f64, maximum: f64, step: f64) -> f64 {
    let clamped = value.clamp(minimum, maximum);
    let snapped = (clamped / step).round() * step;
    snapped.clamp(minimum, maximum)
}

fn build_tick_values(duration: f64, scenes: &[AudioPlayerScene]) -> Vec<f64> {
    let mut ticks: Vec<f64> = scenes
        .iter()
        .filter_map(|scene| scene.timestamp)
        .filter(|value| duration <= 0.0 || *value <= duration)
        .collect();
    ticks.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    ticks.dedup_by(|a, b| (*a - *b).abs() < 0.000_5);
    ticks
}

fn can_link_scene(position: f64, selected_scene_id: Option<i32>, scenes: &[AudioPlayerScene]) -> bool {
    selected_scene_id
        .and_then(|scene_id| try_get_linked_timestamp(position, scene_id, scenes))
        .is_some()
}

fn try_get_linked_timestamp(position: f64, selected_scene_id: i32, scenes: &[AudioPlayerScene]) -> Option<f64> {
    let selected_index = scenes
        .iter()
        .position(|scene| scene.scene_id == selected_scene_id)?;

    let before_timestamp = selected_index
        .checked_sub(1)
        .and_then(|index| scenes[index].timestamp);
    let after_timestamp = scenes
        .get(selected_index + 1)
        .and_then(|scene| scene.timestamp);

    let rounded = round_to_100_millis(position);

    if let Some(before) = before_timestamp
        && rounded <= before
    {
        return None;
    }

    if let Some(after) = after_timestamp
        && rounded >= after
    {
        return None;
    }

    Some(rounded)
}

fn round_to_100_millis(seconds: f64) -> f64 {
    let milliseconds = seconds * 1000.0;
    let rounded = (milliseconds / 100.0).round() * 100.0;
    rounded / 1000.0
}

fn format_seconds(seconds: f64) -> String {
    let normalized = if seconds.abs() < 0.000_05 { 0.0 } else { seconds };
    let mut text = format!("{normalized:.1}");
    if text == "-0.0" {
        text = "0.0".to_string();
    }
    text
}
