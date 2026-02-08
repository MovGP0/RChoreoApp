use crate::floor;

use floor::Report;
use choreo_components::floor::Point;
use choreo_components::global::InteractionMode;
use choreo_state_machine::ScaleAroundDancerStartedTrigger;
use std::time::Duration;

fn setup_context() -> floor::FloorTestContext {
    let context = floor::FloorTestContext::new_without_gesture();
    context.configure_canvas();

    let (choreography, scene) = floor::build_three_position_choreography();
    let scene_view_model = floor::map_scene_view_model(&scene);

    context.update_global_state(|state| {
        state.choreography = choreography;
        state.selected_scene = Some(scene_view_model.clone());
        state.scenes = vec![scene_view_model];
        state.interaction_mode = InteractionMode::RotateAroundDancer;
    });

    context.update_state_machine(|state_machine| {
        let _ = state_machine.try_apply(&ScaleAroundDancerStartedTrigger);
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

fn double_tap(context: &floor::FloorTestContext, point: Point) {
    let view_point = floor::floor_to_view_point(context, point);
    context.send_pointer_pressed(view_point);
    context.send_pointer_released(view_point);
    floor::sleep_ms(100);
    context.send_pointer_pressed(view_point);
    context.send_pointer_released(view_point);
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
fn scale_around_dancer_behavior_spec() {
    let suite = rspec::describe("scale around dancer behavior", (), |spec| {
        spec.it("rotates around tapped dancer", |_| {
            let context = setup_context();
            select_rectangle(&context, Point::new(-2.0, 2.0), Point::new(2.0, 0.0));
            double_tap(&context, Point::new(-1.0, 1.0));
            drag_from_to(&context, Point::new(-1.0, 2.0), Point::new(0.0, 1.0));

            let rotated = context.wait_until(Duration::from_secs(1), || {
                let scene = context.read_global_state(|state| state.selected_scene.clone().expect("scene"));
                let first = &scene.positions[0];
                let second = &scene.positions[1];
                let third = &scene.positions[2];
                (first.x - -1.0).abs() < 0.0001
                    && (first.y - 1.0).abs() < 0.0001
                    && (second.x - -1.0).abs() < 0.0001
                    && (second.y - -1.0).abs() < 0.0001
                    && (third.x - 3.0).abs() < 0.0001
                    && (third.y - -2.0).abs() < 0.0001
            });
            assert!(rotated);
        });

        spec.it("rotates around tapped dancer with mouse", |_| {
            let context = setup_context();
            select_rectangle(&context, Point::new(-2.0, 2.0), Point::new(2.0, 0.0));
            double_tap(&context, Point::new(-1.0, 1.0));
            drag_from_to(&context, Point::new(-1.0, 2.0), Point::new(0.0, 1.0));

            let rotated = context.wait_until(Duration::from_secs(1), || {
                let scene = context.read_global_state(|state| state.selected_scene.clone().expect("scene"));
                let second = &scene.positions[1];
                (second.x - -1.0).abs() < 0.0001 && (second.y - -1.0).abs() < 0.0001
            });
            assert!(rotated);
        });
    });

    let report = floor::run_suite(&suite);
    assert!(report.is_success());
}
