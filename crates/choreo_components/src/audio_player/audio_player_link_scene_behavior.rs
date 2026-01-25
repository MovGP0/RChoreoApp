use crate::behavior::{Behavior, CompositeDisposable};
use crate::global::GlobalStateModel;
use crate::logging::BehaviorLog;
use crate::scenes::SceneViewModel;
use crate::time::format_seconds;

use super::audio_player_view_model::AudioPlayerViewModel;

pub struct AudioPlayerLinkSceneBehavior;

impl AudioPlayerLinkSceneBehavior {
    pub fn link_scene_to_position(
        view_model: &mut AudioPlayerViewModel,
        global_state: &mut GlobalStateModel,
    ) {
        let Some(selected_scene) = global_state.selected_scene.as_mut() else {
            return;
        };

        let Some(rounded_timestamp) =
            try_get_linked_timestamp(view_model, selected_scene, &global_state.scenes)
        else {
            return;
        };

        selected_scene.timestamp = Some(rounded_timestamp);

        if let Some(model_scene) = global_state
            .choreography
            .scenes
            .iter_mut()
            .find(|scene| scene.scene_id == selected_scene.scene_id)
        {
            model_scene.timestamp = Some(format_seconds(rounded_timestamp));
        }

        update_ticks(view_model, &global_state.scenes);
    }

    pub fn update_can_link(view_model: &mut AudioPlayerViewModel, global_state: &GlobalStateModel) {
        let can_link = global_state
            .selected_scene
            .as_ref()
            .and_then(|scene| try_get_linked_timestamp(view_model, scene, &global_state.scenes))
            .is_some();
        view_model.can_link_scene_to_position = can_link;
    }

    pub fn update_ticks(view_model: &mut AudioPlayerViewModel, global_state: &GlobalStateModel) {
        update_ticks(view_model, &global_state.scenes);
    }
}

impl Behavior<AudioPlayerViewModel> for AudioPlayerLinkSceneBehavior {
    fn activate(&self, _view_model: &mut AudioPlayerViewModel, _disposables: &mut CompositeDisposable) {
        BehaviorLog::behavior_activated(
            "AudioPlayerLinkSceneBehavior",
            "AudioPlayerViewModel",
        );
    }
}

fn try_get_linked_timestamp(
    view_model: &AudioPlayerViewModel,
    selected_scene: &SceneViewModel,
    scenes: &[SceneViewModel],
) -> Option<f64> {
    let selected_index = scenes
        .iter()
        .position(|scene| scene.scene_id == selected_scene.scene_id)?;

    let before_timestamp = selected_index
        .checked_sub(1)
        .and_then(|index| scenes[index].timestamp);
    let after_timestamp = scenes
        .get(selected_index + 1)
        .and_then(|scene| scene.timestamp);

    let rounded = round_to_100_millis(view_model.position);

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

pub(crate) fn update_ticks(view_model: &mut AudioPlayerViewModel, scenes: &[SceneViewModel]) {
    let max = view_model.duration;
    let mut ticks: Vec<f64> = scenes
        .iter()
        .filter_map(|scene| scene.timestamp)
        .filter(|value| max <= 0.0 || *value <= max)
        .collect();
    ticks.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    ticks.dedup_by(|a, b| (*a - *b).abs() < 0.000_5);

    let formatted: Vec<String> = ticks.into_iter().map(format_seconds).collect();
    view_model.tick_values = if formatted.is_empty() {
        String::new()
    } else {
        formatted.join(",")
    };
}

fn round_to_100_millis(seconds: f64) -> f64 {
    let milliseconds = seconds * 1000.0;
    let rounded = (milliseconds / 100.0).round() * 100.0;
    rounded / 1000.0
}

