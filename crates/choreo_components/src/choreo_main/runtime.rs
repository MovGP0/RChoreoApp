use std::path::Path;

use choreo_master_mobile_json::export_to_file;
use choreo_master_mobile_json::import;
use choreo_models::ChoreographyModel;
use choreo_models::ChoreographyModelMapper;

use crate::audio_player::actions::AudioPlayerAction;
use crate::audio_player::runtime::AudioPlayerRuntime;
use crate::audio_player::runtime::apply_player_sample;
use crate::audio_player::runtime::apply_player_sample_without_position;
use crate::audio_player::state::AudioPlayerChoreographyScene;
use crate::audio_player::state::AudioPlayerScene;
use crate::audio_player::types::AudioPlayerSample;
use crate::choreography_settings::actions::ChoreographySettingsAction;
use crate::choreography_settings::state::SelectedSceneState;
use crate::scenes::state::parse_timestamp_seconds;
use crate::settings::actions::SettingsAction;

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
        ChoreoMainAction::SettingsAction(SettingsAction::UpdateAudioPlayerBackend { backend }) => {
            switch_audio_backend(view_model, audio_runtime, *backend);
        }
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

fn switch_audio_backend(
    view_model: &mut MainViewModel,
    audio_runtime: &mut AudioPlayerRuntime,
    backend: crate::audio_player::AudioPlayerBackend,
) {
    let backend = backend.normalize_for_current_target();
    if audio_runtime.backend() == backend {
        return;
    }

    let had_player = audio_runtime.has_player();
    let previous_sample = audio_runtime.sample();
    audio_runtime.set_backend(backend);

    if !had_player {
        return;
    }

    let Some(file_path) = view_model
        .state()
        .audio_player_state
        .last_opened_audio_file_path
        .clone()
    else {
        return;
    };

    apply_open_audio_request(view_model, audio_runtime, file_path.as_str());
    restore_audio_player_sample(audio_runtime, previous_sample);

    if let Some(sample) = audio_runtime.sample() {
        apply_player_sample(&mut view_model.state_mut().audio_player_state, sample);
    }
}

fn restore_audio_player_sample(
    audio_runtime: &mut AudioPlayerRuntime,
    sample: Option<AudioPlayerSample>,
) {
    let Some(sample) = sample else {
        return;
    };

    audio_runtime.set_speed(sample.speed);
    audio_runtime.set_volume(sample.volume);
    audio_runtime.set_balance(sample.balance);
    audio_runtime.set_loop(sample.loop_enabled);

    if sample.is_playing {
        audio_runtime.seek_and_play(sample.position);
    } else if sample.position > 0.0 {
        audio_runtime.seek(sample.position);
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

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;
    use std::path::PathBuf;
    use std::time::SystemTime;
    use std::time::UNIX_EPOCH;

    use crate::audio_player::AudioPlayerBackend;
    use crate::audio_player::runtime::AudioPlayerRuntime;
    use crate::choreo_main::MainViewModel;
    use crate::settings::actions::SettingsAction;

    use super::apply_audio_action_side_effects;
    use super::apply_open_audio_request;

    #[test]
    fn updating_audio_backend_in_settings_switches_runtime_backend_for_open_audio() {
        let mut view_model = MainViewModel::new(Vec::new());
        let mut runtime = AudioPlayerRuntime::new(AudioPlayerBackend::Rodio);
        let path = unique_temp_file("wav");
        write_test_wav(&path);
        let file_path = path.to_string_lossy().into_owned();

        apply_open_audio_request(&mut view_model, &mut runtime, file_path.as_str());

        view_model.dispatch(
            crate::choreo_main::actions::ChoreoMainAction::SettingsAction(
                SettingsAction::UpdateAudioPlayerBackend {
                    backend: AudioPlayerBackend::Awedio,
                },
            ),
        );

        let action = crate::choreo_main::actions::ChoreoMainAction::SettingsAction(
            SettingsAction::UpdateAudioPlayerBackend {
                backend: AudioPlayerBackend::Awedio,
            },
        );
        apply_audio_action_side_effects(&mut view_model, &mut runtime, &action);

        assert_eq!(runtime.backend(), AudioPlayerBackend::Awedio);
        assert!(runtime.has_player());

        let _ = fs::remove_file(path);
    }

    fn unique_temp_file(extension: &str) -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time should be after unix epoch")
            .as_nanos();
        let mut path = std::env::temp_dir();
        path.push(format!("rchoreo_audio_backend_switch_{nanos}.{extension}"));
        path
    }

    fn write_test_wav(path: &Path) {
        let sample_rate = 8_000_u32;
        let sample_count = 8_000_usize;
        let data_size = (sample_count * std::mem::size_of::<i16>()) as u32;
        let mut bytes = Vec::with_capacity(44 + data_size as usize);
        bytes.extend_from_slice(b"RIFF");
        bytes.extend_from_slice(&(36 + data_size).to_le_bytes());
        bytes.extend_from_slice(b"WAVE");
        bytes.extend_from_slice(b"fmt ");
        bytes.extend_from_slice(&16_u32.to_le_bytes());
        bytes.extend_from_slice(&1_u16.to_le_bytes());
        bytes.extend_from_slice(&1_u16.to_le_bytes());
        bytes.extend_from_slice(&sample_rate.to_le_bytes());
        bytes.extend_from_slice(&(sample_rate * 2).to_le_bytes());
        bytes.extend_from_slice(&2_u16.to_le_bytes());
        bytes.extend_from_slice(&16_u16.to_le_bytes());
        bytes.extend_from_slice(b"data");
        bytes.extend_from_slice(&data_size.to_le_bytes());
        for index in 0..sample_count {
            let sample = if index % 32 < 16 {
                i16::MAX / 6
            } else {
                -(i16::MAX / 6)
            };
            bytes.extend_from_slice(&sample.to_le_bytes());
        }
        fs::write(path, bytes).expect("test wav file should be written");
    }
}
