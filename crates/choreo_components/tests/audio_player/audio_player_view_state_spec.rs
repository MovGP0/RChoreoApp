use choreo_components::audio_player::actions::AudioPlayerAction;
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
fn audio_player_view_state_restores_playback_after_drag_when_previously_playing() {
    let mut state = AudioPlayerState {
        can_seek: true,
        has_player: true,
        is_playing: true,
        ..AudioPlayerState::default()
    };

    reduce(&mut state, AudioPlayerAction::PositionDragStarted);
    reduce(
        &mut state,
        AudioPlayerAction::PositionDragCompleted { position: 12.0 },
    );

    let mut errors = Vec::new();

    check!(errors, !state.is_user_dragging);
    check_eq!(errors, state.pending_seek_position, Some(12.0));
    check_eq!(errors, state.position, 12.0);
    check!(errors, state.is_playing);

    assert_no_errors(errors);
}

#[test]
fn audio_player_view_state_ignores_stale_player_position_until_target_is_acknowledged() {
    let mut state = AudioPlayerState {
        can_seek: true,
        has_player: true,
        ..AudioPlayerState::default()
    };

    reduce(&mut state, AudioPlayerAction::PositionDragStarted);
    reduce(
        &mut state,
        AudioPlayerAction::PositionDragCompleted { position: 18.0 },
    );
    reduce(
        &mut state,
        AudioPlayerAction::PlayerPositionSampled { position: 4.0 },
    );

    let mut errors = Vec::new();

    check_eq!(errors, state.position, 18.0);
    check_eq!(errors, state.pending_seek_position, Some(18.0));

    reduce(
        &mut state,
        AudioPlayerAction::PlayerPositionSampled { position: 18.0 },
    );

    check_eq!(errors, state.position, 18.0);
    check!(errors, state.pending_seek_position.is_none());

    assert_no_errors(errors);
}

#[test]
fn audio_player_view_state_keeps_dragged_position_after_commit_if_actor_is_stale() {
    let mut state = AudioPlayerState {
        can_seek: true,
        has_player: true,
        position: 4.0,
        ..AudioPlayerState::default()
    };

    reduce(&mut state, AudioPlayerAction::PositionDragStarted);
    reduce(
        &mut state,
        AudioPlayerAction::PositionDragCompleted { position: 18.0 },
    );

    let mut errors = Vec::new();

    check_eq!(errors, state.position, 18.0);
    check_eq!(errors, state.pending_seek_position, Some(18.0));

    assert_no_errors(errors);
}

#[test]
fn audio_player_reducer_and_ui_cover_core_action_paths() {
    let mut state = AudioPlayerState {
        has_player: true,
        can_seek: true,
        ..AudioPlayerState::default()
    };

    reduce(&mut state, AudioPlayerAction::Initialize);
    reduce(&mut state, AudioPlayerAction::TogglePlayPause);
    reduce(&mut state, AudioPlayerAction::Stop);
    reduce(
        &mut state,
        AudioPlayerAction::SeekToPosition { position: 1.5 },
    );
    reduce(
        &mut state,
        AudioPlayerAction::PositionPreviewChanged { position: 2.0 },
    );
    reduce(&mut state, AudioPlayerAction::SpeedChanged { speed: 0.83 });

    let context = egui::Context::default();
    let raw_input = egui::RawInput::default();
    let _ = context.run(raw_input, |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            let _ = choreo_components::audio_player::ui::draw(ui, &state);
        });
    });
}
