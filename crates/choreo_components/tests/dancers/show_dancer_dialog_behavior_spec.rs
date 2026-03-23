use crate::dancers;
use dancers::Report;

#[test]
fn show_dancer_dialog_behavior_spec() {
    let suite = rspec::describe("show dancer dialog behavior", (), |spec| {
        spec.it("applies dialog visibility from payload", |_| {
            let mut state = dancers::state::DancersState::default();

            dancers::reducer::reduce(
                &mut state,
                dancers::actions::DancersAction::ShowDialog {
                    content_id: Some("swap_dancers".to_string()),
                },
            );
            assert!(state.is_dialog_open);
            assert_eq!(state.dialog_content.as_deref(), Some("swap_dancers"));

            dancers::reducer::reduce(
                &mut state,
                dancers::actions::DancersAction::ShowDialog { content_id: None },
            );
            assert!(!state.is_dialog_open);
            assert!(state.dialog_content.is_none());
        });
    });
    let report = dancers::run_suite(&suite);
    assert!(report.is_success());
}
