use std::io;
use std::sync::Arc;

use rspec::ConfigurationBuilder;
use rspec::Logger;
use rspec::Runner;

#[path = "../../src/drawer_host/actions.rs"]
pub mod actions;
#[path = "../../src/drawer_host/reducer.rs"]
pub mod reducer;
#[path = "../../src/drawer_host/state.rs"]
pub mod state;
#[path = "../../src/drawer_host/ui.rs"]
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

pub mod absolute_origin_spec;
pub mod layout_state_spec;
pub mod overlay_behavior_spec;
pub mod slot_rendering_spec;
pub mod ui_smoke_spec;
