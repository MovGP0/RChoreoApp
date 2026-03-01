use std::io;
use std::sync::Arc;

use rspec::ConfigurationBuilder;
use rspec::Logger;
use rspec::Runner;

#[path = "../../src/choreo_main/actions.rs"]
pub mod actions;
#[path = "../../src/choreo_main/reducer.rs"]
pub mod reducer;
#[path = "../../src/choreo_main/state.rs"]
pub mod state;

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

pub mod apply_interaction_mode_behavior_spec;
pub mod hide_dialog_behavior_spec;
pub mod navigate_dancers_to_main_spec;
pub mod navigate_main_to_dancers_spec;
pub mod navigate_main_to_settings_spec;
pub mod navigate_settings_to_main_spec;
pub mod open_audio_behavior_spec;
pub mod open_image_behavior_spec;
pub mod open_svg_file_behavior_spec;
pub mod show_dialog_behavior_spec;
pub mod timestamp_sync_spec;
