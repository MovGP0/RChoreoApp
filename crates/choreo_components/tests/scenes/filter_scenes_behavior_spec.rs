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
fn filter_scenes_by_search_text() {
    let mut state = create_state();
    let choreography = choreography_with_scenes(
        "Test",
        vec![
            scene_model(1, "Opening", None, vec![]),
            scene_model(2, "Chorus", None, vec![]),
            scene_model(3, "Closing", None, vec![]),
        ],
    );

    reduce(
        &mut state,
        ScenesAction::LoadScenes {
            choreography: Box::new(choreography),
        },
    );
    reduce(
        &mut state,
        ScenesAction::UpdateSearchText("clo".to_string()),
    );

    let mut errors = Vec::new();

    check_eq!(errors, state.visible_scenes.len(), 1);
    check_eq!(errors, state.visible_scenes[0].name.as_str(), "Closing");

    assert_no_errors(errors);
}

#[test]
fn filter_scenes_clears_and_restores_all() {
    let mut state = create_state();
    let choreography = choreography_with_scenes(
        "Test",
        vec![
            scene_model(1, "Opening", None, vec![]),
            scene_model(2, "Chorus", None, vec![]),
        ],
    );

    reduce(
        &mut state,
        ScenesAction::LoadScenes {
            choreography: Box::new(choreography),
        },
    );
    reduce(
        &mut state,
        ScenesAction::UpdateSearchText("open".to_string()),
    );
    let mut errors = Vec::new();

    check_eq!(errors, state.visible_scenes.len(), 1);

    reduce(&mut state, ScenesAction::UpdateSearchText(String::new()));
    check_eq!(errors, state.visible_scenes.len(), 2);

    assert_no_errors(errors);
}
