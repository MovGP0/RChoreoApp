use std::sync::mpsc::channel;

use crate::audio_player::audio_player_component::AudioPlayerBackend;
use crate::audio_player::audio_player_component::OpenAudioFileCommand;
use crate::audio_player::audio_player_component::audio_player_behaviors::AudioPlayerBehaviorDependencies;
use crate::audio_player::audio_player_component::build_audio_player_behaviors;
use crate::audio_player::audio_player_component::messages::CloseAudioFileCommand;
use crate::audio_player::audio_player_component::messages::LinkSceneToPositionCommand;
use crate::audio_player::audio_player_component::runtime::AudioPlayerRuntime;
use crate::audio_player::audio_player_component::state::AudioPlayerChoreographyScene;
use crate::audio_player::audio_player_component::state::AudioPlayerScene;
use crate::audio_player::audio_player_component::state::AudioPlayerState;
use crate::audio_player::audio_player_component::reducer::reduce;
use crate::audio_player::audio_player_component::actions::AudioPlayerAction;

#[test]
fn build_pipeline_handles_open_and_close_commands() {
    let (open_tx, open_rx) = channel();
    let (close_tx, close_rx) = channel();
    let (link_tx, link_rx) = channel();
    let (position_tx, _position_rx) = channel();

    let pipeline = build_audio_player_behaviors(AudioPlayerBehaviorDependencies {
        open_audio_receiver: open_rx,
        close_audio_receiver: close_rx,
        position_changed_senders: vec![position_tx],
        link_scene_receiver: link_rx,
        backend: AudioPlayerBackend::Rodio,
        haptic_feedback: None,
    });

    let mut state = AudioPlayerState::default();
    let mut runtime = AudioPlayerRuntime::new(AudioPlayerBackend::Rodio);

    open_tx
        .send(OpenAudioFileCommand {
            file_path: "C:\\temp\\sample.mp3".to_string(),
            trace_context: None,
        })
        .expect("open command should send");
    pipeline.open_audio_file.poll(&mut state, &mut runtime);

    assert!(state.has_player);
    assert!(state.has_stream_factory);
    assert_eq!(
        state.last_opened_audio_file_path.as_deref(),
        Some("C:\\temp\\sample.mp3")
    );

    close_tx
        .send(CloseAudioFileCommand {
            trace_context: None,
        })
        .expect("close command should send");
    pipeline.close_audio_file.poll(&mut state, &mut runtime);

    assert!(!state.has_player);
    assert!(!runtime.has_player());

    let _ = link_tx;
}

#[test]
fn pipeline_links_scene_and_publishes_position_changed_event() {
    let (open_tx, open_rx) = channel();
    let (close_tx, close_rx) = channel();
    let (link_tx, link_rx) = channel();
    let (position_tx, position_rx) = channel();

    let pipeline = build_audio_player_behaviors(AudioPlayerBehaviorDependencies {
        open_audio_receiver: open_rx,
        close_audio_receiver: close_rx,
        position_changed_senders: vec![position_tx],
        link_scene_receiver: link_rx,
        backend: AudioPlayerBackend::Rodio,
        haptic_feedback: None,
    });

    let mut state = AudioPlayerState {
        position: 2.1,
        selected_scene_id: Some(2),
        scenes: vec![
            AudioPlayerScene {
                scene_id: 1,
                name: "A".to_string(),
                timestamp: Some(1.0),
            },
            AudioPlayerScene {
                scene_id: 2,
                name: "B".to_string(),
                timestamp: None,
            },
            AudioPlayerScene {
                scene_id: 3,
                name: "C".to_string(),
                timestamp: Some(5.0),
            },
        ],
        choreography_scenes: vec![
            AudioPlayerChoreographyScene {
                scene_id: 2,
                timestamp: None,
            },
        ],
        ..AudioPlayerState::default()
    };

    let _ = reduce(&mut state, AudioPlayerAction::UpdateTicksAndLinkState);
    link_tx
        .send(LinkSceneToPositionCommand {
            trace_context: None,
        })
        .expect("link command should send");
    pipeline.link_scene.poll(&mut state, None);

    assert_eq!(state.scenes[1].timestamp, Some(2.1));
    assert_eq!(
        state.choreography_scenes[0].timestamp.as_deref(),
        Some("2.1")
    );

    state.position = 3.0;
    pipeline.position_changed.poll(&mut state);
    let event = position_rx.recv().expect("position event should be sent");
    assert_eq!(event.position_seconds, 3.0);
    assert!(event.trace_context.is_none());

    let _ = open_tx;
    let _ = close_tx;
}
