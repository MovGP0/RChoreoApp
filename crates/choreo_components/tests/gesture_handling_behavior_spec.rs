mod common;

use common::Report;
use choreo_components::floor::{GestureHandlingBehavior, Point};

#[test]
fn gesture_handling_behavior_spec() {
    let suite = rspec::describe("gesture handling behavior", (), |spec| {
        spec.it("applies pan translation on pointer drag", |_| {
            let mut context = common::FloorTestContext::new();
            context.configure_canvas();
            let mut behavior = GestureHandlingBehavior::default();

            behavior.handle_pointer_pressed(&mut context.state_machine, common::pointer_pressed(Point::new(10.0, 10.0)));
            behavior.handle_pointer_moved(&mut context.view_model, &mut context.state_machine, common::pointer_moved(Point::new(30.0, 25.0)));

            let matrix = context.view_model.transformation_matrix;
            assert!((matrix.trans_x() - 20.0).abs() < 0.001);
            assert!((matrix.trans_y() - 15.0).abs() < 0.001);
        });

        spec.it("zooms in with mouse wheel", |_| {
            let mut context = common::FloorTestContext::new();
            context.configure_canvas();
            let mut behavior = GestureHandlingBehavior::default();

            behavior.handle_pointer_wheel_changed(
                &mut context.view_model,
                &mut context.state_machine,
                common::pointer_wheel_changed(120.0, Some(Point::new(50.0, 50.0))),
            );

            let matrix = context.view_model.transformation_matrix;
            assert!(matrix.scale_x() > 1.0);
        });

        spec.it("zooms out with mouse wheel", |_| {
            let mut context = common::FloorTestContext::new();
            context.configure_canvas();
            let mut behavior = GestureHandlingBehavior::default();

            behavior.handle_pointer_wheel_changed(
                &mut context.view_model,
                &mut context.state_machine,
                common::pointer_wheel_changed(-120.0, Some(Point::new(50.0, 50.0))),
            );

            let matrix = context.view_model.transformation_matrix;
            assert!(matrix.scale_x() < 1.0);
        });

        spec.it("zooms with two finger pinch", |_| {
            let mut context = common::FloorTestContext::new();
            context.configure_canvas();
            let mut behavior = GestureHandlingBehavior::default();

            behavior.handle_touch(
                &mut context.view_model,
                &mut context.state_machine,
                common::touch_command(1, choreo_components::floor::TouchAction::Pressed, Point::new(40.0, 50.0), true),
            );
            behavior.handle_touch(
                &mut context.view_model,
                &mut context.state_machine,
                common::touch_command(2, choreo_components::floor::TouchAction::Pressed, Point::new(60.0, 50.0), true),
            );
            behavior.handle_touch(
                &mut context.view_model,
                &mut context.state_machine,
                common::touch_command(1, choreo_components::floor::TouchAction::Moved, Point::new(30.0, 50.0), true),
            );
            behavior.handle_touch(
                &mut context.view_model,
                &mut context.state_machine,
                common::touch_command(2, choreo_components::floor::TouchAction::Moved, Point::new(70.0, 50.0), true),
            );
            behavior.handle_touch(
                &mut context.view_model,
                &mut context.state_machine,
                common::touch_command(1, choreo_components::floor::TouchAction::Released, Point::new(30.0, 50.0), false),
            );
            behavior.handle_touch(
                &mut context.view_model,
                &mut context.state_machine,
                common::touch_command(2, choreo_components::floor::TouchAction::Released, Point::new(70.0, 50.0), false),
            );

            let matrix = context.view_model.transformation_matrix;
            assert!(matrix.scale_x() > 1.0);
        });
    });

    let report = common::run_suite(&suite);
    assert!(report.is_success());
}
