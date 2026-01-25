use crate::preferences::Preferences;

use super::scenes_view_model::ScenesPaneViewModel;

pub struct SelectSceneFromAudioPositionBehavior;

impl SelectSceneFromAudioPositionBehavior {
    pub fn update_selection<P: Preferences>(
        view_model: &mut ScenesPaneViewModel<P>,
        position_seconds: f64,
    ) {
        if view_model.scenes.is_empty() {
            return;
        }

        let mut first_index = None;
        for (index, scene) in view_model.scenes.iter().enumerate() {
            if scene.timestamp.is_some() {
                first_index = Some(index);
                break;
            }
        }

        let Some(start_index) = first_index else {
            return;
        };

        let first_scene = &view_model.scenes[start_index];
        let Some(first_timestamp) = first_scene.timestamp else {
            return;
        };

        if position_seconds < first_timestamp {
            view_model.set_selected_scene(Some(first_scene.clone()));
            return;
        }

        for index in start_index..view_model.scenes.len() {
            let current_scene = &view_model.scenes[index];
            let Some(current_timestamp) = current_scene.timestamp else {
                continue;
            };

            let next_index = index + 1;
            if next_index >= view_model.scenes.len() {
                return;
            }

            let next_scene = &view_model.scenes[next_index];
            let Some(next_timestamp) = next_scene.timestamp else {
                continue;
            };

            if position_seconds >= current_timestamp && position_seconds < next_timestamp {
                view_model.set_selected_scene(Some(current_scene.clone()));
                return;
            }
        }
    }
}
