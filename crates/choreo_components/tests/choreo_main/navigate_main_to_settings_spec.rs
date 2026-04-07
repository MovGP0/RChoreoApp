use crate::choreo_main::Report;
use crate::choreo_main::actions::ChoreoMainAction;
use crate::choreo_main::reducer::reduce;
use crate::choreo_main::state::ChoreoMainState;
use crate::choreo_main::state::MainContent;

macro_rules! check_eq {
    ($errors:expr, $left:expr, $right:expr) => {
        if $left != $right {
            $errors.push(format!(
                "{} != {} (left = {:?}, right = {:?})",
                stringify!($left),
                stringify!($right),
                $left,
                $right
            ));
        }
    };
}

fn assert_no_errors(errors: Vec<String>) {
    assert!(
        errors.is_empty(),
        "Assertion failures:\n{}",
        errors.join("\n")
    );
}

#[test]
fn navigate_main_to_settings_spec() {
    let suite = rspec::describe("navigate from main page to settings page", (), |spec| {
        spec.it(
            "shows settings content when settings navigation is requested",
            |_| {
                let mut state = ChoreoMainState::default();
                let mut errors = Vec::new();

                check_eq!(errors, state.content, MainContent::Main);

                reduce(&mut state, ChoreoMainAction::NavigateToSettings);

                check_eq!(errors, state.content, MainContent::Settings);

                assert_no_errors(errors);
            },
        );
    });

    let report = crate::choreo_main::run_suite(&suite);
    assert!(report.is_success());
}
