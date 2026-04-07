use crate::floor::floor_component::actions::FloorAction;
use crate::floor::floor_component::reducer::reduce;
use crate::floor::floor_component::state::FloorState;
use crate::floor::floor_component::state::Point;

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
fn place_position_adds_new_position_at_click_location() {
    let mut state = FloorState {
        is_place_mode: true,
        ..FloorState::default()
    };
    reduce(
        &mut state,
        FloorAction::PlacePosition {
            point: Point::new(1.0, 1.0),
        },
    );

    let mut errors = Vec::new();
    let position = state.positions.first();

    check_eq!(errors, state.positions.len(), 1);
    check!(errors, position.is_some());
    if let Some(position) = position {
        check!(errors, (position.x - 1.0).abs() < 0.0001);
        check!(errors, (position.y - 1.0).abs() < 0.0001);
    }

    assert_no_errors(errors);
}

#[test]
fn place_position_maps_floor_origin_with_asymmetric_bounds() {
    let mut state = FloorState {
        is_place_mode: true,
        floor_left: 10,
        floor_right: 5,
        floor_front: 7,
        floor_back: 3,
        ..FloorState::default()
    };
    reduce(
        &mut state,
        FloorAction::PlacePosition {
            point: Point::new(0.0, 0.0),
        },
    );

    let mut errors = Vec::new();
    let position = state.positions.first();

    check_eq!(errors, state.positions.len(), 1);
    check!(errors, position.is_some());
    if let Some(position) = position {
        check!(errors, (position.x - 0.0).abs() < 0.0001);
        check!(errors, (position.y - 0.0).abs() < 0.0001);
    }

    assert_no_errors(errors);
}
