use super::actions::ScenesAction;
use super::choreography_with_scenes;
use super::create_state;
use super::reducer::reduce;
use super::scene_model;

#[test]
fn open_choreo_updates_state_and_audio_request() {
    let mut state = create_state();
    let choreography = choreography_with_scenes("FromFile", vec![scene_model(1, "Intro", None, vec![])]);

    reduce(
        &mut state,
        ScenesAction::OpenChoreography {
            choreography: Box::new(choreography),
            file_path: Some("C:/tmp/test.choreo".to_string()),
            file_name: None,
            audio_path: Some("C:/tmp/test.mp3".to_string()),
        },
    );

    assert_eq!(state.choreography.name, "FromFile");
    assert!(state.reload_requested);
    assert_eq!(state.last_opened_choreo_file.as_deref(), Some("C:/tmp/test.choreo"));
    assert_eq!(state.pending_open_audio.as_deref(), Some("C:/tmp/test.mp3"));
    assert!(!state.close_audio_requested);
}

#[test]
fn open_choreo_without_audio_requests_close_audio() {
    let mut state = create_state();
    let choreography = choreography_with_scenes("FromName", vec![]);

    reduce(
        &mut state,
        ScenesAction::OpenChoreography {
            choreography: Box::new(choreography),
            file_path: None,
            file_name: Some("browser-import.choreo".to_string()),
            audio_path: None,
        },
    );

    assert_eq!(state.last_opened_choreo_file.as_deref(), Some("browser-import.choreo"));
    assert!(state.close_audio_requested);

    reduce(&mut state, ScenesAction::ClearEphemeralOutputs);
    assert!(!state.close_audio_requested);
    assert!(state.pending_open_audio.is_none());
}
