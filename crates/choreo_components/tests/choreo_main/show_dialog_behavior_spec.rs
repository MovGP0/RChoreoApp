use crate::choreo_main::Report;
use crate::choreo_main::actions::ChoreoMainAction;
use crate::choreo_main::reducer::reduce;
use crate::choreo_main::state::ChoreoMainState;

#[test]
fn show_dialog_behavior_spec() {
    let suite = rspec::describe("show dialog reducer behavior", (), |spec| {
        spec.it("opens dialog with provided content", |_| {
            let mut state = ChoreoMainState::default();
            reduce(
                &mut state,
                ChoreoMainAction::ShowDialog {
                    content: Some("dialog content".to_string()),
                },
            );

            assert!(state.is_dialog_open);
            assert_eq!(state.dialog_content.as_deref(), Some("dialog content"));
        });
    });

    let report = crate::choreo_main::run_suite(&suite);
    assert!(report.is_success());
}
