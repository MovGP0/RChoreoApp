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

#[test]
fn copy_scene_positions_dialog_emits_copy_or_keep_decisions() {
    let mut state = create_state();

    reduce(
        &mut state,
        ScenesAction::LoadScenes {
            choreography: Box::new(choreography_with_scenes(
                "Show",
                vec![scene_model(1, "Alpha", None, vec![])],
            )),
        },
    );

    reduce(&mut state, ScenesAction::OpenCopyScenePositionsDialog);
    let mut errors = Vec::new();

    check!(errors, state.show_copy_scene_positions_dialog);

    reduce(
        &mut state,
        ScenesAction::ConfirmCopyScenePositionsDialog {
            copy_positions: true,
        },
    );
    check!(errors, !state.show_copy_scene_positions_dialog);
    check_eq!(errors, state.copy_scene_positions_decision, Some(true));

    reduce(&mut state, ScenesAction::OpenCopyScenePositionsDialog);
    reduce(
        &mut state,
        ScenesAction::ConfirmCopyScenePositionsDialog {
            copy_positions: false,
        },
    );
    check_eq!(errors, state.copy_scene_positions_decision, Some(false));

    assert!(
        errors.is_empty(),
        "Assertion failures:\n{}",
        errors.join("\n")
    );
}
