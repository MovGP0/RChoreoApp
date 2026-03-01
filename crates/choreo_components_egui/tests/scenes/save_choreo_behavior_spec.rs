use super::actions::ScenesAction;
use super::choreography_with_scenes;
use super::create_state;
use super::reducer::reduce;
use super::scene_model;

#[test]
fn save_choreo_maps_scene_view_state_back_to_model() {
    let mut state = create_state();
    let choreography = choreography_with_scenes("My Choreo", vec![scene_model(1, "Intro", Some("00:12"), vec![])]);

    reduce(
        &mut state,
        ScenesAction::LoadScenes {
            choreography: Box::new(choreography),
        },
    );

    state.last_opened_choreo_file = Some("C:/tmp/saved.choreo".to_string());
    state.scenes[0].text = "  details  ".to_string();
    state.scenes[0].timestamp = Some(12.0);

    reduce(&mut state, ScenesAction::SaveChoreography);

    assert_eq!(state.choreography.scenes[0].name, "Intro");
    assert_eq!(state.choreography.scenes[0].timestamp.as_deref(), Some("12"));
    assert_eq!(state.choreography.scenes[0].text.as_deref(), Some("details"));
    assert!(state.can_save_choreo);
}

#[test]
fn save_choreo_with_no_last_opened_file_does_not_enable_can_save() {
    let mut state = create_state();
    let choreography = choreography_with_scenes("Before", vec![]);

    reduce(
        &mut state,
        ScenesAction::LoadScenes {
            choreography: Box::new(choreography),
        },
    );
    state.last_opened_choreo_file = None;

    reduce(&mut state, ScenesAction::SaveChoreography);

    assert!(!state.can_save_choreo);
}
