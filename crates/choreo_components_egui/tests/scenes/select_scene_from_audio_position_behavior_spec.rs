use super::actions::ScenesAction;
use super::choreography_with_scenes;
use super::create_state;
use super::reducer::reduce;
use super::scene_model;

#[test]
fn select_scene_from_audio_position_selects_matching_range() {
    let mut state = create_state();
    let choreography = choreography_with_scenes(
        "Test",
        vec![
            scene_model(1, "First", Some("00:05"), vec![]),
            scene_model(2, "Second", Some("00:10"), vec![]),
            scene_model(3, "Third", Some("00:20"), vec![]),
        ],
    );

    reduce(
        &mut state,
        ScenesAction::LoadScenes {
            choreography: Box::new(choreography),
        },
    );
    state.clear_ephemeral_outputs();

    reduce(
        &mut state,
        ScenesAction::SelectSceneFromAudioPosition {
            position_seconds: 12.0,
        },
    );

    assert_eq!(state.selected_scene.as_ref().map(|scene| scene.name.as_str()), Some("Second"));
    assert!(state.selected_scene_changed);
    assert!(state.redraw_floor_requested);
}

#[test]
fn select_scene_from_audio_position_does_not_select_before_first_timestamp() {
    let mut state = create_state();
    let choreography = choreography_with_scenes(
        "Test",
        vec![scene_model(1, "First", Some("00:05"), vec![]), scene_model(2, "Second", Some("00:10"), vec![])],
    );

    reduce(
        &mut state,
        ScenesAction::LoadScenes {
            choreography: Box::new(choreography),
        },
    );
    state.selected_scene = None;
    state.clear_ephemeral_outputs();

    reduce(
        &mut state,
        ScenesAction::SelectSceneFromAudioPosition {
            position_seconds: 2.0,
        },
    );

    assert!(state.selected_scene.is_none());
    assert!(!state.selected_scene_changed);
    assert!(!state.redraw_floor_requested);
}

#[test]
fn select_scene_from_audio_position_does_not_emit_when_scene_does_not_change() {
    let mut state = create_state();
    let choreography = choreography_with_scenes(
        "Test",
        vec![
            scene_model(1, "First", Some("00:05"), vec![]),
            scene_model(2, "Second", Some("00:10"), vec![]),
        ],
    );

    reduce(
        &mut state,
        ScenesAction::LoadScenes {
            choreography: Box::new(choreography),
        },
    );
    state.clear_ephemeral_outputs();

    reduce(
        &mut state,
        ScenesAction::SelectSceneFromAudioPosition {
            position_seconds: 7.0,
        },
    );

    assert_eq!(state.selected_scene.as_ref().map(|scene| scene.name.as_str()), Some("First"));
    assert!(!state.selected_scene_changed);
    assert!(!state.redraw_floor_requested);
}

#[test]
fn select_scene_from_audio_position_ignores_non_increasing_next_timestamp() {
    let mut state = create_state();
    let choreography = choreography_with_scenes(
        "Test",
        vec![
            scene_model(1, "First", Some("00:05"), vec![]),
            scene_model(2, "Second", Some("00:03"), vec![]),
        ],
    );

    reduce(
        &mut state,
        ScenesAction::LoadScenes {
            choreography: Box::new(choreography),
        },
    );
    state.clear_ephemeral_outputs();

    reduce(
        &mut state,
        ScenesAction::SelectSceneFromAudioPosition {
            position_seconds: 4.0,
        },
    );

    assert_eq!(state.selected_scene.as_ref().map(|scene| scene.name.as_str()), Some("First"));
    assert!(!state.selected_scene_changed);
    assert!(!state.redraw_floor_requested);
}
