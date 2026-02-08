use crate::floor;

use floor::Report;
use choreo_components::floor::Point;
use choreo_components::global::InteractionMode;
use choreo_state_machine::ScaleAroundDancerStartedTrigger;

fn setup_context() -> floor::FloorTestContext {
    let context = floor::FloorTestContext::new();
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

fn double_tap_touch(context: &floor::FloorTestContext, point: Point) {
    let view_point = floor::floor_to_view_point(context, point);
    context.send_touch(1, choreo_components::floor::TouchAction::Pressed, view_point, true);
    context.send_touch(1, choreo_components::floor::TouchAction::Released, view_point, false);
    floor::sleep_ms(100);
    context.send_touch(1, choreo_components::floor::TouchAction::Pressed, view_point, true);
    context.send_touch(1, choreo_components::floor::TouchAction::Released, view_point, false);
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
fn scale_around_dancer_behavior_spec() {
    let suite = rspec::describe("scale around dancer behavior", (), |spec| {
        spec.it("rotates around tapped dancer", |_| {
            let context = setup_context();
            select_rectangle(&context, Point::new(-2.0, 2.0), Point::new(2.0, 0.0));
            double_tap_touch(&context, Point::new(-1.0, 1.0));
            drag_touch_from_to(&context, Point::new(-1.0, 2.0), Point::new(0.0, 1.0));

            let scene = context.read_global_state(|state| state.selected_scene.clone().expect("scene"));
            let first = &scene.positions[0];
            let second = &scene.positions[1];
            let third = &scene.positions[2];

            floor::assert_close(first.x, -1.0, 0.0001);
            floor::assert_close(first.y, 1.0, 0.0001);
            floor::assert_close(second.x, -1.0, 0.0001);
            floor::assert_close(second.y, -1.0, 0.0001);
            floor::assert_close(third.x, 3.0, 0.0001);
            floor::assert_close(third.y, -2.0, 0.0001);
        });

        spec.it("rotates around tapped dancer with mouse", |_| {
            let context = setup_context();
            select_rectangle(&context, Point::new(-2.0, 2.0), Point::new(2.0, 0.0));
            double_tap(&context, Point::new(-1.0, 1.0));
            drag_from_to(&context, Point::new(-1.0, 2.0), Point::new(0.0, 1.0));

            let scene = context.read_global_state(|state| state.selected_scene.clone().expect("scene"));
            let second = &scene.positions[1];
            floor::assert_close(second.x, -1.0, 0.0001);
            floor::assert_close(second.y, -1.0, 0.0001);
        });
    });

    let report = floor::run_suite(&suite);
    assert!(report.is_success());
}
