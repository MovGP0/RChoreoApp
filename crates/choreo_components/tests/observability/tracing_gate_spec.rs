use std::io;
use std::sync::Arc;

use choreo_components::observability::is_tracing_enabled;
use choreo_components::observability::set_tracing_enabled;
use choreo_components::observability::start_internal_span;
use rspec::ConfigurationBuilder;
use rspec::Logger;
use rspec::Runner;
use rspec::report::Report;

fn run_suite<T>(suite: &rspec::block::Suite<T>) -> rspec::report::SuiteReport
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

#[test]
#[serial_test::serial]
fn tracing_gate_spec() {
    let suite = rspec::describe("observability tracing gate", (), |spec| {
        spec.it("stops creating spans when tracing is disabled", |_| {
            set_tracing_enabled(false);
            let mut span = start_internal_span("observability.disabled", None);
            span.set_bool_attribute("observability.disabled", true);
            assert!(!is_tracing_enabled());
        });

        spec.it("allows tracing to be re-enabled", |_| {
            set_tracing_enabled(true);
            let mut span = start_internal_span("observability.enabled", None);
            span.set_string_attribute("observability.state", "enabled".to_string());
            assert_eq!(is_tracing_enabled(), cfg!(feature = "otel"));
        });
    });

    let report = run_suite(&suite);
    set_tracing_enabled(true);
    assert!(report.is_success());
}
