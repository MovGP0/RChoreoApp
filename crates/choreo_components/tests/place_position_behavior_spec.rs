mod common;

use common::Report;
use choreo_components::floor::{PlacePositionBehavior, Point};
use choreo_state_machine::PlacePositionsStartedTrigger;

#[test]
fn place_position_behavior_spec() {
    let suite = rspec::describe("place position behavior", (), |spec| {
        spec.it("places a new position on click", |_| {
            let mut context = common::FloorTestContext::new();
            context.configure_canvas();

            let (choreography, scene) = common::build_empty_scene_choreography();
            let scene_view_model = common::map_scene_view_model(&scene);
            context.global_state.choreography = choreography;
            context.global_state.selected_scene = Some(scene_view_model.clone());
            context.global_state.scenes = vec![scene_view_model];
            context.global_state.is_place_mode = true;
            let _ = context.state_machine.try_apply(&PlacePositionsStartedTrigger);

            let mut behavior = PlacePositionBehavior::default();
            let view_point = common::floor_to_view_point(&context.view_model, &context.global_state.choreography, Point::new(1.0, 1.0));
            behavior.handle_pointer_pressed(common::pointer_pressed(view_point));
            behavior.handle_pointer_released(&context.view_model, &mut context.global_state, &mut context.state_machine, common::pointer_released(view_point));

            let scene_view_model = context.global_state.selected_scene.as_ref().expect("scene");
            assert_eq!(scene_view_model.positions.len(), 1);
            assert_eq!(context.global_state.choreography.scenes[0].positions.len(), 1);

            let position = &scene_view_model.positions[0];
            common::assert_close(position.x, 1.0, 0.0001);
            common::assert_close(position.y, 1.0, 0.0001);
        });
    });

    let report = common::run_suite(&suite);
    assert!(report.is_success());
}
