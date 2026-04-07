use crate::floor::floor_component::actions::FloorAction;
use crate::floor::floor_component::reducer::reduce;
use crate::floor::floor_component::state::FloorPosition;
use crate::floor::floor_component::state::FloorState;
use crate::floor::floor_component::state::Point;

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
fn rotate_around_center_rotates_selected_positions() {
    let mut state = FloorState::default();
    reduce(
        &mut state,
        FloorAction::SetPositions {
            positions: vec![
                FloorPosition::new(-1.0, 1.0),
                FloorPosition::new(1.0, 1.0),
                FloorPosition::new(3.0, -2.0),
            ],
        },
    );
    reduce(
        &mut state,
        FloorAction::SelectRectangle {
            start: Point::new(-2.0, 2.0),
            end: Point::new(2.0, 0.0),
        },
    );
    reduce(
        &mut state,
        FloorAction::RotateSelectedAroundCenter {
            start: Point::new(0.0, 2.0),
            end: Point::new(1.0, 1.0),
        },
    );

    let mut errors = Vec::new();

    check!(errors, (state.positions[0].x - 0.0).abs() < 0.0001);
    check!(errors, (state.positions[0].y - 2.0).abs() < 0.0001);
    check!(errors, (state.positions[1].x - 0.0).abs() < 0.0001);
    check!(errors, (state.positions[1].y - 0.0).abs() < 0.0001);
    check!(errors, (state.positions[2].x - 3.0).abs() < 0.0001);
    check!(errors, (state.positions[2].y + 2.0).abs() < 0.0001);

    assert_no_errors(errors);
}
