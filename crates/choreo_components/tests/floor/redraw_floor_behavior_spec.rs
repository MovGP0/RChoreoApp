mod floor;

use floor::Report;
use choreo_components::floor::RedrawFloorBehavior;

fn count_draws(receiver: &crossbeam_channel::Receiver<choreo_components::floor::DrawFloorCommand>) -> usize {
    receiver.try_iter().count()
}

#[test]
fn redraw_floor_behavior_spec() {
    let suite = rspec::describe("redraw floor behavior", (), |spec| {
        spec.it("redraws when choreography changes", |_| {
            let mut context = floor::FloorTestContext::new();
            context.configure_canvas();
            let behavior = RedrawFloorBehavior;

            behavior.handle_choreography_changed(&context.view_model);
            assert!(count_draws(&context.draw_floor_receiver) > 0);
        });

        spec.it("redraws when selected scene changes", |_| {
            let mut context = floor::FloorTestContext::new();
            context.configure_canvas();
            let behavior = RedrawFloorBehavior;

            behavior.handle_selected_scene_changed(&context.view_model);
            assert!(count_draws(&context.draw_floor_receiver) > 0);
        });

        spec.it("redraws when redraw command is published", |_| {
            let mut context = floor::FloorTestContext::new();
            context.configure_canvas();
            let behavior = RedrawFloorBehavior;

            behavior.handle_redraw_command(&context.view_model);
            assert!(count_draws(&context.draw_floor_receiver) > 0);
        });
    });

    let report = floor::run_suite(&suite);
    assert!(report.is_success());
}
