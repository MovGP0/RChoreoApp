mod floor;

use floor::Report;
use choreo_components::floor::{Point, RotateAroundCenterBehavior};
use choreo_components::global::InteractionMode;
use choreo_state_machine::RotateAroundCenterStartedTrigger;

fn setup_context() -> (floor::FloorTestContext, RotateAroundCenterBehavior) {
    let mut context = floor::FloorTestContext::new();
    context.configure_canvas();

    let (choreography, scene) = floor::build_three_position_choreography();
    let scene_view_model = floor::map_scene_view_model(&scene);
    context.global_state.choreography = choreography;
    context.global_state.selected_scene = Some(scene_view_model.clone());
    context.global_state.scenes = vec![scene_view_model];
    context.global_state.interaction_mode = InteractionMode::RotateAroundCenter;
    let _ = context.state_machine.try_apply(&RotateAroundCenterStartedTrigger);

    (context, RotateAroundCenterBehavior::default())
}

fn select_rectangle(
    behavior: &mut RotateAroundCenterBehavior,
    context: &mut floor::FloorTestContext,
    start: Point,
    end: Point,
) {
    let view_start = floor::floor_to_view_point(&context.view_model, &context.global_state.choreography, start);
    let view_end = floor::floor_to_view_point(&context.view_model, &context.global_state.choreography, end);
    behavior.handle_pointer_pressed(&context.view_model, &mut context.global_state, &mut context.state_machine, floor::pointer_pressed(view_start));
    behavior.handle_pointer_moved(&context.view_model, &mut context.global_state, &mut context.state_machine, floor::pointer_moved(view_end));
    behavior.handle_pointer_released(&context.view_model, &mut context.global_state, &mut context.state_machine, floor::pointer_released(view_end));
}

fn drag_from_to(
    behavior: &mut RotateAroundCenterBehavior,
    context: &mut floor::FloorTestContext,
    start: Point,
    end: Point,
) {
    let view_start = floor::floor_to_view_point(&context.view_model, &context.global_state.choreography, start);
    let view_end = floor::floor_to_view_point(&context.view_model, &context.global_state.choreography, end);
    behavior.handle_pointer_pressed(&context.view_model, &mut context.global_state, &mut context.state_machine, floor::pointer_pressed(view_start));
    behavior.handle_pointer_moved(&context.view_model, &mut context.global_state, &mut context.state_machine, floor::pointer_moved(view_end));
    behavior.handle_pointer_released(&context.view_model, &mut context.global_state, &mut context.state_machine, floor::pointer_released(view_end));
}

#[test]
fn rotate_around_center_behavior_spec() {
    let suite = rspec::describe("rotate around center behavior", (), |spec| {
        spec.it("rotates selected positions around center", |_| {
            let (mut context, mut behavior) = setup_context();
            select_rectangle(&mut behavior, &mut context, Point::new(-2.0, 2.0), Point::new(2.0, 0.0));
            drag_from_to(&mut behavior, &mut context, Point::new(0.0, 2.0), Point::new(1.0, 1.0));

            let scene = context.global_state.selected_scene.as_ref().expect("scene");
            let first = &scene.positions[0];
            let second = &scene.positions[1];
            let third = &scene.positions[2];

            floor::assert_close(first.x, 0.0, 0.0001);
            floor::assert_close(first.y, 2.0, 0.0001);
            floor::assert_close(second.x, 0.0, 0.0001);
            floor::assert_close(second.y, 0.0, 0.0001);
            floor::assert_close(third.x, 3.0, 0.0001);
            floor::assert_close(third.y, -2.0, 0.0001);
        });
    });

    let report = floor::run_suite(&suite);
    assert!(report.is_success());
}
