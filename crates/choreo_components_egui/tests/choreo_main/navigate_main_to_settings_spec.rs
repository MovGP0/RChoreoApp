use crate::choreo_main::Report;
use crate::choreo_main::actions::ChoreoMainAction;
use crate::choreo_main::reducer::reduce;
use crate::choreo_main::state::ChoreoMainState;
use crate::choreo_main::state::MainContent;

#[test]
fn navigate_main_to_settings_spec() {
    let suite = rspec::describe("navigate from main page to settings page", (), |spec| {
        spec.it(
            "shows settings content when settings navigation is requested",
            |_| {
            let mut state = ChoreoMainState::default();
            assert_eq!(state.content, MainContent::Main);

            reduce(&mut state, ChoreoMainAction::NavigateToSettings);

            assert_eq!(state.content, MainContent::Settings);
            },
        );
    });

    let report = crate::choreo_main::run_suite(&suite);
    assert!(report.is_success());
}
