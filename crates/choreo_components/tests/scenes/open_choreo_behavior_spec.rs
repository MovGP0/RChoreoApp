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
    assert!(
        errors.is_empty(),
        "Assertion failures:\n{}",
        errors.join("\n")
    );
}

#[test]
fn open_choreo_updates_state_and_audio_request() {
    let mut state = create_state();
    let choreography =
        choreography_with_scenes("FromFile", vec![scene_model(1, "Intro", None, vec![])]);

    reduce(
        &mut state,
        ScenesAction::OpenChoreography {
            choreography: Box::new(choreography),
            file_path: Some("C:/tmp/test.choreo".to_string()),
            file_name: None,
            audio_path: Some("C:/tmp/test.mp3".to_string()),
        },
    );

    let mut errors = Vec::new();

    check_eq!(errors, state.choreography.name, "FromFile");
    check!(errors, state.reload_requested);
    check_eq!(
        errors,
        state.last_opened_choreo_file.as_deref(),
        Some("C:/tmp/test.choreo")
    );
    check!(errors, state.selected_scene_changed);
    check_eq!(
        errors,
        state
            .selected_scene
            .as_ref()
            .map(|scene| scene.name.as_str()),
        Some("Intro")
    );
    check_eq!(
        errors,
        state.pending_open_audio.as_deref(),
        Some("C:/tmp/test.mp3")
    );
    check!(errors, !state.close_audio_requested);

    assert_no_errors(errors);
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

    let mut errors = Vec::new();

    check_eq!(
        errors,
        state.last_opened_choreo_file.as_deref(),
        Some("browser-import.choreo")
    );
    check!(errors, state.close_audio_requested);

    reduce(&mut state, ScenesAction::ClearEphemeralOutputs);

    check!(errors, !state.close_audio_requested);
    check!(errors, state.pending_open_audio.is_none());

    assert_no_errors(errors);
}
