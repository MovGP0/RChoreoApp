use super::actions::ChoreographySettingsAction;
use super::color;
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
fn update_floor_color_updates_choreography_and_view_state() {
    let mut state = create_state();
    let floor_color = color(255, 12, 34, 56);

    reduce(
        &mut state,
        ChoreographySettingsAction::UpdateFloorColor(floor_color.clone()),
    );

    let mut errors = Vec::new();

    check_eq!(errors, state.floor_color, floor_color);
    check_eq!(
        errors,
        state.choreography.settings.floor_color,
        state.floor_color
    );
    check_eq!(errors, state.redraw_requested, true);

    assert_no_errors(errors);
}
