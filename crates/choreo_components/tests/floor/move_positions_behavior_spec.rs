use crate::floor;

use floor::Report;
use choreo_components::floor::Point;
use choreo_components::global::InteractionMode;
use choreo_state_machine::{ApplicationStateMachine, MovePositionsStartedTrigger};

fn setup_context() -> floor::FloorTestContext {
    let context = floor::FloorTestContext::new();
    context.configure_canvas();

    let (choreography, scene) = floor::build_three_position_choreography();
    let scene_view_model = floor::map_scene_view_model(&scene);

    context.update_global_state(|state| {
        state.choreography = choreography;
        state.selected_scene = Some(scene_view_model.clone());
        state.scenes = vec![scene_view_model];
        state.interaction_mode = InteractionMode::Move;
    });

    context.update_state_machine(|state_machine: &mut ApplicationStateMachine| {
        let _ = state_machine.try_apply(&MovePositionsStartedTrigger);
    });

    context
}

fn select_rectangle(context: &floor::FloorTestContext, start: Point, end: Point) {
    let view_start = floor::floor_to_view_point(context, start);
    let view_end = floor::floor_to_view_point(context, end);
    context.send_pointer_pressed(view_start);
    context.send_pointer_moved(view_end);
    context.send_pointer_released(view_end);
}

fn drag_from_to(context: &floor::FloorTestContext, start: Point, end: Point) {
    let view_start = floor::floor_to_view_point(context, start);
    let view_end = floor::floor_to_view_point(context, end);
    context.send_pointer_pressed(view_start);
    context.send_pointer_moved(view_end);
    context.send_pointer_released(view_end);
}

fn drag_touch_from_to(context: &floor::FloorTestContext, start: Point, end: Point) {
    let view_start = floor::floor_to_view_point(context, start);
    let view_end = floor::floor_to_view_point(context, end);
    context.send_touch(1, choreo_components::floor::TouchAction::Pressed, view_start, true);
    context.send_touch(1, choreo_components::floor::TouchAction::Moved, view_end, true);
    context.send_touch(1, choreo_components::floor::TouchAction::Released, view_end, false);
}

#[test]
fn move_positions_behavior_spec() {
    let suite = rspec::describe("move positions behavior", (), |spec| {
        spec.it("moves selected positions by drag delta", |_| {
            let context = setup_context();
            let start_first = Point::new(-1.0, 1.0);
            let start_second = Point::new(1.0, 1.0);

            select_rectangle(&context, Point::new(-2.0, 2.0), Point::new(2.0, 0.0));
            drag_touch_from_to(
                &context,
                start_first,
                Point::new(start_first.x + 1.5, start_first.y - 1.0),
            );

            let scene = context.read_global_state(|state| state.selected_scene.clone().expect("scene"));
            let first = &scene.positions[0];
            let second = &scene.positions[1];
            floor::assert_close(first.x, start_first.x + 1.5, 0.0001);
            floor::assert_close(first.y, start_first.y - 1.0, 0.0001);
            floor::assert_close(second.x, start_second.x + 1.5, 0.0001);
            floor::assert_close(second.y, start_second.y - 1.0, 0.0001);
        });

        spec.it("moves selected positions by drag delta with mouse", |_| {
            let context = setup_context();
            let start_first = Point::new(-1.0, 1.0);
            let start_second = Point::new(1.0, 1.0);

            select_rectangle(&context, Point::new(-2.0, 2.0), Point::new(2.0, 0.0));
            drag_from_to(
                &context,
                start_first,
                Point::new(start_first.x + 1.5, start_first.y - 1.0),
            );

            let scene = context.read_global_state(|state| state.selected_scene.clone().expect("scene"));
            let first = &scene.positions[0];
            let second = &scene.positions[1];
            floor::assert_close(first.x, start_first.x + 1.5, 0.0001);
            floor::assert_close(first.y, start_first.y - 1.0, 0.0001);
            floor::assert_close(second.x, start_second.x + 1.5, 0.0001);
            floor::assert_close(second.y, start_second.y - 1.0, 0.0001);
        });

        spec.it("clears selection when clicking outside", |_| {
            let context = setup_context();
            select_rectangle(&context, Point::new(-2.0, 2.0), Point::new(2.0, 0.0));

            drag_from_to(&context, Point::new(4.0, 4.0), Point::new(4.0, 4.0));

            let selected_count = context.read_global_state(|state| state.selected_positions.len());
            let has_rectangle = context.read_global_state(|state| state.selection_rectangle.is_some());

            assert_eq!(selected_count, 0);
            assert!(!has_rectangle);
        });
    });

    let report = floor::run_suite(&suite);
    assert!(report.is_success());
}
