use crate::dancers;
use dancers::Report;

#[test]
fn save_dancer_settings_behavior_spec() {
    let suite = rspec::describe("save dancer settings behavior", (), |spec| {
        spec.it("persists dancers and rewrites scene references", |_| {
            let role = dancers::role("Role");
            let dancer_a = dancers::dancer(1, role.clone(), "Alice", "A", None);
            let dancer_b = dancers::dancer(2, role.clone(), "Bob", "B", None);

            let mut state = dancers::state::DancersState::default().with_global(
                dancers::state::DancersGlobalState {
                    roles: vec![role],
                    dancers: vec![dancer_a.clone(), dancer_b],
                    scenes: vec![dancers::scene(vec![
                        dancers::position(Some(1), Some("Alice")),
                        dancers::position(Some(2), Some("Bob")),
                    ])],
                    scene_views: vec![dancers::state::SceneViewState {
                        positions: vec![
                            dancers::position(Some(1), Some("Alice")),
                            dancers::position(Some(2), Some("Bob")),
                        ],
                    }],
                    selected_scene: Some(dancers::state::SceneViewState {
                        positions: vec![
                            dancers::position(Some(1), Some("Alice")),
                            dancers::position(Some(2), Some("Bob")),
                        ],
                    }),
                    ..dancers::state::DancersGlobalState::default()
                },
            );

            dancers::reducer::reduce(&mut state, dancers::actions::DancersAction::LoadFromGlobal);
            state.roles = vec![state.dancers[0].role.clone()];
            state.dancers = vec![dancers::state::DancerState {
                name: "Updated Alice".to_string(),
                ..dancer_a
            }];
            state.selected_dancer = state.dancers.first().cloned();

            dancers::reducer::reduce(&mut state, dancers::actions::DancersAction::SaveToGlobal);

            assert_eq!(state.global.roles.len(), 1);
            assert_eq!(state.global.dancers.len(), 1);
            assert_eq!(state.global.dancers[0].name, "Updated Alice");
            assert_eq!(state.global.scenes[0].positions.len(), 1);
            assert_eq!(
                state.global.scenes[0].positions[0].dancer_name.as_deref(),
                Some("Updated Alice")
            );
            assert_eq!(state.global.scene_views[0].positions.len(), 1);
            assert_eq!(
                state.global
                    .selected_scene
                    .as_ref()
                    .map(|scene| scene.positions.len()),
                Some(1)
            );
        });
    });
    let report = dancers::run_suite(&suite);
    assert!(report.is_success());
}
