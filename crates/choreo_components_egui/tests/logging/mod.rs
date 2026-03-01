use std::io;
use std::sync::Arc;

use rspec::{ConfigurationBuilder, Logger, Runner};

#[path = "../../src/logging/actions.rs"]
pub mod actions;
#[path = "../../src/logging/reducer.rs"]
pub mod reducer;
#[path = "../../src/logging/state.rs"]
pub mod state;
#[path = "../../src/logging/types.rs"]
pub mod types;

pub mod behavior_log_spec;
pub mod behavior_log_api_spec;
pub mod bounded_log_spec;

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
