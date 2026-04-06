use super::Report;
use super::actions::AppShellAction;
use super::effects::AppShellEffect;
use super::reducer::reduce;
use super::state::AppShellState;

#[test]
fn external_file_routing_spec() {
    let suite = rspec::describe("app shell external file routing", (), |spec| {
        spec.it("ignores blank file paths", |_| {
            let mut state = AppShellState::default();

            let effects = reduce(
                &mut state,
                AppShellAction::ExternalFilePathReceived {
                    file_path: "   ".to_string(),
                },
            );

            assert!(effects.is_empty());
        });

        spec.it("routes non-empty file paths through a shell effect", |_| {
            let mut state = AppShellState::default();

            let effects = reduce(
                &mut state,
                AppShellAction::ExternalFilePathReceived {
                    file_path: "C:/demo.choreo".to_string(),
                },
            );

            assert_eq!(
                effects,
                vec![AppShellEffect::RouteExternalFilePath {
                    file_path: "C:/demo.choreo".to_string(),
                }]
            );
        });
    });

    let report = super::run_suite(&suite);
    assert!(report.is_success());
}
