use super::actions::ChoreoMainAction;
use super::actions::OpenAudioRequested;
use super::actions::OpenSvgFileCommand;
use super::main_page_binding::MainPageActionHandlers;
use super::main_view_model::MainViewModel;

pub(crate) fn consume_outgoing_commands(
    view_model: &mut MainViewModel,
    handlers: &MainPageActionHandlers,
) {
    let audio_requests = view_model.state().outgoing_audio_requests.clone();
    let open_svg_commands = view_model.state().outgoing_open_svg_commands.clone();

    for request in audio_requests {
        if let Some(request_open_audio) = handlers.request_open_audio.as_ref() {
            request_open_audio(request);
            continue;
        }

        // Host fallback: support direct file-open integration when no typed handler is wired.
        if let Some(pick_audio_path) = handlers.pick_audio_path.as_ref() {
            let _ = pick_audio_path();
        }
    }

    for command in open_svg_commands {
        route_open_svg_command(command, handlers);
    }

    view_model.dispatch(ChoreoMainAction::ClearOutgoingCommands);
}

fn route_open_svg_command(command: OpenSvgFileCommand, handlers: &MainPageActionHandlers) {
    if let Some(request_open_image) = handlers.request_open_image.as_ref() {
        request_open_image(command.file_path);
        return;
    }

    if let Some(pick_image_path) = handlers.pick_image_path.as_ref() {
        let _ = pick_image_path();
    }
}

pub(crate) fn enqueue_open_audio_request(
    view_model: &mut MainViewModel,
    request: OpenAudioRequested,
) {
    view_model.request_open_audio(request);
}
