use super::actions::ScenesAction;
use super::choreography_with_scenes;
use super::create_state;
use super::reducer::reduce;
use super::scene_model;
use std::fs;

#[test]
fn save_choreo_maps_scene_view_state_back_to_model() {
    let mut state = create_state();
    let choreography = choreography_with_scenes(
        "My Choreo",
        vec![scene_model(1, "Intro", Some("00:12"), vec![])],
    );

    reduce(
        &mut state,
        ScenesAction::LoadScenes {
            choreography: Box::new(choreography),
        },
    );

    let temp_root = std::env::temp_dir().join("rchoreo_scenes_save_map_back_spec");
    fs::create_dir_all(&temp_root).expect("temp dir should be created");
    let existing_path = temp_root.join("saved.choreo");
    fs::write(&existing_path, "{}").expect("temp file should be created");
    state.last_opened_choreo_file = Some(existing_path.to_string_lossy().into_owned());
    state.scenes[0].text = "  details  ".to_string();
    state.scenes[0].timestamp = Some(12.0);

    reduce(&mut state, ScenesAction::SaveChoreography);

    assert_eq!(state.choreography.scenes[0].name, "Intro");
    assert_eq!(
        state.choreography.scenes[0].timestamp.as_deref(),
        Some("12")
    );
    assert_eq!(
        state.choreography.scenes[0].text.as_deref(),
        Some("details")
    );
    assert!(state.can_save_choreo);

    let _ = fs::remove_file(existing_path);
    let _ = fs::remove_dir(temp_root);
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

#[test]
fn save_choreo_requires_existing_file_for_enablement_parity() {
    let mut state = create_state();
    let choreography = choreography_with_scenes("Before", vec![]);

    reduce(
        &mut state,
        ScenesAction::LoadScenes {
            choreography: Box::new(choreography),
        },
    );

    let temp_root = std::env::temp_dir().join("rchoreo_scenes_save_enablement_spec");
    fs::create_dir_all(&temp_root).expect("temp dir should be created");
    let missing_path = temp_root.join("missing.choreo");
    state.last_opened_choreo_file = Some(missing_path.to_string_lossy().into_owned());

    reduce(&mut state, ScenesAction::SaveChoreography);

    assert!(!state.can_save_choreo);

    let existing_path = temp_root.join("existing.choreo");
    fs::write(&existing_path, "{}").expect("temp file should be created");
    state.last_opened_choreo_file = Some(existing_path.to_string_lossy().into_owned());

    reduce(&mut state, ScenesAction::SaveChoreography);

    assert!(state.can_save_choreo);

    let _ = fs::remove_file(existing_path);
    let _ = fs::remove_dir(temp_root);
}
