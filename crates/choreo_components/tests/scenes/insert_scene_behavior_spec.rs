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

fn assert_no_errors(errors: Vec<String>) {
    assert!(
        errors.is_empty(),
        "Assertion failures:\n{}",
        errors.join("\n")
    );
}

#[test]
fn insert_scene_after_selected_and_select_new_scene() {
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
    reduce(&mut state, ScenesAction::InsertScene { insert_after: true });

    let mut errors = Vec::new();

    check_eq!(errors, state.scenes.len(), 3);
    check_eq!(errors, state.scenes[0].scene_id.0, 1);
    check_eq!(errors, state.scenes[1].name, "New Scene");
    check_eq!(errors, state.scenes[2].scene_id.0, 2);
    check_eq!(
        errors,
        state.selected_scene.as_ref().map(|scene| scene.scene_id.0),
        Some(state.scenes[1].scene_id.0)
    );
    check_eq!(errors, state.choreography.scenes.len(), 3);

    assert_no_errors(errors);
}

#[test]
fn insert_scene_appends_when_none_selected() {
    let mut state = create_state();
    let choreography =
        choreography_with_scenes("Test", vec![scene_model(1, "First", None, vec![])]);

    reduce(
        &mut state,
        ScenesAction::LoadScenes {
            choreography: Box::new(choreography),
        },
    );
    state.selected_scene = None;

    reduce(
        &mut state,
        ScenesAction::InsertScene {
            insert_after: false,
        },
    );

    let mut errors = Vec::new();

    check_eq!(errors, state.scenes.len(), 2);
    check_eq!(errors, state.scenes[1].name, "New Scene");
    check_eq!(errors, state.choreography.scenes.len(), 2);

    assert_no_errors(errors);
}
