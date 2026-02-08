use crate::floor;

use floor::Report;
use std::time::Duration;

#[test]
#[serial_test::serial]
fn redraw_floor_behavior_spec() {
    let suite = rspec::describe("redraw floor behavior", (), |spec| {
        spec.it("redraws when choreography changes", |_| {
            let context = floor::FloorTestContext::new();
            context.configure_canvas();

            context.update_global_state(|state| {
                state.choreography.name = "Updated".to_string();
            });
            context.send_redraw_command();

            let redrawn = context.wait_until(Duration::from_secs(1), || context.draw_count() > 0);
            assert!(redrawn);
        });

        spec.it("redraws when selected scene changes", |_| {
            let context = floor::FloorTestContext::new();
            context.configure_canvas();

            let (_, scene) = floor::build_three_position_choreography();
            let scene_view_model = floor::map_scene_view_model(&scene);
            context.update_global_state(|state| {
                state.selected_scene = Some(scene_view_model);
            });
            context.send_redraw_command();

            let redrawn = context.wait_until(Duration::from_secs(1), || context.draw_count() > 0);
            assert!(redrawn);
        });

        spec.it("redraws when redraw command is published", |_| {
            let context = floor::FloorTestContext::new();
            context.configure_canvas();

            context.send_redraw_command();

            let redrawn = context.wait_until(Duration::from_secs(1), || context.draw_count() > 0);
            assert!(redrawn);
        });
    });

    let report = floor::run_suite(&suite);
    assert!(report.is_success());
}
