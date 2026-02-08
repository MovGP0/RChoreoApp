use crate::floor;
use choreo_components::floor::Point;
use floor::Report;
use std::time::Duration;

#[test]
#[serial_test::serial]
fn gesture_handling_behavior_spec() {
    let suite = rspec::describe("gesture handling behavior", (), |spec| {
        spec.it("applies pan translation on pointer drag", |_| {
            let context = floor::FloorTestContext::new();
            context.configure_canvas();

            context.send_pointer_pressed(Point::new(10.0, 10.0));
            context.send_pointer_moved(Point::new(30.0, 25.0));

            let translated = context.wait_until(Duration::from_secs(1), || {
                let matrix = context.view_model.borrow().transformation_matrix;
                (matrix.trans_x() - 20.0).abs() < 0.001 && (matrix.trans_y() - 15.0).abs() < 0.001
            });
            assert!(translated);
        });

        spec.it("zooms in with mouse wheel", |_| {
            let context = floor::FloorTestContext::new();
            context.configure_canvas();

            context.send_pointer_wheel_changed(120.0, Some(Point::new(50.0, 50.0)));

            let zoomed = context.wait_until(Duration::from_secs(1), || {
                context.view_model.borrow().transformation_matrix.scale_x() > 1.0
            });
            assert!(zoomed);
        });

        spec.it("zooms out with mouse wheel", |_| {
            let context = floor::FloorTestContext::new();
            context.configure_canvas();

            context.send_pointer_wheel_changed(-120.0, Some(Point::new(50.0, 50.0)));

            let zoomed = context.wait_until(Duration::from_secs(1), || {
                context.view_model.borrow().transformation_matrix.scale_x() < 1.0
            });
            assert!(zoomed);
        });

        spec.it("zooms with two finger pinch", |_| {
            let context = floor::FloorTestContext::new();
            context.configure_canvas();

            context.send_touch(
                1,
                choreo_components::floor::TouchAction::Pressed,
                Point::new(40.0, 50.0),
                true,
            );
            context.send_touch(
                2,
                choreo_components::floor::TouchAction::Pressed,
                Point::new(60.0, 50.0),
                true,
            );
            context.send_touch(
                1,
                choreo_components::floor::TouchAction::Moved,
                Point::new(30.0, 50.0),
                true,
            );
            context.send_touch(
                2,
                choreo_components::floor::TouchAction::Moved,
                Point::new(70.0, 50.0),
                true,
            );
            context.send_touch(
                1,
                choreo_components::floor::TouchAction::Released,
                Point::new(30.0, 50.0),
                false,
            );
            context.send_touch(
                2,
                choreo_components::floor::TouchAction::Released,
                Point::new(70.0, 50.0),
                false,
            );

            let zoomed = context.wait_until(Duration::from_secs(1), || {
                context.view_model.borrow().transformation_matrix.scale_x() > 1.0
            });
            assert!(zoomed);
        });
    });

    let report = floor::run_suite(&suite);
    assert!(report.is_success());
}
