use super::actions::ChoreographySettingsAction;
use super::create_state;
use super::reducer::reduce;

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
fn update_subtitle_trims_and_sets_optional_subtitle() {
    let mut state = create_state();

    reduce(
        &mut state,
        ChoreographySettingsAction::UpdateSubtitle("  Subtitle  ".to_string()),
    );

    let mut errors = Vec::new();

    check_eq!(errors, state.choreography.subtitle.as_deref(), Some("Subtitle"));
    check_eq!(errors, state.subtitle.as_str(), "Subtitle");
    check_eq!(errors, state.redraw_requested, true);

    assert_no_errors(errors);
}
