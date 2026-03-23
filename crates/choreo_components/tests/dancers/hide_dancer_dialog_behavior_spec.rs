use crate::dancers;
use dancers::Report;

#[test]
fn hide_dancer_dialog_behavior_spec() {
    let suite = rspec::describe("hide dancer dialog behavior", (), |spec| {
        spec.it("clears dialog state", |_| {
            let mut state = dancers::state::DancersState::default();
            dancers::reducer::reduce(
                &mut state,
                dancers::actions::DancersAction::ShowDialog {
                    content_id: Some("swap_dancers".to_string()),
                },
            );
            assert!(state.is_dialog_open);

            dancers::reducer::reduce(&mut state, dancers::actions::DancersAction::HideDialog);
            assert!(!state.is_dialog_open);
            assert!(state.dialog_content.is_none());
        });
    });
    let report = dancers::run_suite(&suite);
    assert!(report.is_success());
}
