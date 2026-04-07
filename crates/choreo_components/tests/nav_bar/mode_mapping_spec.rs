use crate::nav_bar::nav_bar_component::state::InteractionMode;
use crate::nav_bar::nav_bar_component::state::mode_index;
use crate::nav_bar::nav_bar_component::state::mode_option_from_index;

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

fn assert_no_errors(errors: Vec<String>) {
    assert!(
        errors.is_empty(),
        "Assertion failures:\n{}",
        errors.join("\n")
    );
}

#[test]
fn mode_mapping_round_trips_all_modes() {
    let modes = [
        InteractionMode::View,
        InteractionMode::Move,
        InteractionMode::RotateAroundCenter,
        InteractionMode::RotateAroundDancer,
        InteractionMode::Scale,
        InteractionMode::LineOfSight,
    ];

    let mut errors = Vec::new();

    for mode in modes {
        let index = mode_index(mode);
        check!(errors, index >= 0);
        check_eq!(errors, mode_option_from_index(index), Some(mode));
    }

    check_eq!(errors, mode_option_from_index(-1), None::<InteractionMode>);
    check_eq!(errors, mode_option_from_index(99), None::<InteractionMode>);

    assert_no_errors(errors);
}
