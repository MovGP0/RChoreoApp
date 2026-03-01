use crate::choreo_main::Report;
use crate::choreo_main::actions::ChoreoMainAction;
use crate::choreo_main::reducer::reduce;
use crate::choreo_main::state::ChoreoMainState;
use crate::choreo_main::state::MainContent;

#[test]
fn navigate_dancers_to_main_spec() {
    let suite = rspec::describe("navigate dancers to main", (), |spec| {
        spec.it(
            "returns to main page when dancer settings cancel is requested",
            |_| {
                let mut state = ChoreoMainState::default();
                reduce(&mut state, ChoreoMainAction::NavigateToDancers);
                reduce(&mut state, ChoreoMainAction::NavigateToMain);

                assert_eq!(state.content, MainContent::Main);
            },
        );

        spec.it("returns to main page when dancer settings save is requested", |_| {
            let mut state = ChoreoMainState::default();
            reduce(&mut state, ChoreoMainAction::NavigateToDancers);
            reduce(&mut state, ChoreoMainAction::NavigateToMain);

            assert_eq!(state.content, MainContent::Main);
        });
    });

    let report = crate::choreo_main::run_suite(&suite);
    assert!(report.is_success());
}
