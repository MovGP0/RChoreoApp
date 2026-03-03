use std::io;
use std::sync::Arc;

use rspec::ConfigurationBuilder;
use rspec::Logger;
use rspec::Runner;

pub use choreo_components_egui::settings::actions;
pub use choreo_components_egui::settings::provider;
pub use choreo_components_egui::settings::reducer;
pub use choreo_components_egui::settings::state;
pub use choreo_components_egui::settings::translations;
pub use choreo_components_egui::settings::ui;

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
pub mod provider_pipeline_spec;
pub mod switch_dark_light_mode_behavior_spec;
pub mod ui_smoke_spec;
