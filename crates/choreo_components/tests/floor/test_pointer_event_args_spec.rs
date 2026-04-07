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

#[test]
fn pointer_event_args_exposes_position_and_button() {
    let args = PointerEventArgs {
        position: Point::new(12.0, 34.0),
        button: PointerButton::Primary,
        is_in_contact: true,
    };

    let mut errors = Vec::new();

    check_eq!(errors, args.position, Point::new(12.0, 34.0));
    check_eq!(errors, args.button, PointerButton::Primary);

    assert_no_errors(errors);
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

    let mut errors = Vec::new();

    check_eq!(errors, secondary_args.button, PointerButton::Secondary);

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
    check_eq!(errors, state.last_canvas_view, Some(canvas));
    check!(errors, state.last_pointer_pressed.is_some());
    check!(errors, state.last_pointer_moved.is_some());
    check!(errors, state.last_pointer_released.is_some());
    check!(errors, state.last_touch_event.is_some());

    assert_no_errors(errors);

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
    let mut errors = Vec::new();

    check_eq!(errors, last_pressed.button, PointerButton::Secondary);
    check!(errors, !last_pressed.is_in_contact);
    check_eq!(errors, state.last_canvas_view, Some(canvas));

    assert_no_errors(errors);
}

#[test]
fn secondary_pointer_does_not_start_pan_anchor() {
    let mut state = FloorState::default();
    let canvas = CanvasViewHandle { id: 10 };

    reduce(
        &mut state,
        FloorAction::PointerPressedWithContext {
            canvas_view: canvas,
            event_args: PointerEventArgs {
                position: Point::new(24.0, 24.0),
                button: PointerButton::Secondary,
                is_in_contact: true,
            },
        },
    );
    reduce(
        &mut state,
        FloorAction::PointerMovedWithContext {
            canvas_view: canvas,
            event_args: PointerEventArgs {
                position: Point::new(120.0, 180.0),
                button: PointerButton::Secondary,
                is_in_contact: true,
            },
        },
    );

    let mut errors = Vec::new();

    check!(errors, state.pointer_anchor.is_none());
    check_eq!(errors, state.transformation_matrix.trans_x, 0.0);
    check_eq!(errors, state.transformation_matrix.trans_y, 0.0);

    assert_no_errors(errors);
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

    let mut errors = Vec::new();

    check_eq!(errors, state.last_canvas_view, Some(canvas));
    check!(errors, !state.last_wheel_control_modifier);
    check_eq!(errors, state.last_wheel_position, Some(Point::new(120.0, 132.0)));
    check_eq!(errors, state.transformation_matrix.trans_x, 12.0);
    check_eq!(errors, state.transformation_matrix.trans_y, -24.0);

    assert_no_errors(errors);
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
    let mut errors = Vec::new();

    check!(errors, !state.active_touches.is_empty());

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

    let last_touch = state
        .last_touch_event
        .expect("cancelled touch event should be retained");
    check_eq!(errors, state.last_canvas_view, Some(canvas));
    check_eq!(errors, last_touch.action, TouchAction::Cancelled);
    check_eq!(errors, last_touch.device_type, TouchDeviceType::Pen);
    check!(errors, state.active_touches.is_empty());
    check!(errors, state.pinch_distance.is_none());
    check!(errors, state.pointer_anchor.is_none());

    assert_no_errors(errors);
}

#[test]
fn non_touch_device_touch_events_do_not_mutate_active_gesture_state() {
    let mut state = FloorState::default();
    reduce(
        &mut state,
        FloorAction::Touch {
            id: 1,
            action: TouchAction::Pressed,
            point: Point::new(10.0, 10.0),
            is_in_contact: true,
            device: TouchDeviceType::Mouse,
        },
    );
    reduce(
        &mut state,
        FloorAction::Touch {
            id: 2,
            action: TouchAction::Pressed,
            point: Point::new(20.0, 20.0),
            is_in_contact: true,
            device: TouchDeviceType::Pen,
        },
    );

    let mut errors = Vec::new();

    check_eq!(errors, state.last_touch_device, Some(TouchDeviceType::Pen));
    check!(errors, state.active_touches.is_empty());
    check!(errors, state.pinch_distance.is_none());

    assert_no_errors(errors);
}
