use std::io;
use std::sync::Arc;

use rspec::{ConfigurationBuilder, Logger, Runner};

#[path = "../../src/timestamp_state_machine/actions.rs"]
pub mod actions;
#[path = "../../src/timestamp_state_machine/reducer.rs"]
pub mod reducer;
#[path = "../../src/timestamp_state_machine/state.rs"]
pub mod state;

pub mod actor_sync_spec;
pub mod drag_and_commit_spec;
pub mod playback_phase_spec;

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
