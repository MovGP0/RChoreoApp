use crate::choreo_main::Report;
use crate::choreo_main::actions::ChoreoMainAction;
use crate::choreo_main::reducer::reduce;
use crate::choreo_main::state::ChoreoMainState;
use crate::choreo_main::state::MainContent;

#[test]
fn navigate_main_to_dancers_spec() {
    let suite = rspec::describe("navigate from main page to dancers page", (), |spec| {
        spec.it("shows dancers when navigation to dancer settings is requested", |_| {
            let mut state = ChoreoMainState::default();

            assert_eq!(state.content, MainContent::Main);
            reduce(&mut state, ChoreoMainAction::NavigateToDancers);

            assert_eq!(state.content, MainContent::Dancers);
        });
    });

    let report = crate::choreo_main::run_suite(&suite);
    assert!(report.is_success());
}
