use choreo_components_egui::nav_bar::actions::NavBarAction;
use choreo_components_egui::nav_bar::messages::NavBarSenders;
use std::sync::mpsc::channel;

#[test]
fn view_model_with_senders_forwards_commands() {
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

    let mut view_model = choreo_components_egui::nav_bar::view_model::NavBarViewModel::with_senders(senders);
    view_model.dispatch(NavBarAction::OpenAudio);
    view_model.dispatch(NavBarAction::OpenImage);
    view_model.dispatch(NavBarAction::ResetFloorViewport);
    view_model.dispatch(NavBarAction::SetSelectedMode {
        mode: choreo_components_egui::nav_bar::state::InteractionMode::Move,
    });

    assert!(open_audio_receiver.try_recv().is_ok());
    assert!(open_image_receiver.try_recv().is_ok());
    assert!(reset_receiver.try_recv().is_ok());
    assert!(mode_receiver.try_recv().is_ok());
}
