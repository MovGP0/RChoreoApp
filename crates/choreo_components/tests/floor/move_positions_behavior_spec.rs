use crate::floor;

use choreo_components::floor::Point;
use choreo_components::global::InteractionMode;
use choreo_state_machine::MovePositionsStartedTrigger;
use floor::Report;
use std::time::Duration;

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

    context.update_state_machine(|state_machine| {
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

#[test]
#[serial_test::serial]
fn move_positions_behavior_spec() {
    let suite = rspec::describe("move positions behavior", (), |spec| {
        spec.it("moves selected positions by drag delta", |_| {
            let context = setup_context();
            let start_first = Point::new(-1.0, 1.0);
            let start_second = Point::new(1.0, 1.0);

            select_rectangle(&context, Point::new(-2.0, 2.0), Point::new(2.0, 0.0));
            drag_from_to(
                &context,
                start_first,
                Point::new(start_first.x + 1.5, start_first.y - 1.0),
            );

            let moved = context.wait_until(Duration::from_secs(1), || {
                let scene =
                    context.read_global_state(|state| state.selected_scene.clone().expect("scene"));
                let first = &scene.positions[0];
                let second = &scene.positions[1];
                (first.x - (start_first.x + 1.5)).abs() < 0.0001
                    && (first.y - (start_first.y - 1.0)).abs() < 0.0001
                    && (second.x - (start_second.x + 1.5)).abs() < 0.0001
                    && (second.y - (start_second.y - 1.0)).abs() < 0.0001
            });
            assert!(moved);
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

            let moved = context.wait_until(Duration::from_secs(1), || {
                let scene =
                    context.read_global_state(|state| state.selected_scene.clone().expect("scene"));
                let first = &scene.positions[0];
                let second = &scene.positions[1];
                (first.x - (start_first.x + 1.5)).abs() < 0.0001
                    && (first.y - (start_first.y - 1.0)).abs() < 0.0001
                    && (second.x - (start_second.x + 1.5)).abs() < 0.0001
                    && (second.y - (start_second.y - 1.0)).abs() < 0.0001
            });
            assert!(moved);
        });

        spec.it("clears selection when clicking outside", |_| {
            let context = setup_context();
            select_rectangle(&context, Point::new(-2.0, 2.0), Point::new(2.0, 0.0));

            drag_from_to(&context, Point::new(4.0, 4.0), Point::new(4.0, 4.0));

            let cleared = context.wait_until(Duration::from_secs(1), || {
                let selected_count =
                    context.read_global_state(|state| state.selected_positions.len());
                let has_rectangle =
                    context.read_global_state(|state| state.selection_rectangle.is_some());
                selected_count == 0 && !has_rectangle
            });
            assert!(cleared);
        });
    });

    let report = floor::run_suite(&suite);
    assert!(report.is_success());
}
