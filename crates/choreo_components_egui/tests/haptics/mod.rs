use std::io;
use std::sync::Arc;

use rspec::{ConfigurationBuilder, Logger, Runner};

#[path = "../../src/haptics/actions.rs"]
pub mod actions;
#[path = "../../src/haptics/reducer.rs"]
pub mod reducer;
#[path = "../../src/haptics/state.rs"]
pub mod state;

pub mod click_feedback_spec;
pub mod support_and_backend_spec;

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
