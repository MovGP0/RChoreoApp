use super::actions::ScenesAction;
use super::choreography_with_scenes;
use super::create_state;
use super::reducer::reduce;
use super::scene_model;

#[test]
fn scenes_action_surface_exposes_navigation_and_file_requests_with_caps() {
    let mut state = create_state();

    reduce(
        &mut state,
        ScenesAction::LoadScenes {
            choreography: Box::new(choreography_with_scenes(
                "Show",
                vec![scene_model(1, "One", None, vec![])],
            )),
        },
    );

    assert!(state.can_delete_scene);
    assert!(state.can_navigate_to_settings);
    assert!(state.can_navigate_to_dancer_settings);

    reduce(&mut state, ScenesAction::RequestOpenChoreography);
    reduce(&mut state, ScenesAction::NavigateToSettings);
    reduce(&mut state, ScenesAction::NavigateToDancerSettings);

    assert!(state.request_open_choreo_dialog);
    assert!(state.navigate_to_settings_requested);
    assert!(state.navigate_to_dancer_settings_requested);
}
