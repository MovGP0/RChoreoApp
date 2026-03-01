use crate::choreo_main::Report;
use crate::choreo_main::actions::ChoreoMainAction;
use crate::choreo_main::reducer::reduce;
use crate::choreo_main::state::ChoreoMainState;
use crate::choreo_main::state::MainContent;

#[test]
fn navigate_settings_to_main_spec() {
    let suite = rspec::describe("navigate settings to main", (), |spec| {
        spec.it("returns to main from settings", |_| {
            let mut state = ChoreoMainState::default();
            reduce(&mut state, ChoreoMainAction::NavigateToSettings);
            reduce(&mut state, ChoreoMainAction::NavigateToMain);

            assert_eq!(state.content, MainContent::Main);
        });
    });

    let report = crate::choreo_main::run_suite(&suite);
    assert!(report.is_success());
}
