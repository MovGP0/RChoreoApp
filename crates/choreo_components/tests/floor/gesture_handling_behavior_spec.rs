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

            context.send_pointer_wheel_changed(0.0, 120.0, false, Some(Point::new(50.0, 50.0)));

            let zoomed = context.wait_until(Duration::from_secs(1), || {
                context.view_model.borrow().transformation_matrix.scale_x() > 1.0
            });
            assert!(zoomed);
        });

        spec.it("zooms out with mouse wheel", |_| {
            let context = floor::FloorTestContext::new();
            context.configure_canvas();

            context.send_pointer_wheel_changed(0.0, -120.0, false, Some(Point::new(50.0, 50.0)));

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

        spec.it("pans with smooth scroll gesture deltas", |_| {
            let context = floor::FloorTestContext::new();
            context.configure_canvas();

            context.send_pointer_wheel_changed(14.0, -10.0, false, Some(Point::new(50.0, 50.0)));

            let panned = context.wait_until(Duration::from_secs(1), || {
                let matrix = context.view_model.borrow().transformation_matrix;
                (matrix.trans_x() - 14.0).abs() < 0.001 && (matrix.trans_y() + 10.0).abs() < 0.001
            });
            assert!(panned);
        });

        spec.it("pans with non-notched vertical scroll", |_| {
            let context = floor::FloorTestContext::new();
            context.configure_canvas();

            context.send_pointer_wheel_changed(0.0, -96.0, false, Some(Point::new(50.0, 50.0)));

            let panned = context.wait_until(Duration::from_secs(1), || {
                let matrix = context.view_model.borrow().transformation_matrix;
                matrix.scale_x() == 1.0 && (matrix.trans_y() + 96.0).abs() < 0.001
            });
            assert!(panned);
        });

        spec.it("ignores pointer drag while two-finger touch gesture is active", |_| {
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
                Point::new(35.0, 50.0),
                true,
            );
            context.send_touch(
                2,
                choreo_components::floor::TouchAction::Moved,
                Point::new(65.0, 50.0),
                true,
            );

            let before_pointer_drag = context.view_model.borrow().transformation_matrix;

            context.send_pointer_pressed(Point::new(30.0, 30.0));
            context.send_pointer_moved(Point::new(90.0, 90.0));
            context.send_pointer_released(Point::new(90.0, 90.0));

            context.pump_events();
            let after_pointer_drag = context.view_model.borrow().transformation_matrix;
            assert_eq!(after_pointer_drag, before_pointer_drag);
        });

        spec.it("resets viewport on double tap", |_| {
            let context = floor::FloorTestContext::new();
            context.configure_canvas();

            context.send_pointer_wheel_changed(0.0, 120.0, false, Some(Point::new(50.0, 50.0)));
            let zoomed = context.wait_until(Duration::from_secs(1), || {
                context.view_model.borrow().transformation_matrix.scale_x() > 1.0
            });
            assert!(zoomed);

            context.send_pointer_pressed(Point::new(40.0, 40.0));
            context.send_pointer_released(Point::new(40.0, 40.0));
            context.send_pointer_pressed(Point::new(42.0, 41.0));
            context.send_pointer_released(Point::new(42.0, 41.0));

            let reset = context.wait_until(Duration::from_secs(1), || {
                context.view_model.borrow().transformation_matrix
                    == choreo_components::floor::Matrix::identity()
            });
            assert!(reset);
        });

        spec.it("keeps viewport unchanged on single tap", |_| {
            let context = floor::FloorTestContext::new();
            context.configure_canvas();

            let original_matrix = choreo_components::floor::Matrix::translation(18.0, -12.0)
                .concat(&choreo_components::floor::Matrix::scale(1.2, 1.2, 50.0, 50.0));
            context.set_transformation_matrix(original_matrix);

            context.send_pointer_pressed(Point::new(40.0, 40.0));
            context.send_pointer_released(Point::new(40.0, 40.0));

            context.pump_events();
            assert_eq!(context.view_model.borrow().transformation_matrix, original_matrix);
        });
    });

    let report = floor::run_suite(&suite);
    assert!(report.is_success());
}
