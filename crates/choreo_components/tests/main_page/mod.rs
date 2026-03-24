use std::io;
use std::sync::Arc;

use rspec::ConfigurationBuilder;
use rspec::Logger;
use rspec::Runner;

pub use choreo_components::choreo_main::actions;
pub use choreo_components::choreo_main::reducer;
pub use choreo_components::choreo_main::state;
pub use choreo_components::main_page::ui;

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

pub mod ui_parity_spec;
pub mod z_order_spec;
