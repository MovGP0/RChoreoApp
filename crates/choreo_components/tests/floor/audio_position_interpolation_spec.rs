use crate::floor::floor_component::actions::FloorAction;
use crate::floor::floor_component::reducer::reduce;
use crate::floor::floor_component::state::FloorPosition;
use crate::floor::floor_component::state::FloorState;

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
fn audio_position_interpolation_updates_interpolated_positions() {
    let mut state = FloorState::default();
    reduce(
        &mut state,
        FloorAction::InterpolateAudioPosition {
            from: vec![FloorPosition::new(0.0, 0.0)],
            to: vec![FloorPosition::new(10.0, 0.0)],
            progress: 0.5,
        },
    );

    let mut errors = Vec::new();

    check_eq!(errors, state.interpolated_positions.len(), 1);
    check!(errors, (state.interpolated_positions[0].x - 5.0).abs() < 0.0001);
    check!(errors, (state.interpolated_positions[0].y - 0.0).abs() < 0.0001);

    assert_no_errors(errors);
}
