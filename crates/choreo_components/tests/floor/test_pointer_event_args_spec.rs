use crate::floor;

use choreo_components::floor::{Point, PointerButton, PointerEventArgs};
use floor::Report;

#[test]
#[serial_test::serial]
fn test_pointer_event_args_spec() {
    let suite = rspec::describe("pointer event args", (), |spec| {
        spec.it("exposes position and button", |_| {
            let expected_point = Point::new(12.0, 34.0);
            let expected_button = PointerButton::Primary;

            let args = PointerEventArgs {
                position: expected_point,
                button: expected_button,
                is_in_contact: true,
            };

            assert_eq!(args.position, expected_point);
            assert_eq!(args.button, expected_button);
        });
    });

    let report = floor::run_suite(&suite);
    assert!(report.is_success());
}
