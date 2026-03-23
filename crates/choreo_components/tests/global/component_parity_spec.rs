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

fn global_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("src")
        .join("global")
}

#[test]
fn component_parity_spec() {
    let suite = rspec::describe("global component parity", (), |spec| {
        spec.it("keeps the egui global component non-ui", |_| {
            let mut entries = fs::read_dir(global_dir())
                .expect("global directory should exist")
                .map(|entry| {
                    entry
                        .expect("directory entry should be readable")
                        .file_name()
                        .to_string_lossy()
                        .into_owned()
                })
                .collect::<Vec<_>>();

            entries.sort();

            assert_eq!(
                entries,
                vec![
                    "global_provider.rs".to_string(),
                    "global_state_actor.rs".to_string(),
                    "mod.rs".to_string(),
                    "types.rs".to_string(),
                ]
            );
        });

        spec.it(
            "does not re-export ui from the global module surface",
            |_| {
                let module_source = fs::read_to_string(global_dir().join("mod.rs"))
                    .expect("global mod source should be readable");

                assert!(
                    !module_source.contains("pub mod ui"),
                    "unexpected global export: pub mod ui"
                );
            },
        );
    });

    let report = run_suite(&suite);
    assert!(report.is_success());
}
