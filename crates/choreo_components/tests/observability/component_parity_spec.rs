use std::fs;
use std::io;
use std::path::PathBuf;
use std::sync::Arc;

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

fn observability_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("src")
        .join("observability")
}

#[test]
fn component_parity_spec() {
    let suite = rspec::describe("observability component parity", (), |spec| {
        spec.it("keeps the egui observability component non-ui", |_| {
            let mut entries = fs::read_dir(observability_dir())
                .expect("observability directory should exist")
                .map(|entry| {
                    entry
                        .expect("directory entry should be readable")
                        .file_name()
                        .to_string_lossy()
                        .into_owned()
                })
                .collect::<Vec<_>>();

            entries.sort();

            assert_eq!(entries, vec!["mod.rs".to_string()]);
        });

        spec.it("does not re-export reducer-style module surface", |_| {
            let module_source = fs::read_to_string(observability_dir().join("mod.rs"))
                .expect("observability mod source should be readable");

            for forbidden_export in [
                "pub mod actions",
                "pub mod reducer",
                "pub mod state",
                "pub mod ui",
            ] {
                assert!(
                    !module_source.contains(forbidden_export),
                    "unexpected observability export: {forbidden_export}"
                );
            }
        });
    });

    let report = run_suite(&suite);
    assert!(report.is_success());
}
