use super::actions::ChoreographySettingsAction;
use super::create_state;
use super::reducer::reduce;
use super::scene_model;
use super::selected_scene;

#[test]
fn load_choreography_settings_maps_choreography_and_selected_scene() {
    let mut state = create_state();
    let mut choreography = super::choreography_with_name("My Choreo");
    choreography.author = Some("Author".to_string());
    choreography.floor.size_front = 12;
    choreography.settings.show_timestamps = true;
    choreography.scenes = vec![scene_model(10, "Original", Some("Line"), Some("3"))];

    reduce(
        &mut state,
        ChoreographySettingsAction::LoadChoreography {
            choreography: Box::new(choreography),
            selected_scene: Some(selected_scene(10, "Original")),
        },
    );

    assert_eq!(state.name, "My Choreo");
    assert_eq!(state.author, "Author");
    assert_eq!(state.floor_front, 12);
    assert!(state.show_timestamps);
    assert!(state.has_selected_scene);
    assert_eq!(state.scene_name, "Original");
    assert!(state.scene_has_timestamp);
    assert!((state.scene_timestamp_seconds - 3.0).abs() < 0.0001);
}

#[test]
fn load_choreography_settings_reloads_when_load_action_is_dispatched_again() {
    let mut state = create_state();
    let initial = super::choreography_with_name("Before");

    reduce(
        &mut state,
        ChoreographySettingsAction::LoadChoreography {
            choreography: Box::new(initial),
            selected_scene: None,
        },
    );

    let mut updated = super::choreography_with_name("After");
    updated.floor.size_back = 77;
    reduce(
        &mut state,
        ChoreographySettingsAction::LoadChoreography {
            choreography: Box::new(updated),
            selected_scene: None,
        },
    );

    assert_eq!(state.name, "After");
    assert_eq!(state.floor_back, 77);
}
