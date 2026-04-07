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
fn select_scene_updates_selected_scene_and_emits_flags() {
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

    reduce(&mut state, ScenesAction::SelectScene { index: 1 });

    let mut errors = Vec::new();

    check_eq!(
        errors,
        state
            .selected_scene
            .as_ref()
            .map(|scene| scene.name.as_str()),
        Some("Second")
    );
    check!(errors, state.selected_scene_changed);
    check!(errors, state.redraw_floor_requested);

    assert_no_errors(errors);
}

#[test]
fn select_scene_ignores_out_of_range_index() {
    let mut state = create_state();
    let choreography =
        choreography_with_scenes("Test", vec![scene_model(1, "First", None, vec![])]);

    reduce(
        &mut state,
        ScenesAction::LoadScenes {
            choreography: Box::new(choreography),
        },
    );
    state.clear_ephemeral_outputs();

    reduce(&mut state, ScenesAction::SelectScene { index: 10 });

    let mut errors = Vec::new();

    check_eq!(
        errors,
        state
            .selected_scene
            .as_ref()
            .map(|scene| scene.name.as_str()),
        Some("First")
    );
    check!(errors, !state.selected_scene_changed);
    check!(errors, !state.redraw_floor_requested);

    assert_no_errors(errors);
}
