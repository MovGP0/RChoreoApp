use super::actions::ScenesAction;
use super::build_position;
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
fn load_scenes_maps_models_and_selects_first_scene() {
    let mut state = create_state();
    let choreography = choreography_with_scenes(
        "Test",
        vec![
            scene_model(1, "Intro", Some("00:05"), vec![build_position(0.0, 0.0)]),
            scene_model(2, "Verse", None, vec![build_position(1.0, 1.0)]),
        ],
    );

    reduce(
        &mut state,
        ScenesAction::LoadScenes {
            choreography: Box::new(choreography),
        },
    );

    let mut errors = Vec::new();

    check_eq!(errors, state.scenes.len(), 2);
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
        state
            .selected_scene
            .as_ref()
            .and_then(|scene| scene.timestamp),
        Some(5.0)
    );

    assert_no_errors(errors);
}

#[test]
fn reload_scenes_reloads_from_choreography() {
    let mut state = create_state();
    let choreography =
        choreography_with_scenes("Test", vec![scene_model(1, "First", None, vec![])]);

    reduce(
        &mut state,
        ScenesAction::LoadScenes {
            choreography: Box::new(choreography),
        },
    );

    state
        .choreography
        .scenes
        .push(scene_model(2, "Second", Some("00:09"), vec![]));
    reduce(&mut state, ScenesAction::ReloadScenes);

    let mut errors = Vec::new();

    check_eq!(errors, state.scenes.len(), 2);
    check_eq!(errors, state.scenes[1].name, "Second");
    check_eq!(errors, state.scenes[1].timestamp, Some(9.0));
    check!(errors, state.reload_requested);

    assert_no_errors(errors);
}

#[test]
fn reload_scenes_reselects_first_scene_and_marks_selection_changed() {
    let mut state = create_state();
    let choreography = choreography_with_scenes(
        "Test",
        vec![
            scene_model(1, "First", None, vec![]),
            scene_model(2, "Second", None, vec![]),
        ],
    );

    reduce(
        &mut state,
        ScenesAction::LoadScenes {
            choreography: Box::new(choreography),
        },
    );
    state.clear_ephemeral_outputs();

    reduce(&mut state, ScenesAction::SelectScene { index: 1 });
    state.clear_ephemeral_outputs();

    reduce(&mut state, ScenesAction::ReloadScenes);

    let mut errors = Vec::new();

    check_eq!(
        errors,
        state
            .selected_scene
            .as_ref()
            .map(|scene| scene.name.as_str()),
        Some("First")
    );
    check!(errors, state.selected_scene_changed);
    check!(errors, state.reload_requested);

    assert_no_errors(errors);
}
