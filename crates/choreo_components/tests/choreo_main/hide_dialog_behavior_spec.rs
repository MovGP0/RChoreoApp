use crate::choreo_main::Report;
use crate::choreo_main::actions::ChoreoMainAction;
use crate::choreo_main::reducer::reduce;
use crate::choreo_main::state::ChoreoMainState;

#[test]
fn hide_dialog_behavior_spec() {
    let suite = rspec::describe("hide dialog reducer behavior", (), |spec| {
        spec.it("closes dialog when hide action is dispatched", |_| {
            let mut state = ChoreoMainState::default();
            reduce(
                &mut state,
                ChoreoMainAction::ShowDialog {
                    content: Some("hello".to_string()),
                },
            );
            reduce(&mut state, ChoreoMainAction::HideDialog);

            assert!(!state.is_dialog_open);
            assert!(state.dialog_content.is_none());
        });
    });

    let report = crate::choreo_main::run_suite(&suite);
    assert!(report.is_success());
}
