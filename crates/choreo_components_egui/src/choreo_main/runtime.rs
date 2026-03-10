use std::path::Path;

use choreo_master_mobile_json::export_to_file;
use choreo_master_mobile_json::import;
use choreo_models::ChoreographyModel;
use choreo_models::ChoreographyModelMapper;

use crate::audio_player::actions::AudioPlayerAction;
use crate::audio_player::runtime::AudioPlayerRuntime;
use crate::audio_player::runtime::apply_player_sample_without_position;
use crate::audio_player::state::AudioPlayerChoreographyScene;
use crate::audio_player::state::AudioPlayerScene;
use crate::choreography_settings::actions::ChoreographySettingsAction;
use crate::choreography_settings::state::SelectedSceneState;
use crate::scenes::state::parse_timestamp_seconds;

use super::actions::ChoreoMainAction;
use super::actions::OpenAudioRequested;
use super::actions::OpenChoreoRequested;
use super::actions::OpenSvgFileCommand;
use super::behavior_pipeline::MainBehaviorPipeline;
use super::main_page_binding::MainPageActionHandlers;
use super::main_view_model::MainViewModel;
use super::reducer::sync_audio_position_internal;

pub(crate) fn consume_outgoing_commands(
    view_model: &mut MainViewModel,
    handlers: &MainPageActionHandlers,
    behavior_pipeline: &MainBehaviorPipeline,
    audio_runtime: &mut AudioPlayerRuntime,
) {
    let audio_requests = view_model.state().outgoing_audio_requests.clone();
    let choreo_requests = view_model.state().outgoing_open_choreo_requests.clone();
    let save_requests = view_model.state().outgoing_save_choreo_requests.clone();
    let open_svg_commands = view_model.state().outgoing_open_svg_commands.clone();

    for request in choreo_requests {
        route_open_choreo_request(
            view_model,
            request,
            handlers,
            behavior_pipeline,
            audio_runtime,
        );
    }

    for request in audio_requests {
        route_open_audio_request(
            view_model,
            audio_runtime,
            request,
            handlers,
            behavior_pipeline,
            true,
        );
    }

    for request in save_requests {
        apply_save_choreo_request(view_model, request.file_path.as_str());
    }

    for command in open_svg_commands {
        route_open_svg_command(command, view_model, handlers, behavior_pipeline);
    }

    view_model.dispatch(ChoreoMainAction::ClearOutgoingCommands);
}

fn route_open_choreo_request(
    view_model: &mut MainViewModel,
    request: OpenChoreoRequested,
    handlers: &MainPageActionHandlers,
    behavior_pipeline: &MainBehaviorPipeline,
    audio_runtime: &mut AudioPlayerRuntime,
) {
    let resolved_request = if request.contents.trim().is_empty() {
        handlers
            .pick_choreo_file
            .as_ref()
            .and_then(|pick_choreo_file| pick_choreo_file())
    } else {
        Some(request)
    };

    let Some(resolved_request) = resolved_request else {
        return;
    };

    if let Some(request_open_choreo) = handlers.request_open_choreo.as_ref() {
        request_open_choreo(resolved_request.clone());
    }

    apply_open_choreo_request(
        view_model,
        resolved_request,
        handlers,
        behavior_pipeline,
        audio_runtime,
    );
}

