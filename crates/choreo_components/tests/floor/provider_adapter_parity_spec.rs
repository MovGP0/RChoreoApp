use crate::floor::floor_component::AudioInterpolationInput;
use crate::floor::floor_component::FloorAdapter;
use crate::floor::floor_component::FloorAdapterInput;
use crate::floor::floor_component::FloorPointerEventSenders;
use crate::floor::floor_component::FloorProvider;
use crate::floor::floor_component::FloorProviderDependencies;
use crate::floor::floor_component::FloorRenderGate;
use crate::floor::floor_component::FloorRenderGateImpl;
use crate::floor::floor_component::state::FloorPosition;
use crate::floor::floor_component::state::FloorState;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

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

#[derive(Default)]
struct CountingRenderGate {
    rendered: AtomicBool,
    marks: AtomicUsize,
}

impl FloorRenderGate for CountingRenderGate {
    fn is_rendered(&self) -> bool {
        self.rendered.load(Ordering::SeqCst)
    }

    fn mark_rendered(&self) {
        self.rendered.store(true, Ordering::SeqCst);
        self.marks.fetch_add(1, Ordering::SeqCst);
    }

    fn wait_for_first_render(&self) {}
}

#[test]
fn adapter_maps_scene_overlay_and_audio_interpolation_into_state() {
    let adapter = FloorAdapter::new();
    let mut state = FloorState::default();
    let (draw_sender, _draw_receiver) = std::sync::mpsc::sync_channel(4);
    let mut view_model = crate::floor::floor_component::FloorCanvasViewModel::new(
        draw_sender,
        FloorPointerEventSenders {
            pointer_pressed_senders: Vec::new(),
            pointer_moved_senders: Vec::new(),
            pointer_released_senders: Vec::new(),
            pointer_wheel_changed_senders: Vec::new(),
            touch_senders: Vec::new(),
        },
    );

    adapter.apply(
        &mut state,
        &mut view_model,
        FloorAdapterInput {
            scene_positions: vec![FloorPosition::new(0.0, 0.0), FloorPosition::new(24.0, 36.0)],
            axis_x_label: "X Axis".to_string(),
            axis_y_label: "Y Axis".to_string(),
            legend_entries: vec![("Lead".to_string(), [255, 64, 64, 255])],
            svg_path: Some("floor.svg".to_string()),
            placement_remaining: Some(3),
            interpolation: Some(AudioInterpolationInput {
                from: vec![FloorPosition::new(0.0, 0.0), FloorPosition::new(24.0, 36.0)],
                to: vec![
                    FloorPosition::new(12.0, 0.0),
                    FloorPosition::new(24.0, 48.0),
                ],
                progress: 0.5,
            }),
            layout_size: Some((1200.0, 720.0)),
        },
    );

    let mut errors = Vec::new();

    check_eq!(errors, state.positions.len(), 2);
    check_eq!(errors, state.axis_labels.len(), 2);
    check_eq!(errors, state.legend_entries.len(), 1);
    check_eq!(errors, state.placement_remaining, Some(3));
    check_eq!(errors, state.svg_path.as_deref(), Some("floor.svg"));
    check_eq!(errors, state.interpolated_positions.len(), 2);
    check!(
        errors,
        (state.interpolated_positions[0].x - 6.0).abs() < 0.001
    );
    check!(errors, view_model.has_floor_bounds());
    check_eq!(errors, view_model.canvas_size().width, 1200.0);
    check!(errors, !state.path_commands.is_empty());
    check!(errors, !state.dashed_path_commands.is_empty());

    assert_no_errors(errors);
}

#[test]
fn provider_orchestrates_draw_and_redraw_with_render_gate() {
    let render_gate = Arc::new(FloorRenderGateImpl::new());
    let mut provider = FloorProvider::new(FloorProviderDependencies {
        state: FloorState::default(),
        floor_adapter: FloorAdapter::new(),
        floor_render_gate: render_gate,
        view_model_behaviors: Vec::new(),
        floor_event_senders: FloorPointerEventSenders {
            pointer_pressed_senders: Vec::new(),
            pointer_moved_senders: Vec::new(),
            pointer_released_senders: Vec::new(),
            pointer_wheel_changed_senders: Vec::new(),
            touch_senders: Vec::new(),
        },
    });

    provider.activate();
    provider.floor_view_model().borrow_mut().draw_floor();
    provider.tick();
    let mut errors = Vec::new();

    check!(errors, provider.floor_render_gate().is_rendered());
    check!(errors, provider.state().draw_count >= 2);
    provider.floor_view_model().borrow().request_redraw();
    check!(errors, provider.state().draw_count >= 2);
    provider.tick();
    check!(errors, provider.state().draw_count >= 2);

    assert_no_errors(errors);

    provider.deactivate();
}

#[test]
fn provider_request_redraw_does_not_mark_render_gate_before_draw_floor_command() {
    let render_gate = Arc::new(CountingRenderGate::default());
    let mut provider = FloorProvider::new(FloorProviderDependencies {
        state: FloorState::default(),
        floor_adapter: FloorAdapter::new(),
        floor_render_gate: render_gate.clone(),
        view_model_behaviors: Vec::new(),
        floor_event_senders: FloorPointerEventSenders {
            pointer_pressed_senders: Vec::new(),
            pointer_moved_senders: Vec::new(),
            pointer_released_senders: Vec::new(),
            pointer_wheel_changed_senders: Vec::new(),
            touch_senders: Vec::new(),
        },
    });

    provider.activate();
    provider.floor_view_model().borrow().request_redraw();
    let mut errors = Vec::new();

    check!(errors, !render_gate.is_rendered());
    check_eq!(errors, render_gate.marks.load(Ordering::SeqCst), 0);
    check_eq!(errors, provider.state().draw_count, 1);

    provider.floor_view_model().borrow_mut().draw_floor();
    provider.tick();

    check!(errors, render_gate.is_rendered());
    check_eq!(errors, render_gate.marks.load(Ordering::SeqCst), 1);

    assert_no_errors(errors);

    provider.deactivate();
}
