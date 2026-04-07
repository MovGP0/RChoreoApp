use std::rc::Rc;
use std::sync::Arc;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use std::sync::mpsc::channel;

use choreo_master_mobile_json::Color;
use choreo_master_mobile_json::SceneId;
use choreo_models::ChoreographyModel;
use choreo_models::SceneModel;

use choreo_components::audio_player::AudioPlayerBackend;
use choreo_components::audio_player::AudioPlayerHapticFeedback;
use choreo_components::audio_player::AudioPlayerPipelineDependencies;
use choreo_components::audio_player::actions::AudioPlayerAction;
use choreo_components::audio_player::build_audio_player_pipeline;
use choreo_components::audio_player::messages::LinkSceneToPositionCommand;
use choreo_components::audio_player::reduce_with_haptics;
use choreo_components::audio_player::state::AudioPlayerState;
use choreo_components::global::GlobalStateActor;
use choreo_components::global::SceneViewModel;
use choreo_components::observability::TraceContext;
use choreo_components::preferences::InMemoryPreferences;

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

    let mut errors = Vec::new();

    check!(errors, state.is_playing);
    check_eq!(errors, click_count.load(Ordering::Relaxed), 1);

    assert_no_errors(errors);
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
    let mut errors = Vec::new();

    let global_state = GlobalStateActor::new();
    check!(errors, global_state.try_update(|state| {
        state.scenes = vec![
            scene_view_model(1, "A", Some(1.0)),
            scene_view_model(2, "B", None),
            scene_view_model(3, "C", Some(5.0)),
        ];
        state.selected_scene = state.scenes.get(1).cloned();
        state.choreography = ChoreographyModel {
            scenes: vec![
                scene_model(1, "A", Some("1.0")),
                scene_model(2, "B", None),
                scene_model(3, "C", Some("5.0")),
            ],
            ..ChoreographyModel::default()
        };
    }));

    let pipeline = build_audio_player_pipeline(AudioPlayerPipelineDependencies {
        global_state_store: Rc::clone(&global_state),
        open_audio_receiver: open_rx,
        close_audio_receiver: close_rx,
        position_changed_senders: vec![position_tx],
        link_scene_receiver: link_rx,
        preferences: Rc::new(InMemoryPreferences::new()),
        haptic_feedback: Some(haptic),
    });

    let trace_context = TraceContext {
        trace_id_hex: Some("abc123".to_string()),
        span_id_hex: Some("def456".to_string()),
    };

    let mut state = AudioPlayerState {
        position: 2.0,
        ..AudioPlayerState::default()
    };
    let mut runtime = choreo_components::audio_player::runtime::AudioPlayerRuntime::new(
        AudioPlayerBackend::Rodio,
    );
    pipeline.ticks.poll(&mut state, &mut runtime);

    check!(
        errors,
        link_tx
            .send(LinkSceneToPositionCommand {
                trace_context: Some(trace_context.clone()),
            })
            .is_ok()
    );
    pipeline
        .link_scene
        .poll(&mut state, pipeline.haptic_feedback.as_deref());

    check_eq!(errors, click_count.load(Ordering::Relaxed), 1);
    check_eq!(errors, state.last_trace_context, Some(trace_context.clone()));

    state.position = 3.0;
    pipeline.position_changed.poll(&mut state);
    let event = match position_rx.recv() {
        Ok(event) => Some(event),
        Err(err) => {
            errors.push(format!("position event should be sent: {err}"));
            None
        }
    };

    check_eq!(
        errors,
        event.as_ref().map(|event| event.trace_context.clone()),
        Some(Some(TraceContext {
            trace_id_hex: Some("abc123".to_string()),
            span_id_hex: Some("def456".to_string()),
        }))
    );

    assert_no_errors(errors);

    let _ = open_tx;
    let _ = close_tx;
}

fn scene_model(scene_id: i32, name: &str, timestamp: Option<&str>) -> SceneModel {
    SceneModel {
        scene_id: SceneId(scene_id),
        positions: Vec::new(),
        name: name.to_string(),
        text: None,
        fixed_positions: false,
        timestamp: timestamp.map(str::to_string),
        variation_depth: 0,
        variations: Vec::new(),
        current_variation: Vec::new(),
        color: Color::transparent(),
    }
}

fn scene_view_model(scene_id: i32, name: &str, timestamp: Option<f64>) -> SceneViewModel {
    let mut scene = SceneViewModel::new(SceneId(scene_id), name, Color::transparent());
    scene.timestamp = timestamp;
    scene
}
