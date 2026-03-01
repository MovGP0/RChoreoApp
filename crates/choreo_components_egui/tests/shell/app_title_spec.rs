use crate::shell::Report;
use crate::shell::actions::ShellAction;
use crate::shell::reducer::reduce;
use crate::shell::state::ShellState;

#[test]
fn app_title_spec() {
    let suite = rspec::describe("shell app title", (), |spec| {
        spec.it("defaults to ChoreoApp title", |_| {
            let mut state = ShellState::default();
            reduce(&mut state, ShellAction::Initialize);
            assert_eq!(state.app_title, "ChoreoApp");
        });
    });

    let report = crate::shell::run_suite(&suite);
    assert!(report.is_success());
}
