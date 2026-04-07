use super::actions::ScenesAction;
use super::choreography_with_scenes;
use super::create_state;
use super::reducer::reduce;
use super::scene_model;

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
fn scenes_action_surface_exposes_navigation_and_file_requests_with_caps() {
    let mut state = create_state();

    reduce(
        &mut state,
        ScenesAction::LoadScenes {
            choreography: Box::new(choreography_with_scenes(
                "Show",
                vec![scene_model(1, "One", None, vec![])],
            )),
        },
    );

    let mut errors = Vec::new();

    check!(errors, state.can_delete_scene);
    check!(errors, state.can_navigate_to_settings);
    check!(errors, state.can_navigate_to_dancer_settings);

    reduce(&mut state, ScenesAction::RequestOpenChoreography);
    reduce(&mut state, ScenesAction::NavigateToSettings);
    reduce(&mut state, ScenesAction::NavigateToDancerSettings);

    check!(errors, state.request_open_choreo_dialog);
    check!(errors, state.navigate_to_settings_requested);
    check!(errors, state.navigate_to_dancer_settings_requested);

    assert_no_errors(errors);
}
