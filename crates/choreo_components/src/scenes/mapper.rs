use choreo_models::SceneModel;

use super::view_model::SceneViewModel;

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

fn parse_timestamp_seconds(value: &str) -> Option<f64> {
    let value = value.trim();
    if value.is_empty() {
        return None;
    }

    let mut parts = value.split(':').collect::<Vec<_>>();
    if parts.len() > 3 {
        return None;
    }

    let seconds_part = parts.pop()?;
    let minutes_part = parts.pop().unwrap_or("0");
    let hours_part = parts.pop().unwrap_or("0");

    let seconds = seconds_part.parse::<f64>().ok()?;
    let minutes = minutes_part.parse::<f64>().ok()?;
    let hours = hours_part.parse::<f64>().ok()?;

    Some(hours * 3600.0 + minutes * 60.0 + seconds)
}

fn format_seconds(value: f64) -> String {
    let mut text = format!("{value:.3}");
    if let Some(dot) = text.find('.') {
        while text.ends_with('0') {
            text.pop();
        }
        if text.ends_with('.') {
            text.pop();
        }
        if text.len() == dot {
            text.push('0');
        }
    }
    text
}

fn normalize_text(value: &str) -> Option<String> {
    if value.trim().is_empty() {
        None
    } else {
        Some(value.trim().to_string())
    }
}
