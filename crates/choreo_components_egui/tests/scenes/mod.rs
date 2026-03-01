#[path = "../../src/scenes/actions.rs"]
pub mod actions;
#[path = "../../src/scenes/reducer.rs"]
pub mod reducer;
#[path = "../../src/scenes/state.rs"]
pub mod state;

pub mod apply_placement_mode_behavior_spec;
pub mod filter_scenes_behavior_spec;
pub mod insert_scene_behavior_spec;
pub mod load_scenes_behavior_spec;
pub mod open_choreo_behavior_spec;
pub mod save_choreo_behavior_spec;
pub mod select_scene_behavior_spec;
pub mod select_scene_from_audio_position_behavior_spec;
pub mod show_scene_timestamps_behavior_spec;

use std::rc::Rc;

use choreo_master_mobile_json::{Color, DancerId, SceneId};
use choreo_models::{ChoreographyModel, DancerModel, PositionModel, RoleModel, SceneModel};

pub fn create_state() -> state::ScenesState {
    state::ScenesState::default()
}

pub fn scene_model(scene_id: i32, name: &str, timestamp: Option<&str>, positions: Vec<PositionModel>) -> SceneModel {
    SceneModel {
        scene_id: SceneId(scene_id),
        positions,
        name: name.to_string(),
        text: None,
        fixed_positions: false,
        timestamp: timestamp.map(str::to_string),
        variation_depth: 0,
        variations: Vec::new(),
        current_variation: Vec::new(),
        color: Color::transparent(),
    }
}

pub fn choreography_with_scenes(name: &str, scenes: Vec<SceneModel>) -> ChoreographyModel {
    ChoreographyModel {
        name: name.to_string(),
        scenes,
        ..ChoreographyModel::default()
    }
}

pub fn build_position(x: f64, y: f64) -> PositionModel {
    PositionModel {
        dancer: None,
        orientation: None,
        x,
        y,
        curve1_x: None,
        curve1_y: None,
        curve2_x: None,
        curve2_y: None,
        movement1_x: None,
        movement1_y: None,
        movement2_x: None,
        movement2_y: None,
    }
}

pub fn build_dancer(dancer_id: i32, name: &str) -> Rc<DancerModel> {
    let role = Rc::new(RoleModel {
        z_index: 0,
        name: "role".to_string(),
        color: Color::transparent(),
    });

    Rc::new(DancerModel {
        dancer_id: DancerId(dancer_id),
        role,
        name: name.to_string(),
        shortcut: name.to_string(),
        color: Color::transparent(),
        icon: None,
    })
}
