use super::actions::ChoreographySettingsAction;
use super::actions::UpdateSelectedSceneAction;
use super::color;
use super::create_state;
use super::reducer::reduce;
use super::scene_model;
use super::selected_scene;

macro_rules! check_eq {
    ($errors:expr, $left:expr, $right:expr) => {
        if $left != $right {
            $errors.push(format!(
                "{} != {} (left = {:?}, right = {:?})",
                stringify!($left),
                stringify!($right),
                $left,
                $right
            ));
        }
    };
}

macro_rules! check {
    ($errors:expr, $condition:expr) => {
        if !$condition {
            $errors.push(format!("condition failed: {}", stringify!($condition)));
        }
    };
}

fn assert_no_errors(errors: Vec<String>) {
    assert!(
        errors.is_empty(),
        "Assertion failures:\n{}",
        errors.join("\n")
    );
}

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
        ChoreographySettingsAction::UpdateSelectedScene(
            UpdateSelectedSceneAction::SyncFromSelected,
        ),
    );

    let mut errors = Vec::new();

    check_eq!(errors, state.scene_name, "Original");

    reduce(
        &mut state,
        ChoreographySettingsAction::UpdateSelectedScene(UpdateSelectedSceneAction::SceneName(
            "Updated".to_string(),
        )),
    );

    check_eq!(errors, state.scene_name, "Updated");
    check_eq!(errors, state.choreography.scenes[0].name, "Updated");
    check!(errors, state.redraw_requested);

    assert_no_errors(errors);
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
    let mut errors = Vec::new();

    check_eq!(
        errors,
        state.choreography.scenes[0].text.as_deref(),
        Some("Note")
    );
    check!(errors, state.redraw_requested);

    state.clear_ephemeral_outputs();
    reduce(
        &mut state,
        ChoreographySettingsAction::UpdateSelectedScene(
            UpdateSelectedSceneAction::SceneTimestamp {
                has_timestamp: true,
                seconds: 12.5,
            },
        ),
    );
    check_eq!(
        errors,
        state.choreography.scenes[0].timestamp.as_deref(),
        Some("12.5")
    );
    check!(errors, state.redraw_requested);

    state.clear_ephemeral_outputs();
    reduce(
        &mut state,
        ChoreographySettingsAction::UpdateSelectedScene(
            UpdateSelectedSceneAction::SceneFixedPositions(true),
        ),
    );
    check!(errors, state.choreography.scenes[0].fixed_positions);
    check!(errors, state.redraw_requested);

    state.clear_ephemeral_outputs();
    let scene_color = color(255, 5, 6, 7);
    reduce(
        &mut state,
        ChoreographySettingsAction::UpdateSelectedScene(UpdateSelectedSceneAction::SceneColor(
            scene_color.clone(),
        )),
    );
    check_eq!(errors, state.choreography.scenes[0].color, scene_color);
    check!(errors, state.redraw_requested);

    assert_no_errors(errors);
}