fn apply_open_choreo_request(
    view_model: &mut MainViewModel,
    request: OpenChoreoRequested,
    handlers: &MainPageActionHandlers,
    behavior_pipeline: &MainBehaviorPipeline,
    audio_runtime: &mut AudioPlayerRuntime,
) {
    let Ok(json_model) = import(&request.contents) else {
        return;
    };

    let mapper = ChoreographyModelMapper;
    let choreography = mapper.map_to_model(&json_model);
    let selected_scene = choreography.scenes.first().map(map_selected_scene_state);
    let audio_request =
        resolve_audio_request(&choreography, request.file_path.as_deref()).map(|file_path| {
            OpenAudioRequested {
                file_path,
                trace_context: None,
            }
        });

    view_model.dispatch(ChoreoMainAction::UpdateSceneSearchText(String::new()));
    view_model.dispatch(ChoreoMainAction::ChoreographySettingsAction(
        ChoreographySettingsAction::LoadChoreography {
            choreography: Box::new(choreography.clone()),
            selected_scene,
        },
    ));
    view_model.dispatch(ChoreoMainAction::AudioPlayerAction(
        AudioPlayerAction::SetScenes {
            scenes: map_audio_player_scenes(&choreography),
            selected_scene_id: choreography.scenes.first().map(|scene| scene.scene_id.0),
            choreography_scenes: map_audio_player_choreography_scenes(&choreography),
        },
    ));
    view_model.dispatch(ChoreoMainAction::AudioPlayerAction(
        AudioPlayerAction::UpdateTicksAndLinkState,
    ));
    let last_opened_choreo_file = request.file_path.clone().or(request.file_name.clone());
    let state = view_model.state_mut();
    state.last_opened_choreo_file = last_opened_choreo_file;
    state.draw_floor_request_count += 1;

    if let Some(audio_request) = audio_request {
        route_open_audio_request(
            view_model,
            audio_runtime,
            audio_request,
            handlers,
            behavior_pipeline,
            false,
        );
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

fn route_open_audio_request(
    view_model: &mut MainViewModel,
    audio_runtime: &mut AudioPlayerRuntime,
    request: OpenAudioRequested,
    handlers: &MainPageActionHandlers,
    behavior_pipeline: &MainBehaviorPipeline,
    allow_picker_fallback: bool,
) {
    if !request.file_path.trim().is_empty() {
        if let Some(behavior) = behavior_pipeline.open_audio_behavior.as_ref() {
            behavior.apply(request.clone());
        }
        apply_open_audio_request(view_model, audio_runtime, request.file_path.as_str());
        if let Some(request_open_audio) = handlers.request_open_audio.as_ref() {
            request_open_audio(request);
        }
        view_model.dispatch(ChoreoMainAction::OpenAudioPanel);
        return;
    }

    if let Some(request_open_audio) = handlers.request_open_audio.as_ref() {
        request_open_audio(request);
        view_model.dispatch(ChoreoMainAction::OpenAudioPanel);
        return;
    }

    if allow_picker_fallback
        && let Some(pick_audio_path) = handlers.pick_audio_path.as_ref()
        && let Some(file_path) = pick_audio_path()
    {
        route_open_audio_request(
            view_model,
            audio_runtime,
            OpenAudioRequested {
                file_path,
                trace_context: request.trace_context,
            },
            handlers,
            behavior_pipeline,
            false,
        );
    }
}

fn apply_open_audio_request(
    view_model: &mut MainViewModel,
    audio_runtime: &mut AudioPlayerRuntime,
    file_path: &str,
) {
    if file_path.trim().is_empty() {
        return;
    }

    let file_exists = Path::new(file_path).is_file();
    audio_runtime.set_backend(view_model.state().settings_state.audio_player_backend);
    if file_exists {
        audio_runtime.open_file(file_path.to_string());
    } else {
        audio_runtime.close();
    }

    view_model.dispatch(ChoreoMainAction::AudioPlayerAction(
        AudioPlayerAction::OpenAudioFile {
            file_path: file_path.to_string(),
            file_exists,
        },
    ));

    if file_exists && let Some(sample) = audio_runtime.sample() {
        crate::audio_player::runtime::apply_player_sample(
            &mut view_model.state_mut().audio_player_state,
            sample,
        );
    }
}

pub(crate) fn apply_audio_action_side_effects(
    view_model: &mut MainViewModel,
    audio_runtime: &mut AudioPlayerRuntime,
    action: &ChoreoMainAction,
) {
    match action {
        ChoreoMainAction::AudioPlayerAction(AudioPlayerAction::TogglePlayPause) => {
            audio_runtime.toggle_play_pause();
        }
        ChoreoMainAction::AudioPlayerAction(AudioPlayerAction::Stop) => {
            audio_runtime.stop();
        }
        ChoreoMainAction::AudioPlayerAction(AudioPlayerAction::SeekToPosition { position }) => {
            audio_runtime.seek(*position);
        }
        ChoreoMainAction::AudioPlayerAction(AudioPlayerAction::PositionDragStarted) => {
            if view_model
                .state()
                .audio_player_state
                .was_playing_before_drag
            {
                audio_runtime.pause();
            }
        }
        ChoreoMainAction::AudioPlayerAction(AudioPlayerAction::PositionDragCompleted {
            position,
        }) => {
            if view_model.state().audio_player_state.is_playing {
                audio_runtime.seek_and_play(*position);
            } else {
                audio_runtime.seek(*position);
            }
        }
        ChoreoMainAction::AudioPlayerAction(AudioPlayerAction::SpeedChanged { speed }) => {
            audio_runtime.set_speed(*speed);
        }
        _ => {}
    }
}

pub(crate) fn poll_audio_runtime(
    view_model: &mut MainViewModel,
    audio_runtime: &mut AudioPlayerRuntime,
) -> bool {
    let Some(sample) = audio_runtime.sample() else {
        return false;
    };

    let state = view_model.state_mut();
    apply_player_sample_without_position(&mut state.audio_player_state, sample);

    let effects = crate::audio_player::reducer::reduce(
        &mut state.audio_player_state,
        AudioPlayerAction::PlayerPositionSampled {
            position: sample.position,
        },
    );
    debug_assert!(effects.is_empty());

    let effects = crate::audio_player::reducer::reduce(
        &mut state.audio_player_state,
        AudioPlayerAction::PublishPositionIfChanged,
    );
    for effect in effects {
        match effect {
            crate::audio_player::reducer::AudioPlayerEffect::PositionChangedPublished {
                position_seconds,
            } => {
                sync_audio_position_internal(state, position_seconds);
            }
        }
    }

    state.audio_player_state.has_player
        && (state.audio_player_state.is_playing
            || state.audio_player_state.pending_seek_position.is_some())
}

fn map_selected_scene_state(scene: &choreo_models::SceneModel) -> SelectedSceneState {
    SelectedSceneState {
        scene_id: scene.scene_id,
        name: scene.name.clone(),
        text: scene.text.clone().unwrap_or_default(),
        fixed_positions: scene.fixed_positions,
        timestamp: scene.timestamp.as_deref().and_then(parse_timestamp_seconds),
        color: scene.color.clone(),
    }
}

fn map_audio_player_scenes(choreography: &ChoreographyModel) -> Vec<AudioPlayerScene> {
    choreography
        .scenes
        .iter()
        .map(|scene| AudioPlayerScene {
            scene_id: scene.scene_id.0,
            name: scene.name.clone(),
            timestamp: scene.timestamp.as_deref().and_then(parse_timestamp_seconds),
        })
        .collect()
}

fn map_audio_player_choreography_scenes(
    choreography: &ChoreographyModel,
) -> Vec<AudioPlayerChoreographyScene> {
    choreography
        .scenes
        .iter()
        .map(|scene| AudioPlayerChoreographyScene {
            scene_id: scene.scene_id.0,
            timestamp: scene.timestamp.clone(),
        })
        .collect()
}

fn resolve_audio_request(
    choreography: &ChoreographyModel,
    choreography_file_path: Option<&str>,
) -> Option<String> {
    if let Some(path) = choreography.settings.music_path_absolute.as_ref()
        && !path.trim().is_empty()
    {
        return Some(path.clone());
    }

    if let Some(relative) = choreography.settings.music_path_relative.as_ref()
        && !relative.trim().is_empty()
        && let Some(file_path) = choreography_file_path
    {
        let base_dir = Path::new(file_path)
            .parent()
            .unwrap_or_else(|| Path::new(""));
        return Some(base_dir.join(relative).to_string_lossy().into_owned());
    }

    None
}

fn apply_save_choreo_request(view_model: &mut MainViewModel, file_path: &str) {
    if file_path.trim().is_empty() {
        return;
    }

    let path = Path::new(file_path);
    if !path.exists() {
        return;
    }

    let mut choreography = view_model
        .state()
        .choreography_settings_state
        .choreography
        .clone();
    choreography.last_save_date = crate::time::SystemClock::now_utc();
    let mapper = ChoreographyModelMapper;
    let json_model = mapper.map_to_json(&choreography);
    if export_to_file(path, &json_model).is_err() {
        return;
    }

    view_model
        .state_mut()
        .choreography_settings_state
        .choreography
        .last_save_date = choreography.last_save_date;
}

pub(crate) fn enqueue_open_audio_request(
    view_model: &mut MainViewModel,
    request: OpenAudioRequested,
) {
    view_model.request_open_audio(request);
}

