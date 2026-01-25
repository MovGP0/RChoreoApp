use choreo_models::SceneModel;

use crate::time::{format_seconds, parse_timestamp_seconds};

use super::scenes_view_model::SceneViewModel;

pub struct SceneMapper;

impl SceneMapper {
    pub fn map_view_model_to_model(&self, source: &SceneViewModel, target: &mut SceneModel) {
        target.scene_id = source.scene_id;
        target.name = source.name.clone();
        target.text = normalize_text(&source.text);
        target.fixed_positions = source.fixed_positions;
        target.timestamp = source.timestamp.map(format_seconds);
        target.variation_depth = source.variation_depth;
        target.color = source.color.clone();

        target.variations = source
            .variations
            .iter()
            .map(|list| clone_scene_list(list))
            .collect();
        target.current_variation = clone_scene_list(&source.current_variation);

        target.positions.clear();
        target.positions.extend(source.positions.iter().cloned());
    }

    pub fn map_model_to_view_model(&self, source: &SceneModel, target: &mut SceneViewModel) {
        target.scene_id = source.scene_id;
        target.name = source.name.clone();
        target.text = source.text.clone().unwrap_or_default();
        target.fixed_positions = source.fixed_positions;
        target.timestamp = source
            .timestamp
            .as_deref()
            .and_then(parse_timestamp_seconds);
        target.variation_depth = source.variation_depth;
        target.color = source.color.clone();

        target.positions.clear();
        target.positions.extend(source.positions.iter().cloned());
        target.variations = source
            .variations
            .iter()
            .map(|list| clone_scene_list(list))
            .collect();
        target.current_variation = clone_scene_list(&source.current_variation);
    }
}

fn clone_scene_list(source: &[SceneModel]) -> Vec<SceneModel> {
    source.iter().map(clone_scene).collect()
}

fn clone_scene(source: &SceneModel) -> SceneModel {
    SceneModel {
        scene_id: source.scene_id,
        positions: source.positions.to_vec(),
        name: source.name.clone(),
        text: source.text.clone(),
        fixed_positions: source.fixed_positions,
        timestamp: source.timestamp.clone(),
        variation_depth: source.variation_depth,
        variations: source
            .variations
            .iter()
            .map(|list| clone_scene_list(list))
            .collect(),
        current_variation: clone_scene_list(&source.current_variation),
        color: source.color.clone(),
    }
}

fn normalize_text(value: &str) -> Option<String> {
    if value.trim().is_empty() {
        None
    } else {
        Some(value.trim().to_string())
    }
}


pub(crate) fn build_scene_name(scenes: &[SceneViewModel]) -> String {
    const BASE_NAME: &str = "New Scene";
    if scenes.iter().all(|scene| scene.name != BASE_NAME) {
        return BASE_NAME.to_string();
    }

    let mut suffix = 2;
    loop {
        let candidate = format!("{BASE_NAME} {suffix}");
        if scenes.iter().all(|scene| scene.name != candidate) {
            return candidate;
        }
        suffix += 1;
    }
}

pub(crate) fn next_scene_id(
    scenes: &[SceneViewModel],
) -> choreo_master_mobile_json::SceneId {
    let mut next = 0;
    for scene in scenes {
        next = next.max(scene.scene_id.0 as i64);
    }
    choreo_master_mobile_json::SceneId(next.saturating_add(1) as i32)
}
