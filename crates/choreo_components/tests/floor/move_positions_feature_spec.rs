use crate::floor;
use crate::floor::floor_component::actions::FloorAction;
use crate::floor::floor_component::reducer::reduce;
use crate::floor::floor_component::state::FloorPosition;
use crate::floor::floor_component::state::FloorState;
use crate::floor::floor_component::state::Point;

use floor::Report;

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
    state
}

fn select_rectangle(state: &mut FloorState, start: Point, end: Point) {
    reduce(state, FloorAction::SelectRectangle { start, end });
}

fn move_selected(state: &mut FloorState, delta_x: f64, delta_y: f64) {
    reduce(state, FloorAction::MoveSelectedByDelta { delta_x, delta_y });
}

fn approx_eq(actual: f64, expected: f64) -> bool {
    (actual - expected).abs() < 0.0001
}

#[test]
fn move_positions_feature_spec() {
    let suite = rspec::describe("move positions feature", (), |spec| {
        spec.it("moves all selected positions by drag delta", |_| {
            let mut state = setup_state();

            select_rectangle(&mut state, Point::new(-2.0, 2.0), Point::new(2.0, 0.0));
            move_selected(&mut state, 1.5, -1.0);

            let mut errors = Vec::new();

            check!(errors, approx_eq(state.positions[0].x, 0.5));
            check!(errors, approx_eq(state.positions[0].y, 0.0));
            check!(errors, approx_eq(state.positions[1].x, 2.5));
            check!(errors, approx_eq(state.positions[1].y, 0.0));
            check!(errors, approx_eq(state.positions[2].x, 3.0));
            check!(errors, approx_eq(state.positions[2].y, -2.0));

            assert_no_errors(errors);
        });

        spec.it("clears selection when clicking outside", |_| {
            let mut state = setup_state();

            select_rectangle(&mut state, Point::new(-2.0, 2.0), Point::new(2.0, 0.0));
            reduce(&mut state, FloorAction::ClearSelection);

            let mut errors = Vec::new();

            check_eq!(errors, state.selected_positions.len(), 0);
            check!(errors, state.selection_rectangle.is_none());

            assert_no_errors(errors);
        });

        spec.it("moves a single position when dragging", |_| {
            let mut state = setup_state();
            state.selected_positions = vec![0];

            move_selected(&mut state, -1.0, 2.0);

            let mut errors = Vec::new();

            check!(errors, approx_eq(state.positions[0].x, -2.0));
            check!(errors, approx_eq(state.positions[0].y, 3.0));
            check!(errors, approx_eq(state.positions[1].x, 1.0));
            check!(errors, approx_eq(state.positions[1].y, 1.0));
            check!(errors, approx_eq(state.positions[2].x, 3.0));
            check!(errors, approx_eq(state.positions[2].y, -2.0));
            check_eq!(errors, state.selected_positions.len(), 1);

            assert_no_errors(errors);
        });

        spec.it("selects positions with mouse drag rectangle", |_| {
            let mut state = setup_state();

            select_rectangle(&mut state, Point::new(-2.0, 2.0), Point::new(2.0, 0.0));

            let mut errors = Vec::new();

            check_eq!(errors, state.selected_positions.len(), 2);
            check!(errors, state.selected_positions.contains(&0));
            check!(errors, state.selected_positions.contains(&1));
            check!(errors, !state.selected_positions.contains(&2));

            assert_no_errors(errors);
        });

        spec.it(
            "selects positions with mouse drag rectangle after translation",
            |_| {
                let mut state = setup_state();
                state.transformation_matrix.translate(10.0, -12.0);

                select_rectangle(&mut state, Point::new(-2.0, 2.0), Point::new(2.0, 0.0));

                let mut errors = Vec::new();

                check_eq!(errors, state.selected_positions.len(), 2);
                check!(errors, state.selected_positions.contains(&0));
                check!(errors, state.selected_positions.contains(&1));
                check!(errors, !state.selected_positions.contains(&2));

                assert_no_errors(errors);
            },
        );
    });

    let report = floor::run_suite(&suite);
    assert!(report.is_success());
}
