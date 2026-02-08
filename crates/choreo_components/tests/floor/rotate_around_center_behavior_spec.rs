use crate::floor;

use floor::Report;
use choreo_components::floor::Point;
use choreo_components::global::InteractionMode;
use choreo_state_machine::RotateAroundCenterStartedTrigger;
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
        state.interaction_mode = InteractionMode::RotateAroundCenter;
    });

    context.update_state_machine(|state_machine| {
        let _ = state_machine.try_apply(&RotateAroundCenterStartedTrigger);
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
fn rotate_around_center_behavior_spec() {
    let suite = rspec::describe("rotate around center behavior", (), |spec| {
        spec.it("rotates selected positions around center", |_| {
            let context = setup_context();
            select_rectangle(&context, Point::new(-2.0, 2.0), Point::new(2.0, 0.0));
            drag_from_to(&context, Point::new(0.0, 2.0), Point::new(1.0, 1.0));

            let rotated = context.wait_until(Duration::from_secs(1), || {
                let scene = context.read_global_state(|state| state.selected_scene.clone().expect("scene"));
                let first = &scene.positions[0];
                let second = &scene.positions[1];
                let third = &scene.positions[2];
                (first.x - 0.0).abs() < 0.0001
                    && (first.y - 2.0).abs() < 0.0001
                    && (second.x - 0.0).abs() < 0.0001
                    && (second.y - 0.0).abs() < 0.0001
                    && (third.x - 3.0).abs() < 0.0001
                    && (third.y - -2.0).abs() < 0.0001
            });
            assert!(rotated);
        });
    });

    let report = floor::run_suite(&suite);
    assert!(report.is_success());
}
