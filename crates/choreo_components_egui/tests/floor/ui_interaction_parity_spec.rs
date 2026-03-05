use crate::floor::floor_component::actions::FloorAction;
use crate::floor::floor_component::reducer::reduce;
use crate::floor::floor_component::state::FloorState;
use crate::floor::floor_component::state::Point;

fn draw_actions(state: &FloorState, raw_input: egui::RawInput) -> Vec<FloorAction> {
    let context = egui::Context::default();
    let mut actions: Vec<FloorAction> = Vec::new();
    let _ = context.run(raw_input, |ctx| {
        let ui_actions = egui::Area::new("floor-ui-interaction-parity".into())
            .fixed_pos(egui::pos2(0.0, 0.0))
            .show(ctx, |ui| {
                ui.set_min_size(egui::vec2(640.0, 480.0));
                crate::floor::floor_component::ui::draw(ui, state)
            })
            .inner;
        actions.extend(ui_actions);
    });
    actions
}

#[test]
fn draw_emits_pointer_wheel_and_touch_actions_for_canvas_events() {
    let state = FloorState::default();

    let raw_input = egui::RawInput {
        screen_rect: Some(egui::Rect::from_min_size(
            egui::pos2(0.0, 0.0),
            egui::vec2(640.0, 480.0),
        )),
        events: vec![
            egui::Event::PointerButton {
                pos: egui::pos2(200.0, 150.0),
                button: egui::PointerButton::Primary,
                pressed: true,
                modifiers: egui::Modifiers::NONE,
            },
            egui::Event::PointerMoved(egui::pos2(230.0, 170.0)),
            egui::Event::MouseWheel {
                unit: egui::MouseWheelUnit::Point,
                delta: egui::vec2(0.0, 120.0),
                modifiers: egui::Modifiers {
                    ctrl: true,
                    ..egui::Modifiers::NONE
                },
            },
            egui::Event::Touch {
                device_id: egui::TouchDeviceId(1),
                id: egui::TouchId(4),
                phase: egui::TouchPhase::Start,
                pos: egui::pos2(210.0, 155.0),
                force: None,
            },
            egui::Event::PointerButton {
                pos: egui::pos2(230.0, 170.0),
                button: egui::PointerButton::Primary,
                pressed: false,
                modifiers: egui::Modifiers::NONE,
            },
        ],
        ..egui::RawInput::default()
    };

    let actions = draw_actions(&state, raw_input);

    assert!(
        actions
            .iter()
            .any(|action| matches!(action, FloorAction::PointerPressedWithContext { .. }))
    );
    assert!(
        actions
            .iter()
            .any(|action| matches!(action, FloorAction::PointerMovedWithContext { .. }))
    );
    assert!(
        actions
            .iter()
            .any(|action| matches!(action, FloorAction::PointerReleasedWithContext { .. }))
    );
    assert!(actions.iter().any(|action| matches!(
        action,
        FloorAction::PointerWheelChangedWithContext {
            control_modifier: true,
            ..
        }
    )));
    assert!(
        actions
            .iter()
            .any(|action| matches!(action, FloorAction::TouchWithContext { .. }))
    );
}

#[test]
fn pan_and_zoom_recompute_layout_bounds() {
    let mut state = FloorState::default();
    reduce(
        &mut state,
        FloorAction::SetLayout {
            width_px: 1000.0,
            height_px: 700.0,
        },
    );

    let base_floor_x = state.floor_x;
    let base_floor_y = state.floor_y;
    let base_floor_width = state.floor_width;
    let base_floor_height = state.floor_height;

    reduce(
        &mut state,
        FloorAction::PointerWheelChanged {
            delta_x: 0.0,
            delta_y: 120.0,
            ctrl: true,
            cursor: Some(Point::new(500.0, 350.0)),
        },
    );
    assert!(state.floor_width > base_floor_width);
    assert!(state.floor_height > base_floor_height);
    let floor_x_after_zoom = state.floor_x;
    let floor_y_after_zoom = state.floor_y;

    reduce(
        &mut state,
        FloorAction::PointerPressed {
            point: Point::new(200.0, 200.0),
        },
    );
    reduce(
        &mut state,
        FloorAction::PointerMoved {
            point: Point::new(260.0, 220.0),
        },
    );

    assert!(state.floor_x > floor_x_after_zoom);
    assert!(state.floor_y > floor_y_after_zoom);
    assert_ne!(state.floor_x, base_floor_x);
    assert_ne!(state.floor_y, base_floor_y);
}

#[test]
fn secondary_pointer_input_keeps_button_context_and_does_not_start_pan() {
    let state = FloorState::default();

    let raw_input = egui::RawInput {
        screen_rect: Some(egui::Rect::from_min_size(
            egui::pos2(0.0, 0.0),
            egui::vec2(640.0, 480.0),
        )),
        events: vec![
            egui::Event::PointerButton {
                pos: egui::pos2(200.0, 150.0),
                button: egui::PointerButton::Secondary,
                pressed: true,
                modifiers: egui::Modifiers::NONE,
            },
            egui::Event::PointerMoved(egui::pos2(260.0, 190.0)),
            egui::Event::PointerButton {
                pos: egui::pos2(260.0, 190.0),
                button: egui::PointerButton::Secondary,
                pressed: false,
                modifiers: egui::Modifiers::NONE,
            },
        ],
        ..egui::RawInput::default()
    };

    let actions = draw_actions(&state, raw_input);
    assert!(
        actions
            .iter()
            .all(|action| !matches!(action, FloorAction::ClearSelection))
    );

    let mut reduced = FloorState::default();
    for action in actions {
        reduce(&mut reduced, action);
    }

    let last_pressed = reduced
        .last_pointer_pressed
        .expect("secondary press metadata should be preserved");
    assert_eq!(
        last_pressed.button,
        crate::floor::floor_component::state::PointerButton::Secondary
    );
    assert_eq!(reduced.transformation_matrix.trans_x, 0.0);
    assert_eq!(reduced.transformation_matrix.trans_y, 0.0);
    assert!(reduced.pointer_anchor.is_none());
}
