use crate::floor;

use floor::Report;
use choreo_components::floor::Point;
use choreo_components::global::InteractionMode;
use choreo_state_machine::ScalePositionsStartedTrigger;

fn setup_context() -> floor::FloorTestContext {
    let context = floor::FloorTestContext::new();
    context.configure_canvas();

    let (choreography, scene) = floor::build_three_position_choreography();
    let scene_view_model = floor::map_scene_view_model(&scene);

    context.update_global_state(|state| {
        state.choreography = choreography;
        state.selected_scene = Some(scene_view_model.clone());
        state.scenes = vec![scene_view_model];
        state.interaction_mode = InteractionMode::Scale;
    });

    context.update_state_machine(|state_machine| {
        let _ = state_machine.try_apply(&ScalePositionsStartedTrigger);
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
fn scale_positions_behavior_spec() {
    let suite = rspec::describe("scale positions behavior", (), |spec| {
        spec.it("scales selected positions", |_| {
            let context = setup_context();
            select_rectangle(&context, Point::new(-2.0, 2.0), Point::new(2.0, 0.0));
            drag_from_to(&context, Point::new(2.0, 1.0), Point::new(4.0, 1.0));

            let scene = context.read_global_state(|state| state.selected_scene.clone().expect("scene"));
            let first = &scene.positions[0];
            let second = &scene.positions[1];
            let third = &scene.positions[2];

            floor::assert_close(first.x, -2.0, 0.0001);
            floor::assert_close(first.y, 1.0, 0.0001);
            floor::assert_close(second.x, 2.0, 0.0001);
            floor::assert_close(second.y, 1.0, 0.0001);
            floor::assert_close(third.x, 3.0, 0.0001);
            floor::assert_close(third.y, -2.0, 0.0001);
        });
    });

    let report = floor::run_suite(&suite);
    assert!(report.is_success());
}
