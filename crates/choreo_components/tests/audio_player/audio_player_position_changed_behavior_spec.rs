use choreo_components::audio_player::actions::AudioPlayerAction;
use choreo_components::audio_player::reducer::AudioPlayerEffect;
use choreo_components::audio_player::reducer::reduce;
use choreo_components::audio_player::state::AudioPlayerState;

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
fn audio_player_position_changed_emits_events_only_when_position_changes() {
    let mut state = AudioPlayerState {
        position: 1.0,
        ..AudioPlayerState::default()
    };

    let mut errors = Vec::new();

    let effects = reduce(&mut state, AudioPlayerAction::PublishPositionIfChanged);
    check_eq!(
        errors,
        effects,
        vec![AudioPlayerEffect::PositionChangedPublished {
            position_seconds: 1.0
        }]
    );

    let no_effect = reduce(&mut state, AudioPlayerAction::PublishPositionIfChanged);
    check!(errors, no_effect.is_empty());

    state.position = 2.0;
    let effects_again = reduce(&mut state, AudioPlayerAction::PublishPositionIfChanged);
    check_eq!(
        errors,
        effects_again,
        vec![AudioPlayerEffect::PositionChangedPublished {
            position_seconds: 2.0
        }]
    );

    assert_no_errors(errors);
}
