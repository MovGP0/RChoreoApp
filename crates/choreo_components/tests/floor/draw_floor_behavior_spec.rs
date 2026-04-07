use crate::floor::floor_component::actions::FloorAction;
use crate::floor::floor_component::reducer::reduce;
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
fn draw_floor_marks_render_gate_only_once_while_counting_draws() {
    let mut state = FloorState::default();

    reduce(&mut state, FloorAction::DrawFloor);
    reduce(&mut state, FloorAction::DrawFloor);

    let mut errors = Vec::new();

    check_eq!(errors, state.draw_count, 2);
    check!(errors, state.render_marked);
    check_eq!(errors, state.render_mark_count, 1);

    assert_no_errors(errors);
}

#[test]
fn draw_floor_without_action_keeps_render_gate_unmarked() {
    let state = FloorState::default();

    let mut errors = Vec::new();

    check_eq!(errors, state.draw_count, 0);
    check!(errors, !state.render_marked);
    check_eq!(errors, state.render_mark_count, 0);

    assert_no_errors(errors);
}
