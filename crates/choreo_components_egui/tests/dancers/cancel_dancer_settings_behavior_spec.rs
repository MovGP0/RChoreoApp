use crate::dancers;
use dancers::Report;

#[test]
fn cancel_dancer_settings_behavior_spec() {
    let suite = rspec::describe("cancel dancer settings behavior", (), |spec| {
        spec.it("closes dialog without swapping dancers", |_| {
            let mut state = dancers::state::DancersState {
                is_dialog_open: true,
                dialog_content: Some("swap_dancers".to_string()),
                ..dancers::state::DancersState::default()
            };

            dancers::reducer::reduce(&mut state, dancers::actions::DancersAction::Cancel);

            assert!(!state.is_dialog_open);
            assert!(state.dialog_content.is_none());
        });
    });
    let report = dancers::run_suite(&suite);
    assert!(report.is_success());
}
