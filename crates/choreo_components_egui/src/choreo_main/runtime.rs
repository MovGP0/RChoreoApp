use super::actions::ChoreoMainAction;
use super::actions::OpenAudioRequested;
use super::actions::OpenChoreoRequested;
use super::actions::OpenSvgFileCommand;
use super::behavior_pipeline::MainBehaviorPipeline;
use super::main_page_binding::MainPageActionHandlers;
use super::main_view_model::MainViewModel;
use super::messages::OpenImageRequested;

pub(crate) fn consume_outgoing_commands(
    view_model: &mut MainViewModel,
    handlers: &MainPageActionHandlers,
    behavior_pipeline: &MainBehaviorPipeline,
) {
    let audio_requests = view_model.state().outgoing_audio_requests.clone();
    let choreo_requests = view_model.state().outgoing_open_choreo_requests.clone();
    let open_svg_commands = view_model.state().outgoing_open_svg_commands.clone();

    for request in choreo_requests {
        route_open_choreo_request(request, handlers);
    }

    for request in audio_requests {
        if let Some(behavior) = behavior_pipeline.open_audio_behavior.as_ref() {
            behavior.apply(request.clone());
        }

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
        route_open_svg_command(command, view_model, handlers, behavior_pipeline);
    }

    view_model.dispatch(ChoreoMainAction::ClearOutgoingCommands);
}

fn route_open_choreo_request(request: OpenChoreoRequested, handlers: &MainPageActionHandlers) {
    if let Some(request_open_choreo) = handlers.request_open_choreo.as_ref() {
        request_open_choreo(request);
    }
}

fn route_open_svg_command(
    command: OpenSvgFileCommand,
    view_model: &mut MainViewModel,
    handlers: &MainPageActionHandlers,
    behavior_pipeline: &MainBehaviorPipeline,
) {
    if let Some(behavior) = behavior_pipeline.open_svg_file_behavior.as_ref() {
        behavior.apply(view_model, command.clone());
    } else {
        view_model.dispatch(ChoreoMainAction::ApplyOpenSvgFile(command.clone()));
    }

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

pub(crate) fn enqueue_open_image_request(
    view_model: &mut MainViewModel,
    request: OpenImageRequested,
    behavior_pipeline: &MainBehaviorPipeline,
) {
    if let Some(behavior) = behavior_pipeline.open_image_behavior.as_ref() {
        let command = behavior.apply(request);
        view_model.dispatch(ChoreoMainAction::RequestOpenImage {
            file_path: command.file_path,
        });
        return;
    }

    view_model.request_open_image(request);
}
