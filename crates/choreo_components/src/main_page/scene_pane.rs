use egui::Color32;
use egui::Stroke;
use egui::StrokeKind;
use egui::Ui;
use egui::UiBuilder;

use crate::choreo_main::actions::ChoreoMainAction;
use crate::choreo_main::state::ChoreoMainState;
use crate::choreo_main::state::InteractionMode;
use crate::material::styling::material_palette::material_palette_for_visuals;
use crate::scene_list_item::SceneItemState;
use crate::scenes;
use crate::scenes::state::ScenesState;
use crate::scenes::state::format_seconds;
use crate::scenes::state::parse_timestamp_seconds;
use choreo_master_mobile_json::Color;
use choreo_master_mobile_json::SceneId;

use super::layout::DRAWER_WIDTH_LEFT_PX;
use super::mappings::map_scene_pane_action;

const SCENES_DEBUG_BORDER_COLOR: Color32 = Color32::from_rgb(0, 208, 96);
const PANEL_DEBUG_CONTENT_PADDING_PX: f32 = 4.0;

pub(super) fn draw_scenes_drawer(
    ui: &mut Ui,
    state: &ChoreoMainState,
    actions: &mut Vec<ChoreoMainAction>,
) {
    let pane_state = scene_pane_state(state);
    let panel_rect = ui.max_rect();
    ui.set_clip_rect(panel_rect);
    ui.set_min_size(panel_rect.size());
    ui.set_width(DRAWER_WIDTH_LEFT_PX);
    ui.set_min_width(DRAWER_WIDTH_LEFT_PX);
    ui.painter()
        .rect_filled(
            panel_rect,
            0.0,
            material_palette_for_visuals(ui.visuals()).surface_container,
        );
    if cfg!(debug_assertions) {
        ui.painter().rect_stroke(
            panel_rect,
            0.0,
            Stroke::new(1.0, SCENES_DEBUG_BORDER_COLOR),
            StrokeKind::Inside,
        );
    }
    let inner_rect = panel_rect.shrink(PANEL_DEBUG_CONTENT_PADDING_PX);
    let _ = ui.scope_builder(UiBuilder::new().max_rect(inner_rect), |ui| {
        for action in scenes::ui::draw(ui, &pane_state) {
            if let Some(mapped_action) = map_scene_pane_action(action) {
                actions.push(mapped_action);
            }
        }
    });
}

#[must_use]
pub fn scene_pane_state(state: &ChoreoMainState) -> ScenesState {
    let scenes = if state
        .choreography_settings_state
        .choreography
        .scenes
        .is_empty()
    {
        state
            .scenes
            .iter()
            .enumerate()
            .map(|(index, scene)| SceneItemState {
                scene_id: SceneId(index as i32 + 1),
                name: scene.name.clone(),
                text: String::new(),
                fixed_positions: false,
                timestamp: scene.timestamp_seconds,
                is_selected: state.selected_scene_index == Some(index),
                positions: Vec::new(),
                variation_depth: 0,
                variations: Vec::new(),
                current_variation: Vec::new(),
                color: Color::transparent(),
            })
            .collect::<Vec<_>>()
    } else {
        state
            .choreography_settings_state
            .choreography
            .scenes
            .iter()
            .enumerate()
            .map(|(index, scene)| project_scene_list_item(index, scene, state))
            .collect::<Vec<_>>()
    };
    let visible_scenes = filter_scene_items(&scenes, &state.scene_search_text);
    let selected_scene = state
        .selected_scene_index
        .and_then(|index| scenes.get(index).cloned());
    let mut pane_state = ScenesState {
        choreography: choreo_models::ChoreographyModel::default(),
        scenes,
        visible_scenes,
        selected_scene: selected_scene.clone(),
        search_text: state.scene_search_text.clone(),
        show_timestamps: state.choreography_settings_state.show_timestamps
            || state
                .choreography_settings_state
                .choreography
                .settings
                .show_timestamps,
        is_place_mode: state.interaction_mode != InteractionMode::View,
        can_save_choreo: can_save_choreo(state),
        can_delete_scene: selected_scene.is_some(),
        can_navigate_to_settings: true,
        can_navigate_to_dancer_settings: true,
        has_selected_scene: selected_scene.is_some(),
        ..ScenesState::default()
    };
    if let Some(selected_scene) = selected_scene {
        pane_state.selected_scene_name = selected_scene.name.clone();
        pane_state.selected_scene_text = selected_scene.text.clone();
        pane_state.selected_scene_fixed_positions = selected_scene.fixed_positions;
        pane_state.selected_scene_timestamp_text = selected_scene
            .timestamp
            .map(format_seconds)
            .unwrap_or_default();
        pane_state.selected_scene_color = selected_scene.color;
    }
    pane_state
}

fn can_save_choreo(state: &ChoreoMainState) -> bool {
    let has_file = state
        .last_opened_choreo_file
        .as_ref()
        .is_some_and(|path| !path.trim().is_empty() && std::path::Path::new(path).exists());
    let choreography = &state.choreography_settings_state.choreography;
    let has_choreo = !choreography.name.is_empty() || !choreography.scenes.is_empty();
    has_file && has_choreo
}

fn filter_scene_items(scenes: &[SceneItemState], search_text: &str) -> Vec<SceneItemState> {
    if search_text.trim().is_empty() {
        return scenes.to_vec();
    }

    let search_text = search_text.to_ascii_lowercase();
    scenes
        .iter()
        .filter(|scene| scene.name.to_ascii_lowercase().contains(&search_text))
        .cloned()
        .collect()
}

fn project_scene_list_item(
    index: usize,
    scene: &choreo_models::SceneModel,
    state: &ChoreoMainState,
) -> SceneItemState {
    SceneItemState {
        scene_id: scene.scene_id,
        name: scene.name.clone(),
        text: scene.text.clone().unwrap_or_default(),
        fixed_positions: scene.fixed_positions,
        timestamp: scene.timestamp.as_deref().and_then(parse_timestamp_seconds),
        is_selected: state.selected_scene_index == Some(index),
        positions: Vec::new(),
        variation_depth: 0,
        variations: Vec::new(),
        current_variation: Vec::new(),
        color: scene.color.clone(),
    }
}
