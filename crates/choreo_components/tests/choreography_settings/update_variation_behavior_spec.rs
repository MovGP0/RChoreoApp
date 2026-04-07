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

macro_rules! check {
    ($errors:expr, $condition:expr) => {
        if !$condition {
            $errors.push(format!("condition failed: {}", stringify!($condition)));
        }
    };
}

#[test]
fn update_variation_trims_and_sets_optional_variation() {
    let mut state = create_state();

    reduce(
        &mut state,
        ChoreographySettingsAction::UpdateVariation("  alt  ".to_string()),
    );

    let mut errors = Vec::new();
    check_eq!(errors, state.choreography.variation.as_deref(), Some("alt"));
    check_eq!(errors, state.variation, "alt");
    check!(errors, state.redraw_requested);
    assert!(
        errors.is_empty(),
        "Assertion failures:\n{}",
        errors.join("\n")
    );
}
