use crate::dancers;
use dancers::Report;

#[test]
fn cancel_dancer_settings_behavior_spec() {
    let suite = rspec::describe("cancel dancer settings behavior", (), |spec| {
        spec.it("does not mutate dialog state", |_| {
            let mut state = dancers::state::DancersState {
                is_dialog_open: true,
                dialog_content: Some("swap_dancers".to_string()),
                ..dancers::state::DancersState::default()
            };

            dancers::reducer::reduce(&mut state, dancers::actions::DancersAction::Cancel);

            assert!(state.is_dialog_open);
            assert_eq!(state.dialog_content.as_deref(), Some("swap_dancers"));
        });
    });
    let report = dancers::run_suite(&suite);
    assert!(report.is_success());
}
