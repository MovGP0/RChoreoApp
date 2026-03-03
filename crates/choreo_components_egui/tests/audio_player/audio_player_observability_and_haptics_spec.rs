use std::sync::Arc;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use std::sync::mpsc::channel;

use crate::audio_player::audio_player_component::AudioPlayerBackend;
use crate::audio_player::audio_player_component::actions::AudioPlayerAction;
use crate::audio_player::audio_player_component::audio_player_behaviors::AudioPlayerBehaviorDependencies;
use crate::audio_player::audio_player_component::audio_player_behaviors::AudioPlayerHapticFeedback;
use crate::audio_player::audio_player_component::audio_player_behaviors::reduce_with_haptics;
use crate::audio_player::audio_player_component::build_audio_player_behaviors;
use crate::audio_player::audio_player_component::messages::LinkSceneToPositionCommand;
use crate::audio_player::audio_player_component::reducer::reduce;
use crate::audio_player::audio_player_component::state::AudioPlayerChoreographyScene;
use crate::audio_player::audio_player_component::state::AudioPlayerScene;
use crate::audio_player::audio_player_component::state::AudioPlayerState;
use crate::observability::TraceContext;

struct MockHaptic {
    click_count: Arc<AtomicUsize>,
}

impl AudioPlayerHapticFeedback for MockHaptic {
    fn is_supported(&self) -> bool {
        true
    }

    fn perform_click(&self) {
        self.click_count.fetch_add(1, Ordering::Relaxed);
    }
}

#[test]
fn toggle_play_pause_triggers_haptic_click_side_effect() {
    let mut state = AudioPlayerState {
        has_player: true,
        ..AudioPlayerState::default()
    };
    let click_count = Arc::new(AtomicUsize::new(0));
    let haptic = MockHaptic {
        click_count: Arc::clone(&click_count),
    };

    reduce_with_haptics(
        &mut state,
        AudioPlayerAction::TogglePlayPause,
        Some(&haptic),
    );

    assert!(state.is_playing);
    assert_eq!(click_count.load(Ordering::Relaxed), 1);
}

#[test]
fn link_command_keeps_trace_context_and_propagates_to_position_event() {
    let (open_tx, open_rx) = channel();
    let (close_tx, close_rx) = channel();
    let (link_tx, link_rx) = channel();
    let (position_tx, position_rx) = channel();
    let click_count = Arc::new(AtomicUsize::new(0));
    let haptic = Box::new(MockHaptic {
        click_count: Arc::clone(&click_count),
    });

    let pipeline = build_audio_player_behaviors(AudioPlayerBehaviorDependencies {
        open_audio_receiver: open_rx,
        close_audio_receiver: close_rx,
        position_changed_senders: vec![position_tx],
        link_scene_receiver: link_rx,
        backend: AudioPlayerBackend::Rodio,
        haptic_feedback: Some(haptic),
    });

    let trace_context = TraceContext {
        trace_id_hex: Some("abc123".to_string()),
        span_id_hex: Some("def456".to_string()),
    };

    let mut state = AudioPlayerState {
        position: 2.0,
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
        choreography_scenes: vec![AudioPlayerChoreographyScene {
            scene_id: 2,
            timestamp: None,
        }],
        ..AudioPlayerState::default()
    };

    let _ = reduce(&mut state, AudioPlayerAction::UpdateTicksAndLinkState);
    link_tx
        .send(LinkSceneToPositionCommand {
            trace_context: Some(trace_context.clone()),
        })
        .expect("link command should send");
    pipeline
        .link_scene
        .poll(&mut state, pipeline.haptic_feedback.as_deref());

    assert_eq!(click_count.load(Ordering::Relaxed), 1);
    assert_eq!(state.last_trace_context, Some(trace_context));

    state.position = 3.0;
    pipeline.position_changed.poll(&mut state);
    let event = position_rx.recv().expect("position event should be sent");
    assert_eq!(
        event.trace_context,
        Some(TraceContext {
            trace_id_hex: Some("abc123".to_string()),
            span_id_hex: Some("def456".to_string()),
        })
    );

    let _ = open_tx;
    let _ = close_tx;
}
