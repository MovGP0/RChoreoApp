use std::rc::Rc;
use std::time::Duration;

use crate::dancers;

use choreo_models::DancerModel;
use dancers::Report;

#[test]
#[serial_test::serial]
fn selected_dancer_state_behavior_spec() {
    let suite = rspec::describe("selected dancer state behavior", (), |spec| {
        spec.it(
            "updates selected state, role, and icon option after selection",
            |_| {
                let role = dancers::build_role("Role");
                let first = dancers::build_dancer(1, role.clone(), "A", "A", None);
                let second = dancers::build_dancer(2, role, "B", "B", None);
                let context = dancers::DancersTestContext::with_global_state(|state| {
                    state.choreography.roles = vec![dancers::build_role("Role")];
                    state.choreography.dancers = vec![first, second];
                });

                let icon_key = context
                    .view_model
                    .borrow()
                    .icon_options
                    .first()
                    .map(|option| option.key.clone())
                    .expect("icon options should not be empty");
                {
                    let mut view_model = context.view_model.borrow_mut();
                    let updated = Rc::new(DancerModel {
                        icon: Some(icon_key.clone()),
                        ..(*view_model.dancers[1]).clone()
                    });
                    view_model.dancers[1] = updated;
                }

                context.view_model.borrow_mut().select_dancer(1);

                let updated = context.wait_until(Duration::from_secs(1), || {
                    let view_model = context.view_model.borrow();
                    view_model.has_selected_dancer
                        && view_model.can_delete_dancer
                        && view_model
                            .selected_icon_option
                            .as_ref()
                            .map(|icon| icon.key.as_str())
                            == Some(icon_key.as_str())
                });
                assert!(updated);

                let view_model = context.view_model.borrow();
                assert_eq!(
                    view_model
                        .selected_role
                        .as_ref()
                        .map(|role| role.name.as_str()),
                    Some("Role")
                );
            },
        );
    });

    let report = dancers::run_suite(&suite);
    assert!(report.is_success());
}
