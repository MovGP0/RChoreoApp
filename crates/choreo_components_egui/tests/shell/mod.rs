use std::io;
use std::sync::Arc;

use rspec::ConfigurationBuilder;
use rspec::Logger;
use rspec::Runner;

#[path = "../../src/shell/actions.rs"]
pub mod actions;
#[path = "../../src/shell/reducer.rs"]
pub mod reducer;
#[path = "../../src/shell/state.rs"]
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

pub mod app_title_spec;
pub mod material_scheme_applier_spec;
