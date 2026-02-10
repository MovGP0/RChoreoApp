use std::time::Duration;

use crate::dancers;

use choreo_master_mobile_json::Color;
use dancers::Report;

#[test]
#[serial_test::serial]
fn update_dancer_details_behavior_spec() {
    let suite = rspec::describe("update dancer details behavior", (), |spec| {
        spec.it(
            "updates selected dancer name shortcut and color and propagates to list",
            |_| {
                let role = dancers::build_role("Gentleman");
                let dancer = dancers::build_dancer(1, role, "Alice", "A", None);
                let context = dancers::DancersTestContext::with_global_state(|state| {
                    state.choreography.dancers = vec![dancer];
                });

                context
                    .view_model
                    .borrow_mut()
                    .update_dancer_name("Alice Updated".to_string());
                context
                    .view_model
                    .borrow_mut()
                    .update_dancer_shortcut("AU".to_string());
                context.view_model.borrow_mut().update_dancer_color(Color {
                    r: 10,
                    g: 20,
                    b: 30,
                    a: 255,
                });

                let updated = context.wait_until(Duration::from_secs(1), || {
                    let view_model = context.view_model.borrow();
                    let Some(selected) = view_model.selected_dancer.as_ref() else {
                        return false;
                    };
                    selected.name == "Alice Updated"
                        && selected.shortcut == "AU"
                        && selected.color.r == 10
                        && selected.color.g == 20
                        && selected.color.b == 30
                        && view_model.dancers[0].name == "Alice Updated"
                        && view_model.dancers[0].shortcut == "AU"
                        && view_model.dancers[0].color.r == 10
                        && view_model.dancers[0].color.g == 20
                        && view_model.dancers[0].color.b == 30
                });
                assert!(updated);
            },
        );
    });

    let report = dancers::run_suite(&suite);
    assert!(report.is_success());
}
