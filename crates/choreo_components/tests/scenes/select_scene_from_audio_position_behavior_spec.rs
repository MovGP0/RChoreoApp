use super::actions::ScenesAction;
use super::choreography_with_scenes;
use super::create_state;
use super::reducer::reduce;
use super::scene_model;

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
    assert!(errors.is_empty(), "assertion failures: {:?}", errors);
}

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

    let mut errors = Vec::new();
    check_eq!(
        errors,
        state.selected_scene.as_ref().map(|scene| scene.name.as_str()),
        Some("Second")
    );
    check!(errors, state.selected_scene_changed);
    check!(errors, state.redraw_floor_requested);
    assert_no_errors(errors);
}

#[test]
fn select_scene_from_audio_position_does_not_select_before_first_timestamp() {
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
    state.selected_scene = None;
    state.clear_ephemeral_outputs();

    reduce(
        &mut state,
        ScenesAction::SelectSceneFromAudioPosition {
            position_seconds: 2.0,
        },
    );

    let mut errors = Vec::new();
    check!(errors, state.selected_scene.is_none());
    check!(errors, !state.selected_scene_changed);
    check!(errors, !state.redraw_floor_requested);
    assert_no_errors(errors);
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

    let mut errors = Vec::new();
    check_eq!(
        errors,
        state.selected_scene.as_ref().map(|scene| scene.name.as_str()),
        Some("First")
    );
    check!(errors, !state.selected_scene_changed);
    check!(errors, !state.redraw_floor_requested);
    assert_no_errors(errors);
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

    let mut errors = Vec::new();
    check_eq!(
        errors,
        state.selected_scene.as_ref().map(|scene| scene.name.as_str()),
        Some("First")
    );
    check!(errors, !state.selected_scene_changed);
    check!(errors, !state.redraw_floor_requested);
    assert_no_errors(errors);
}
