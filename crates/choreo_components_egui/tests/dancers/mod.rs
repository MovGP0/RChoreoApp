use std::io;
use std::sync::Arc;

use choreo_master_mobile_json::Color;
use rspec::{ConfigurationBuilder, Logger, Runner};

#[path = "../../src/dancers/actions.rs"]
pub mod actions;
#[path = "../../src/dancers/reducer.rs"]
pub mod reducer;
#[path = "../../src/dancers/state.rs"]
pub mod state;

pub mod add_dancer_behavior_spec;
pub mod cancel_dancer_settings_behavior_spec;
pub mod delete_dancer_behavior_spec;
pub mod hide_dancer_dialog_behavior_spec;
pub mod load_dancer_settings_behavior_spec;
pub mod reload_dancer_settings_behavior_spec;
pub mod save_dancer_settings_behavior_spec;
pub mod selected_dancer_state_behavior_spec;
pub mod selected_icon_behavior_spec;
pub mod selected_role_behavior_spec;
pub mod show_dancer_dialog_behavior_spec;
pub mod swap_dancer_selection_behavior_spec;
pub mod swap_dancers_behavior_spec;
pub mod update_dancer_details_behavior_spec;

pub use rspec::report::Report;

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

pub fn color(r: u8, g: u8, b: u8) -> Color {
    Color { r, g, b, a: 255 }
}

pub fn role(name: &str) -> state::RoleState {
    state::RoleState {
        name: name.to_string(),
        color: state::transparent_color(),
        z_index: 0,
    }
}

pub fn dancer(
    dancer_id: i32,
    role: state::RoleState,
    name: &str,
    shortcut: &str,
    icon: Option<&str>,
) -> state::DancerState {
    state::DancerState {
        dancer_id,
        role,
        name: name.to_string(),
        shortcut: shortcut.to_string(),
        color: state::transparent_color(),
        icon: icon.map(str::to_string),
    }
}

pub fn position(dancer_id: Option<i32>, dancer_name: Option<&str>) -> state::PositionState {
    state::PositionState {
        dancer_id,
        dancer_name: dancer_name.map(str::to_string),
    }
}

pub fn scene(positions: Vec<state::PositionState>) -> state::SceneState {
    state::SceneState {
        positions,
        variations: Vec::new(),
        current_variation: Vec::new(),
    }
}
