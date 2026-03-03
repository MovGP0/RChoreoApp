use crate::floor::floor_component::actions::FloorAction;
use crate::floor::floor_component::reducer::reduce;
use crate::floor::floor_component::state::CanvasViewHandle;
use crate::floor::floor_component::state::FloorState;
use crate::floor::floor_component::state::InteractionMode;
use crate::floor::floor_component::state::Point;
use crate::floor::floor_component::state::PointerButton;
use crate::floor::floor_component::state::PointerEventArgs;
use crate::floor::floor_component::state::TouchAction;
use crate::floor::floor_component::state::TouchDeviceType;
use crate::floor::floor_component::state::TouchEventArgs;

#[test]
fn pointer_event_args_exposes_position_and_button() {
    let args = PointerEventArgs {
        position: Point::new(12.0, 34.0),
        button: PointerButton::Primary,
        is_in_contact: true,
    };

    assert_eq!(args.position, Point::new(12.0, 34.0));
    assert_eq!(args.button, PointerButton::Primary);
}

#[test]
fn floor_variants_and_ui_are_exercised_for_reducer_coverage() {
    let mut state = FloorState::default();
    reduce(&mut state, FloorAction::Initialize);
    reduce(
        &mut state,
        FloorAction::SetInteractionMode {
            mode: InteractionMode::Move,
        },
    );
    reduce(
        &mut state,
        FloorAction::SetInteractionMode {
            mode: InteractionMode::RotateAroundCenter,
        },
    );
    reduce(
        &mut state,
        FloorAction::SetInteractionMode {
            mode: InteractionMode::RotateAroundDancer,
        },
    );
    reduce(
        &mut state,
        FloorAction::SetInteractionMode {
            mode: InteractionMode::Scale,
        },
    );
    reduce(
        &mut state,
        FloorAction::SetInteractionMode {
            mode: InteractionMode::Place,
        },
    );
    reduce(&mut state, FloorAction::ResetViewport);
    reduce(
        &mut state,
        FloorAction::Touch {
            id: 1,
            action: TouchAction::Released,
            point: Point::new(0.0, 0.0),
            is_in_contact: false,
            device: TouchDeviceType::Touch,
        },
    );
    let secondary_args = PointerEventArgs {
        position: Point::new(0.0, 0.0),
        button: PointerButton::Secondary,
        is_in_contact: false,
    };
    assert_eq!(secondary_args.button, PointerButton::Secondary);

    let canvas = CanvasViewHandle { id: 7 };
    reduce(
        &mut state,
        FloorAction::PointerPressedWithContext {
            canvas_view: canvas,
            event_args: PointerEventArgs {
                position: Point::new(12.0, 12.0),
                button: PointerButton::Primary,
                is_in_contact: true,
            },
        },
    );
    reduce(
        &mut state,
        FloorAction::PointerMovedWithContext {
            canvas_view: canvas,
            event_args: PointerEventArgs {
                position: Point::new(14.0, 15.0),
                button: PointerButton::Primary,
                is_in_contact: true,
            },
        },
    );
    reduce(
        &mut state,
        FloorAction::PointerReleasedWithContext {
            canvas_view: canvas,
            event_args: PointerEventArgs {
                position: Point::new(14.0, 15.0),
                button: PointerButton::Primary,
                is_in_contact: false,
            },
        },
    );
    reduce(
        &mut state,
        FloorAction::PointerWheelChangedWithContext {
            canvas_view: canvas,
            delta_x: 0.0,
            delta_y: 16.0,
            control_modifier: true,
            position: Some(Point::new(10.0, 10.0)),
        },
    );
    reduce(
        &mut state,
        FloorAction::TouchWithContext {
            canvas_view: canvas,
            event_args: TouchEventArgs {
                id: 99,
                action: TouchAction::Pressed,
                device_type: TouchDeviceType::Touch,
                location: Point::new(20.0, 20.0),
                in_contact: true,
            },
        },
    );
    assert_eq!(state.last_canvas_view, Some(canvas));
    assert!(state.last_pointer_pressed.is_some());
    assert!(state.last_pointer_moved.is_some());
    assert!(state.last_pointer_released.is_some());
    assert!(state.last_touch_event.is_some());

    let context = egui::Context::default();
    let _ = context.run(egui::RawInput::default(), |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            let _ = crate::floor::floor_component::ui::draw(ui, &state);
        });
    });
}

#[test]
fn pointer_pressed_with_secondary_button_preserves_event_contract() {
    let mut state = FloorState::default();
    let canvas = CanvasViewHandle { id: 9 };

    reduce(
        &mut state,
        FloorAction::PointerPressedWithContext {
            canvas_view: canvas,
            event_args: PointerEventArgs {
                position: Point::new(36.0, 48.0),
                button: PointerButton::Secondary,
                is_in_contact: false,
            },
        },
    );

    let last_pressed = state
        .last_pointer_pressed
        .expect("pointer context should store latest pointer args");
    assert_eq!(last_pressed.button, PointerButton::Secondary);
    assert!(!last_pressed.is_in_contact);
    assert_eq!(state.last_canvas_view, Some(canvas));
}

#[test]
fn pointer_wheel_context_stores_metadata_and_applies_pan() {
    let mut state = FloorState::default();
    let canvas = CanvasViewHandle { id: 11 };

    reduce(
        &mut state,
        FloorAction::PointerWheelChangedWithContext {
            canvas_view: canvas,
            delta_x: 12.0,
            delta_y: -24.0,
            control_modifier: false,
            position: Some(Point::new(120.0, 132.0)),
        },
    );

    assert_eq!(state.last_canvas_view, Some(canvas));
    assert!(!state.last_wheel_control_modifier);
    assert_eq!(state.last_wheel_position, Some(Point::new(120.0, 132.0)));
    assert_eq!(state.transformation_matrix.trans_x, 12.0);
    assert_eq!(state.transformation_matrix.trans_y, -24.0);
}

#[test]
fn cancelled_touch_clears_active_touch_state_and_keeps_context() {
    let mut state = FloorState::default();
    let canvas = CanvasViewHandle { id: 12 };
    reduce(
        &mut state,
        FloorAction::PointerPressed {
            point: Point::new(10.0, 10.0),
        },
    );
    reduce(
        &mut state,
        FloorAction::Touch {
            id: 1,
            action: TouchAction::Pressed,
            point: Point::new(0.0, 0.0),
            is_in_contact: true,
            device: TouchDeviceType::Touch,
        },
    );
    reduce(
        &mut state,
        FloorAction::Touch {
            id: 2,
            action: TouchAction::Pressed,
            point: Point::new(24.0, 0.0),
            is_in_contact: true,
            device: TouchDeviceType::Touch,
        },
    );
    assert!(!state.active_touches.is_empty());

    reduce(
        &mut state,
        FloorAction::TouchWithContext {
            canvas_view: canvas,
            event_args: TouchEventArgs {
                id: 2,
                action: TouchAction::Cancelled,
                device_type: TouchDeviceType::Pen,
                location: Point::new(24.0, 0.0),
                in_contact: false,
            },
        },
    );

    assert_eq!(state.last_canvas_view, Some(canvas));
    let last_touch = state
        .last_touch_event
        .expect("cancelled touch event should be retained");
    assert_eq!(last_touch.action, TouchAction::Cancelled);
    assert_eq!(last_touch.device_type, TouchDeviceType::Pen);
    assert!(state.active_touches.is_empty());
    assert!(state.pinch_distance.is_none());
    assert!(state.pointer_anchor.is_none());
}
