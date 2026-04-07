use crate::floor::floor_component::actions::FloorAction;
use crate::floor::floor_component::reducer::reduce;
use crate::floor::floor_component::state::FloorPosition;
use crate::floor::floor_component::state::FloorState;
use crate::floor::floor_component::state::Point;

macro_rules! check_close {
    ($errors:expr, $left:expr, $right:expr) => {
        if ($left - $right).abs() >= 0.0001 {
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
fn move_positions_moves_selected_items_and_supports_grid_snap() {
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
        FloorAction::MoveSelectedByDelta {
            delta_x: 1.5,
            delta_y: -1.0,
        },
    );

    let mut errors = Vec::new();
    check_close!(errors, state.positions[0].x, 0.5);
    check_close!(errors, state.positions[0].y, 0.0);
    check_close!(errors, state.positions[1].x, 2.5);
    check_close!(errors, state.positions[1].y, 0.0);
    check_close!(errors, state.positions[2].x, 3.0);
    check_close!(errors, state.positions[2].y, -2.0);

    reduce(
        &mut state,
        FloorAction::SetSnapToGrid {
            enabled: true,
            resolution: 4,
        },
    );
    reduce(
        &mut state,
        FloorAction::MoveSelectedByDelta {
            delta_x: -0.3,
            delta_y: 0.0,
        },
    );

    check_close!(errors, state.positions[0].x, 0.25);
    check_close!(errors, state.positions[1].x, 2.25);

    assert_no_errors(errors);
}
