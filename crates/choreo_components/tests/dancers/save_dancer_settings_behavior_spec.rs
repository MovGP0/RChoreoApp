use std::rc::Rc;
use std::time::Duration;

use crate::dancers;

use choreo_models::DancerModel;
use choreo_components::scenes::SceneViewModel;
use dancers::Report;

#[test]
#[serial_test::serial]
fn save_dancer_settings_behavior_spec() {
    let suite = rspec::describe("save dancer settings behavior", (), |spec| {
        spec.it(
            "persists dancers and rewrites scene dancer references",
            |_| {
                let role = dancers::build_role("Role");
                let dancer_a = dancers::build_dancer(1, role.clone(), "Alice", "A", None);
                let dancer_b = dancers::build_dancer(2, role.clone(), "Bob", "B", None);
                let scene = dancers::build_scene(
                    1,
                    vec![
                        dancers::build_position(Some(dancer_a.clone())),
                        dancers::build_position(Some(dancer_b.clone())),
                    ],
                );
                let scene_view_model = SceneViewModel {
                    scene_id: scene.scene_id,
                    name: scene.name.clone(),
                    text: scene.text.clone().unwrap_or_default(),
                    fixed_positions: scene.fixed_positions,
                    timestamp: None,
                    is_selected: true,
                    positions: scene.positions.clone(),
                    variation_depth: scene.variation_depth,
                    variations: scene.variations.clone(),
                    current_variation: scene.current_variation.clone(),
                    color: scene.color.clone(),
                };
                let context = dancers::DancersTestContext::with_global_state(|state| {
                    state.choreography.roles = vec![role];
                    state.choreography.dancers = vec![dancer_a.clone(), dancer_b];
                    state.choreography.scenes = vec![scene];
                    state.scenes = vec![scene_view_model.clone()];
                    state.selected_scene = Some(scene_view_model);
                });

                {
                    let mut view_model = context.view_model.borrow_mut();
                    let updated = Rc::new(DancerModel {
                        name: "Updated Alice".to_string(),
                        ..(*view_model.dancers[0]).clone()
                    });
                    view_model.roles = vec![updated.role.clone()];
                    view_model.dancers = vec![updated.clone()];
                    view_model.selected_dancer = Some(updated);
                }

                context.view_model.borrow_mut().save();

                let saved = context.wait_until(Duration::from_secs(1), || {
                    context.read_global_state(|state| {
                        state.choreography.dancers.len() == 1
                            && state.choreography.scenes[0].positions.len() == 1
                            && state.scenes[0].positions.len() == 1
                            && state
                                .selected_scene
                                .as_ref()
                                .map(|scene| scene.positions.len())
                                == Some(1)
                    })
                });
                assert!(saved);

                context.read_global_state(|state| {
                    assert_eq!(state.choreography.roles.len(), 1);
                    assert_eq!(state.choreography.dancers.len(), 1);
                    assert_eq!(state.choreography.dancers[0].name, "Updated Alice");
                    assert_eq!(state.choreography.scenes[0].positions.len(), 1);
                    let dancer = state.choreography.scenes[0].positions[0]
                        .dancer
                        .as_ref()
                        .expect("remaining scene position should keep dancer");
                    assert_eq!(dancer.dancer_id.0, 1);
                    let scene_dancer = state.scenes[0].positions[0]
                        .dancer
                        .as_ref()
                        .expect("remaining scene view-model position should keep dancer");
                    assert_eq!(scene_dancer.name, "Updated Alice");
                    let selected_scene_dancer = state
                        .selected_scene
                        .as_ref()
                        .and_then(|scene| scene.positions.first())
                        .and_then(|position| position.dancer.as_ref())
                        .expect("selected scene position should keep dancer");
                    assert_eq!(selected_scene_dancer.name, "Updated Alice");
                });
            },
        );
    });

    let report = dancers::run_suite(&suite);
    assert!(report.is_success());
}
