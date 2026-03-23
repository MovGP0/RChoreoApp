use crate::floor;
use crate::floor::Report;
use crate::floor::floor_component::actions::FloorAction;
use crate::floor::floor_component::reducer::reduce;
use crate::floor::floor_component::state::FloorPosition;
use crate::floor::floor_component::state::FloorState;
use crate::floor::floor_component::state::Point;

fn setup_state() -> FloorState {
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
        FloorAction::SetPivotFromPoint {
            point: Point::new(-1.0, 1.0),
        },
    );

    state
}

fn rotate_selected_positions(state: &mut FloorState) {
    reduce(
        state,
        FloorAction::RotateSelectedAroundPivot {
            start: Point::new(-1.0, 2.0),
            end: Point::new(0.0, 1.0),
        },
    );
}

fn assert_expected_positions(state: &FloorState) {
    assert!((state.positions[0].x + 1.0).abs() < 0.0001);
    assert!((state.positions[0].y - 1.0).abs() < 0.0001);
    assert!((state.positions[1].x + 1.0).abs() < 0.0001);
    assert!((state.positions[1].y + 1.0).abs() < 0.0001);
    assert!((state.positions[2].x - 3.0).abs() < 0.0001);
    assert!((state.positions[2].y + 2.0).abs() < 0.0001);
}

#[test]
fn scale_around_dancer_behavior_spec() {
    let suite = rspec::describe("scale around dancer behavior", (), |spec| {
        spec.it("rotates around tapped dancer", |_| {
            let mut state = setup_state();
            rotate_selected_positions(&mut state);
            assert_expected_positions(&state);
        });

        spec.it("rotates around tapped dancer with mouse", |_| {
            let mut state = setup_state();
            rotate_selected_positions(&mut state);
            assert_expected_positions(&state);
        });
    });

    let report = floor::run_suite(&suite);
    assert!(report.is_success());
}
