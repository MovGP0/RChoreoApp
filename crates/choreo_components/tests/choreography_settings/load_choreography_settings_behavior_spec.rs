use super::actions::ChoreographySettingsAction;
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

macro_rules! check_close {
    ($errors:expr, $left:expr, $right:expr, $tolerance:expr) => {
        if ($left - $right).abs() > $tolerance {
            $errors.push(format!(
                "{} !~= {} (left = {:?}, right = {:?}, tolerance = {:?})",
                stringify!($left),
                stringify!($right),
                $left,
                $right,
                $tolerance
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
fn load_choreography_settings_maps_choreography_and_selected_scene() {
    let mut state = create_state();
    let mut choreography = super::choreography_with_name("My Choreo");
    choreography.author = Some("Author".to_string());
    choreography.floor.size_front = 12;
    choreography.settings.show_timestamps = true;
    choreography.scenes = vec![scene_model(10, "Original", Some("Line"), Some("3"))];

    reduce(
        &mut state,
        ChoreographySettingsAction::LoadChoreography {
            choreography: Box::new(choreography),
            selected_scene: Some(selected_scene(10, "Original")),
        },
    );

    let mut errors = Vec::new();

    check_eq!(errors, state.name, "My Choreo");
    check_eq!(errors, state.author, "Author");
    check_eq!(errors, state.floor_front, 12);
    check!(errors, state.show_timestamps);
    check!(errors, state.has_selected_scene);
    check_eq!(errors, state.scene_name, "Original");
    check!(errors, state.scene_has_timestamp);
    check_close!(errors, state.scene_timestamp_seconds, 3.0, 0.0001);

    assert_no_errors(errors);
}

#[test]
fn load_choreography_settings_reloads_when_load_action_is_dispatched_again() {
    let mut state = create_state();
    let initial = super::choreography_with_name("Before");

    reduce(
        &mut state,
        ChoreographySettingsAction::LoadChoreography {
            choreography: Box::new(initial),
            selected_scene: None,
        },
    );

    let mut updated = super::choreography_with_name("After");
    updated.floor.size_back = 77;
    reduce(
        &mut state,
        ChoreographySettingsAction::LoadChoreography {
            choreography: Box::new(updated),
            selected_scene: None,
        },
    );

    let mut errors = Vec::new();

    check_eq!(errors, state.name, "After");
    check_eq!(errors, state.floor_back, 77);

    assert_no_errors(errors);
}
