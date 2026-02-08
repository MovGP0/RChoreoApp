use crate::scenes::SceneViewModel;

pub(crate) fn build_tick_values(duration: f64, scenes: &[SceneViewModel]) -> Vec<f64> {
    let mut ticks: Vec<f64> = scenes
        .iter()
        .filter_map(|scene| scene.timestamp)
        .filter(|value| duration <= 0.0 || *value <= duration)
        .collect();
    ticks.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    ticks.dedup_by(|a, b| (*a - *b).abs() < 0.000_5);
    ticks
}

pub(crate) fn can_link_scene(
    position: f64,
    selected_scene: Option<&SceneViewModel>,
    scenes: &[SceneViewModel],
) -> bool {
    selected_scene
        .and_then(|scene| try_get_linked_timestamp(position, scene, scenes))
        .is_some()
}

pub(crate) fn try_get_linked_timestamp(
    position: f64,
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
