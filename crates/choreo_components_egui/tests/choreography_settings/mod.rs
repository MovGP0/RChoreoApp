#[path = "../../src/choreography_settings/actions.rs"]
pub mod actions;
#[path = "../../src/choreography_settings/reducer.rs"]
pub mod reducer;
#[path = "../../src/choreography_settings/state.rs"]
pub mod state;

pub mod load_choreography_settings_behavior_spec;
pub mod load_settings_preferences_behavior_spec;
pub mod update_author_behavior_spec;
pub mod update_comment_behavior_spec;
pub mod update_date_behavior_spec;
pub mod update_description_behavior_spec;
pub mod update_draw_path_from_behavior_spec;
pub mod update_draw_path_to_behavior_spec;
pub mod update_floor_back_behavior_spec;
pub mod update_floor_color_behavior_spec;
pub mod update_floor_front_behavior_spec;
pub mod update_floor_left_behavior_spec;
pub mod update_floor_right_behavior_spec;
pub mod update_grid_lines_behavior_spec;
pub mod update_grid_resolution_behavior_spec;
pub mod update_name_behavior_spec;
pub mod update_positions_at_side_behavior_spec;
pub mod update_selected_scene_behavior_spec;
pub mod update_show_legend_behavior_spec;
pub mod update_show_timestamps_behavior_spec;
pub mod update_snap_to_grid_behavior_spec;
pub mod update_subtitle_behavior_spec;
pub mod update_transparency_behavior_spec;
pub mod update_variation_behavior_spec;

use choreo_master_mobile_json::{Color, SceneId};
use choreo_models::{ChoreographyModel, SceneModel};

pub fn create_state() -> state::ChoreographySettingsState {
    state::ChoreographySettingsState::default()
}

pub fn color(a: u8, r: u8, g: u8, b: u8) -> Color {
    Color { a, r, g, b }
}

pub fn selected_scene(scene_id: i32, name: &str) -> state::SelectedSceneState {
    state::SelectedSceneState {
        scene_id: SceneId(scene_id),
        name: name.to_string(),
        text: String::new(),
        fixed_positions: false,
        timestamp: None,
        color: Color::transparent(),
    }
}

pub fn scene_model(scene_id: i32, name: &str, text: Option<&str>, timestamp: Option<&str>) -> SceneModel {
    SceneModel {
        scene_id: SceneId(scene_id),
        positions: Vec::new(),
        name: name.to_string(),
        text: text.map(str::to_string),
        fixed_positions: false,
        timestamp: timestamp.map(str::to_string),
        variation_depth: 0,
        variations: Vec::new(),
        current_variation: Vec::new(),
        color: Color::transparent(),
    }
}

pub fn choreography_with_name(name: &str) -> ChoreographyModel {
    ChoreographyModel {
        name: name.to_string(),
        ..ChoreographyModel::default()
    }
}
