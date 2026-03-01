use super::actions::ChoreographySettingsAction;
use super::actions::UpdateSelectedSceneAction;
use super::color;
use super::create_state;
use super::reducer::reduce;
use super::scene_model;
use super::selected_scene;

#[test]
fn update_selected_scene_syncs_and_updates_name() {
    let mut state = create_state();
    let mut choreography = super::choreography_with_name("Test");
    choreography.scenes = vec![scene_model(10, "Original", None, Some("3"))];

    reduce(
        &mut state,
        ChoreographySettingsAction::LoadChoreography {
            choreography: Box::new(choreography),
            selected_scene: Some(selected_scene(10, "Original")),
        },
    );

    reduce(
        &mut state,
        ChoreographySettingsAction::UpdateSelectedScene(UpdateSelectedSceneAction::SyncFromSelected),
    );

    assert_eq!(state.scene_name, "Original");

    reduce(
        &mut state,
        ChoreographySettingsAction::UpdateSelectedScene(UpdateSelectedSceneAction::SceneName(
            "Updated".to_string(),
        )),
    );

    assert_eq!(state.scene_name, "Updated");
    assert_eq!(state.choreography.scenes[0].name, "Updated");
}

#[test]
fn update_selected_scene_updates_text_timestamp_and_color() {
    let mut state = create_state();
    let mut choreography = super::choreography_with_name("Test");
    choreography.scenes = vec![scene_model(10, "Original", None, None)];

    reduce(
        &mut state,
        ChoreographySettingsAction::LoadChoreography {
            choreography: Box::new(choreography),
            selected_scene: Some(selected_scene(10, "Original")),
        },
    );

    reduce(
        &mut state,
        ChoreographySettingsAction::UpdateSelectedScene(UpdateSelectedSceneAction::SceneText(
            "  Note  ".to_string(),
        )),
    );
    assert_eq!(state.choreography.scenes[0].text.as_deref(), Some("Note"));

    reduce(
        &mut state,
        ChoreographySettingsAction::UpdateSelectedScene(UpdateSelectedSceneAction::SceneTimestamp {
            has_timestamp: true,
            seconds: 12.5,
        }),
    );
    assert_eq!(state.choreography.scenes[0].timestamp.as_deref(), Some("12.5"));

    reduce(
        &mut state,
        ChoreographySettingsAction::UpdateSelectedScene(
            UpdateSelectedSceneAction::SceneFixedPositions(true),
        ),
    );
    assert!(state.choreography.scenes[0].fixed_positions);

    let scene_color = color(255, 5, 6, 7);
    reduce(
        &mut state,
        ChoreographySettingsAction::UpdateSelectedScene(UpdateSelectedSceneAction::SceneColor(
            scene_color.clone(),
        )),
    );
    assert_eq!(state.choreography.scenes[0].color, scene_color);
}
