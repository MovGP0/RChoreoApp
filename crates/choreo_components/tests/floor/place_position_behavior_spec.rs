use crate::floor;

use floor::Report;
use choreo_components::floor::Point;
use choreo_state_machine::PlacePositionsStartedTrigger;
use std::time::Duration;

#[test]
fn place_position_behavior_spec() {
    let suite = rspec::describe("place position behavior", (), |spec| {
        spec.it("places a new position on click", |_| {
            let context = floor::FloorTestContext::new();
            context.configure_canvas();

            let (choreography, scene) = floor::build_empty_scene_choreography();
            let scene_view_model = floor::map_scene_view_model(&scene);

            context.update_global_state(|state| {
                state.choreography = choreography;
                state.selected_scene = Some(scene_view_model.clone());
                state.scenes = vec![scene_view_model];
                state.is_place_mode = true;
            });

            context.update_state_machine(|state_machine| {
                let _ = state_machine.try_apply(&PlacePositionsStartedTrigger);
            });

            let view_point = floor::floor_to_view_point(&context, Point::new(1.0, 1.0));
            context.send_pointer_pressed(view_point);
            context.send_pointer_released(view_point);

            let added = context.wait_until(Duration::from_secs(1), || {
                let scene_count = context
                    .read_global_state(|state| state.selected_scene.as_ref().map(|scene| scene.positions.len()).unwrap_or(0));
                let choreography_count = context.read_global_state(|state| state.choreography.scenes[0].positions.len());
                scene_count == 1 && choreography_count == 1
            });
            assert!(added);

            let scene = context.read_global_state(|state| state.selected_scene.clone().expect("scene"));
            let position = &scene.positions[0];
            floor::assert_close(position.x, 1.0, 0.0001);
            floor::assert_close(position.y, 1.0, 0.0001);
        });
    });

    let report = floor::run_suite(&suite);
    assert!(report.is_success());
}
