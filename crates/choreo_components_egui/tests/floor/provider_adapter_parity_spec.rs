use crate::floor::floor_component::AudioInterpolationInput;
use crate::floor::floor_component::FloorAdapter;
use crate::floor::floor_component::FloorAdapterInput;
use crate::floor::floor_component::FloorPointerEventSenders;
use crate::floor::floor_component::FloorProvider;
use crate::floor::floor_component::FloorProviderDependencies;
use crate::floor::floor_component::FloorRenderGateImpl;
use crate::floor::floor_component::state::FloorPosition;
use crate::floor::floor_component::state::FloorState;
use std::sync::Arc;

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

    assert_eq!(state.positions.len(), 2);
    assert_eq!(state.axis_labels.len(), 2);
    assert_eq!(state.legend_entries.len(), 1);
    assert_eq!(state.placement_remaining, Some(3));
    assert_eq!(state.svg_path.as_deref(), Some("floor.svg"));
    assert_eq!(state.interpolated_positions.len(), 2);
    assert!((state.interpolated_positions[0].x - 6.0).abs() < 0.001);
    assert!(view_model.has_floor_bounds());
    assert_eq!(view_model.canvas_size().width, 1200.0);
    assert!(!state.path_commands.is_empty());
    assert!(!state.dashed_path_commands.is_empty());
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
    assert!(provider.floor_render_gate().is_rendered());
    assert!(provider.state().draw_count >= 2);
    provider.floor_view_model().borrow().request_redraw();
    assert!(provider.state().draw_count >= 2);
    provider.tick();
    assert!(provider.state().draw_count >= 2);

    provider.deactivate();
}
