use std::io;
use std::sync::Arc;

use rspec::ConfigurationBuilder;
use rspec::Logger;
use rspec::Runner;

#[path = "../../src/settings/actions.rs"]
pub mod actions;
#[path = "../../src/settings/reducer.rs"]
pub mod reducer;
#[path = "../../src/settings/state.rs"]
pub mod state;
#[path = "../../src/settings/ui.rs"]
pub mod ui;

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

pub mod audio_backend_preferences_behavior_spec;
pub mod color_preferences_behavior_spec;
pub mod load_settings_preferences_behavior_spec;
pub mod material_theme_application_spec;
pub mod switch_dark_light_mode_behavior_spec;
pub mod ui_smoke_spec;
