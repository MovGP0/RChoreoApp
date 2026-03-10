#[path = "../../src/scenes/actions.rs"]
pub mod actions;
#[path = "../../src/scenes/provider.rs"]
pub mod provider;
#[path = "../../src/scenes/reducer.rs"]
pub mod reducer;
#[path = "../../src/scenes/state.rs"]
pub mod state;
#[path = "../../src/scenes/translations.rs"]
pub mod translations;
#[path = "../../src/scenes/ui.rs"]
pub mod ui;

pub use choreo_components_egui::material::icons as ui_icons;
pub use rspec::report::Report;

pub mod action_surface_parity_spec;
pub mod apply_placement_mode_behavior_spec;
pub mod copy_scene_positions_dialog_spec;
pub mod delete_scene_dialog_behavior_spec;
pub mod delete_scene_dialog_ui_spec;
pub mod filter_scenes_behavior_spec;
pub mod insert_scene_behavior_spec;
pub mod load_scenes_behavior_spec;
pub mod open_choreo_behavior_spec;
pub mod provider_lifecycle_parity_spec;
pub mod save_choreo_behavior_spec;
pub mod scene_item_view_parity_spec;
pub mod select_scene_behavior_spec;
pub mod select_scene_from_audio_position_behavior_spec;
pub mod selected_scene_detail_projection_spec;
pub mod show_scene_timestamps_behavior_spec;
pub mod ui_action_flow_parity_spec;

use std::io;
use std::rc::Rc;
use std::sync::Arc;

use choreo_master_mobile_json::{Color, DancerId, SceneId};
use choreo_models::{ChoreographyModel, DancerModel, PositionModel, RoleModel, SceneModel};
use rspec::ConfigurationBuilder;
use rspec::Logger;
use rspec::Runner;

pub fn run_suite<T>(suite: &rspec::block::Suite<T>) -> rspec::report::SuiteReport
where
    T: Clone + Send + Sync + std::fmt::Debug,
{
    let configuration = ConfigurationBuilder::default()
        .parallel(false)
        .exit_on_failure(false)
        .build()
        .expect("rspec configuration should build");
    let logger = Arc::new(Logger::new(io::stdout()));
    let runner = Runner::new(configuration, vec![logger]);
    runner.run(suite)
}

pub fn create_state() -> state::ScenesState {
    state::ScenesState::default()
}

pub fn scene_model(
    scene_id: i32,
    name: &str,
    timestamp: Option<&str>,
    positions: Vec<PositionModel>,
) -> SceneModel {
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
