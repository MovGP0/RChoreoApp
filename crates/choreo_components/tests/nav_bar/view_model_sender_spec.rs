use choreo_components::nav_bar::actions::NavBarAction;
use choreo_components::nav_bar::messages::NavBarSenders;
use std::sync::mpsc::channel;

#[test]
fn view_model_with_senders_forwards_commands() {
    macro_rules! check {
        ($errors:ident, $expr:expr) => {{
            if !$expr {
                $errors.push(stringify!($expr).to_string());
            }
        }};
    }

    macro_rules! assert_no_errors {
        ($errors:ident) => {
            assert!(
                $errors.is_empty(),
                "expected no assertion errors, found:\n{}",
                $errors.join("\n")
            );
        };
    }

    let (open_audio_sender, open_audio_receiver) = channel();
    let (open_image_sender, open_image_receiver) = channel();
    let (reset_sender, reset_receiver) = channel();
    let (mode_sender, mode_receiver) = channel();

    let senders = NavBarSenders {
        open_audio_requested: open_audio_sender,
        open_image_requested: open_image_sender,
        reset_floor_viewport_requested: reset_sender,
        interaction_mode_changed: mode_sender,
    };

    let mut view_model =
        choreo_components::nav_bar::view_model::NavBarViewModel::with_senders(senders);
    view_model.dispatch(NavBarAction::OpenAudio);
    view_model.dispatch(NavBarAction::OpenImage);
    view_model.dispatch(NavBarAction::ResetFloorViewport);
    view_model.dispatch(NavBarAction::SetSelectedMode {
        mode: choreo_components::nav_bar::state::InteractionMode::Move,
    });

    let mut errors = Vec::new();
    check!(errors, open_audio_receiver.try_recv().is_ok());
    check!(errors, open_image_receiver.try_recv().is_ok());
    check!(errors, reset_receiver.try_recv().is_ok());
    check!(errors, mode_receiver.try_recv().is_ok());
    assert_no_errors!(errors);
}
