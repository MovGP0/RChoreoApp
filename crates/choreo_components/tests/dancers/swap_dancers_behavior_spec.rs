use std::time::Duration;

use crate::dancers;

use choreo_master_mobile_json::Color;
use choreo_master_mobile_json::DancerId;
use choreo_models::DancerModel;
use dancers::Report;
use std::rc::Rc;

#[test]
#[serial_test::serial]
fn swap_dancers_behavior_spec() {
    let suite = rspec::describe("swap dancers behavior", (), |spec| {
        spec.it(
            "swaps role name shortcut color and icon for two different dancers while keeping identities",
            |_| {
                let gentleman = dancers::build_role("Gentleman");
                let lady = dancers::build_role("Lady");
                let first = Rc::new(DancerModel {
                    dancer_id: DancerId(1),
                    role: gentleman.clone(),
                    name: "Alex".to_string(),
                    shortcut: "AL".to_string(),
                    color: Color {
                        r: 10,
                        g: 20,
                        b: 30,
                        a: 255,
                    },
                    icon: Some("IconA".to_string()),
                });
                let second = Rc::new(DancerModel {
                    dancer_id: DancerId(2),
                    role: lady.clone(),
                    name: "Bella".to_string(),
                    shortcut: "BE".to_string(),
                    color: Color {
                        r: 40,
                        g: 50,
                        b: 60,
                        a: 255,
                    },
                    icon: Some("IconB".to_string()),
                });
                let context = dancers::DancersTestContext::with_global_state(|state| {
                    state.choreography.roles = vec![gentleman, lady];
                    state.choreography.dancers = vec![first, second];
                });

                let ready = context.wait_until(Duration::from_secs(1), || {
                    context.view_model.borrow().can_swap_dancers
                });
                assert!(ready);

                context.view_model.borrow_mut().swap_dancers();

                let swapped = context.wait_until(Duration::from_secs(1), || {
                    let view_model = context.view_model.borrow();
                    if view_model.dancers.len() < 2 {
                        return false;
                    }

                    let first_dancer = &view_model.dancers[0];
                    let second_dancer = &view_model.dancers[1];

                    first_dancer.dancer_id.0 == 1
                        && second_dancer.dancer_id.0 == 2
                        && first_dancer.role.name == "Lady"
                        && second_dancer.role.name == "Gentleman"
                        && first_dancer.name == "Bella"
                        && second_dancer.name == "Alex"
                        && first_dancer.shortcut == "BE"
                        && second_dancer.shortcut == "AL"
                        && first_dancer.color.r == 40
                        && second_dancer.color.r == 10
                        && first_dancer.icon.as_deref() == Some("IconB")
                        && second_dancer.icon.as_deref() == Some("IconA")
                });
                assert!(swapped);
            },
        );

        spec.it("does not swap when selection is invalid", |_| {
            let role = dancers::build_role("Role");
            let only = dancers::build_dancer(1, role, "A", "A", None);
            let context = dancers::DancersTestContext::with_global_state(|state| {
                state.choreography.dancers = vec![only];
            });

            context.view_model.borrow_mut().swap_dancers();
            context.pump_events();

            let view_model = context.view_model.borrow();
            assert!(!view_model.can_swap_dancers);
            assert_eq!(view_model.dancers.len(), 1);
            assert_eq!(view_model.dancers[0].name, "A");
        });
    });

    let report = dancers::run_suite(&suite);
    assert!(report.is_success());
}
