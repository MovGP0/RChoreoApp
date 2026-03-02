use std::io;
use std::sync::Arc;

use rspec::ConfigurationBuilder;
use rspec::Logger;
use rspec::Runner;

#[path = "../../src/slider_with_ticks/actions.rs"]
pub mod actions;
#[path = "../../src/slider_with_ticks/reducer.rs"]
pub mod reducer;
#[path = "../../src/slider_with_ticks/state.rs"]
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

pub mod slider_with_ticks_behavior_spec;
