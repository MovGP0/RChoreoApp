use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::time::{Duration, Instant};

use crate::behavior::TimerDisposable;
use crate::behavior::{Behavior, CompositeDisposable};
use crate::logging::BehaviorLog;
use choreo_state_machine::{
    ApplicationStateMachine, PanCompletedTrigger, PanStartedTrigger, StateKind,
    ZoomCompletedTrigger, ZoomStartedTrigger,
};
use crossbeam_channel::Receiver;
use nject::injectable;
use slint::TimerMode;

use super::floor_view_model::FloorCanvasViewModel;
use super::messages::{
    PointerButton, PointerMovedCommand, PointerPressedCommand, PointerReleasedCommand,
    PointerWheelChangedCommand, TouchAction, TouchCommand, TouchDeviceType,
};
use super::types::{Matrix, Point};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
enum GestureArbitrationState {
    #[default]
    Idle,
    Pointer,
    Touch,
}

#[derive(Default, Clone)]
#[injectable]
#[inject(
    |state_machine: Rc<RefCell<ApplicationStateMachine>>,
     pointer_pressed_receiver: Receiver<PointerPressedCommand>,
     pointer_moved_receiver: Receiver<PointerMovedCommand>,
     pointer_released_receiver: Receiver<PointerReleasedCommand>,
     pointer_wheel_changed_receiver: Receiver<PointerWheelChangedCommand>,
     touch_receiver: Receiver<TouchCommand>| {
        Self::new(
            state_machine,
            pointer_pressed_receiver,
            pointer_moved_receiver,
            pointer_released_receiver,
            pointer_wheel_changed_receiver,
            touch_receiver,
        )
    }
)]
pub struct GestureHandlingBehavior {
    state_machine: Option<Rc<RefCell<ApplicationStateMachine>>>,
    pointer_pressed_receiver: Option<Receiver<PointerPressedCommand>>,
    pointer_moved_receiver: Option<Receiver<PointerMovedCommand>>,
    pointer_released_receiver: Option<Receiver<PointerReleasedCommand>>,
    pointer_wheel_changed_receiver: Option<Receiver<PointerWheelChangedCommand>>,
    touch_receiver: Option<Receiver<TouchCommand>>,
    active_touches: HashMap<i64, Point>,
    last_hover_position: Option<Point>,
    last_pointer_position: Option<Point>,
    pointer_press_origin: Option<Point>,
    pointer_moved_since_press: bool,
    last_tap_position: Option<Point>,
    last_tap_at: Option<Instant>,
    last_single_touch_position: Option<Point>,
    last_touch_center: Option<Point>,
    last_touch_distance: Option<f64>,
    touch_pan_active: bool,
    touch_zoom_active: bool,
    arbitration_state: GestureArbitrationState,
    pointer_blocked_until: Option<Instant>,
}

impl GestureHandlingBehavior {
    pub const TOUCH_PAN_FACTOR: f32 = 0.5;
    const WHEEL_NOTCH_DELTA: f64 = 120.0;
    const WHEEL_NOTCH_EPSILON: f64 = 0.5;
    const DOUBLE_TAP_WINDOW: Duration = Duration::from_millis(350);
    const DOUBLE_TAP_MAX_DISTANCE: f64 = 24.0;
    const TAP_MOVE_TOLERANCE: f64 = 8.0;
    const POINTER_BLOCK_AFTER_TOUCH: Duration = Duration::from_millis(180);

    pub fn new(
        state_machine: Rc<RefCell<ApplicationStateMachine>>,
        pointer_pressed_receiver: Receiver<PointerPressedCommand>,
        pointer_moved_receiver: Receiver<PointerMovedCommand>,
        pointer_released_receiver: Receiver<PointerReleasedCommand>,
        pointer_wheel_changed_receiver: Receiver<PointerWheelChangedCommand>,
        touch_receiver: Receiver<TouchCommand>,
    ) -> Self {
        Self {
            state_machine: Some(state_machine),
            pointer_pressed_receiver: Some(pointer_pressed_receiver),
            pointer_moved_receiver: Some(pointer_moved_receiver),
            pointer_released_receiver: Some(pointer_released_receiver),
            pointer_wheel_changed_receiver: Some(pointer_wheel_changed_receiver),
            touch_receiver: Some(touch_receiver),
            ..Self::default()
        }
    }

    fn handle_pointer_pressed(
        &mut self,
        state_machine: &mut ApplicationStateMachine,
        command: PointerPressedCommand,
    ) {
        if Self::is_gesture_blocked(state_machine) {
            return;
        }

        self.last_hover_position = Some(command.event_args.position);
        if self.is_pointer_blocked() {
            self.last_pointer_position = None;
            self.pointer_press_origin = None;
            self.pointer_moved_since_press = false;
            return;
        }

        if command.event_args.button != PointerButton::Primary {
            self.last_pointer_position = None;
            return;
        }

        self.arbitration_state = GestureArbitrationState::Pointer;
        self.last_pointer_position = Some(command.event_args.position);
        self.pointer_press_origin = Some(command.event_args.position);
        self.pointer_moved_since_press = false;
    }

    fn handle_pointer_moved(
        &mut self,
        view_model: &mut FloorCanvasViewModel,
        state_machine: &mut ApplicationStateMachine,
        command: PointerMovedCommand,
    ) {
        if Self::is_gesture_blocked(state_machine) {
            return;
        }

        self.last_hover_position = Some(command.event_args.position);
        if self.is_pointer_blocked() {
            self.last_pointer_position = None;
            self.pointer_press_origin = None;
            self.pointer_moved_since_press = false;
            return;
        }

        let Some(last) = self.last_pointer_position else {
            return;
        };

        if let Some(origin) = self.pointer_press_origin {
            let movement = Self::distance_between(origin, command.event_args.position);
            if movement > Self::TAP_MOVE_TOLERANCE {
                self.pointer_moved_since_press = true;
            }
        }

        let delta_x = (command.event_args.position.x - last.x) as f32;
        let delta_y = (command.event_args.position.y - last.y) as f32;
        let _ = state_machine.try_apply(&PanStartedTrigger);
        Self::apply_translation(view_model, delta_x, delta_y);
        self.last_pointer_position = Some(command.event_args.position);
    }

    fn handle_pointer_released(
        &mut self,
        view_model: &mut FloorCanvasViewModel,
        state_machine: &mut ApplicationStateMachine,
        command: PointerReleasedCommand,
    ) {
        if Self::is_gesture_blocked(state_machine) {
            return;
        }
        if self.is_pointer_blocked() {
            self.last_pointer_position = None;
            self.pointer_press_origin = None;
            self.pointer_moved_since_press = false;
            return;
        }

        let _ = state_machine.try_apply(&PanCompletedTrigger);
        self.try_handle_tap_release(view_model, command);
        self.last_pointer_position = None;
        self.pointer_press_origin = None;
        self.pointer_moved_since_press = false;
        self.arbitration_state = GestureArbitrationState::Idle;
    }

    fn handle_pointer_wheel_changed(
        &mut self,
        view_model: &mut FloorCanvasViewModel,
        state_machine: &mut ApplicationStateMachine,
        command: PointerWheelChangedCommand,
    ) {
        if Self::is_gesture_blocked(state_machine) {
            return;
        }

        if self.is_pointer_blocked() {
            return;
        }

        if Self::should_pan_with_wheel(&command) {
            self.block_pointer_temporarily();
            let _ = state_machine.try_apply(&PanStartedTrigger);
            let delta_x = command.delta_x as f32;
            let delta_y = command.delta_y as f32;
            Self::apply_translation(view_model, delta_x, delta_y);
            let _ = state_machine.try_apply(&PanCompletedTrigger);
            return;
        }

        let _ = state_machine.try_apply(&ZoomStartedTrigger);
        let zoom_factor = if command.delta_y > 0.0 { 1.1 } else { 0.9 };
        let zoom_center = command.position.or(self.last_hover_position);
        let Some(center) = zoom_center else {
            return;
        };

        let scale_matrix = Matrix::scale(
            zoom_factor as f32,
            zoom_factor as f32,
            center.x as f32,
            center.y as f32,
        );
        Self::apply_transformation(view_model, scale_matrix);
        let _ = state_machine.try_apply(&ZoomCompletedTrigger);
    }

    fn should_pan_with_wheel(command: &PointerWheelChangedCommand) -> bool {
        if command.control_modifier {
            return false;
        }

        if command.delta_x.abs() > f64::EPSILON {
            return true;
        }

        !Self::is_notched_wheel_delta(command.delta_y)
    }

    fn is_notched_wheel_delta(delta: f64) -> bool {
        let magnitude = delta.abs();
        if magnitude <= f64::EPSILON {
            return false;
        }

        let notch_count = (magnitude / Self::WHEEL_NOTCH_DELTA).round();
        if notch_count < 1.0 {
            return false;
        }

        let expected = notch_count * Self::WHEEL_NOTCH_DELTA;
        (magnitude - expected).abs() <= Self::WHEEL_NOTCH_EPSILON
    }

    fn handle_touch(
        &mut self,
        view_model: &mut FloorCanvasViewModel,
        state_machine: &mut ApplicationStateMachine,
        command: TouchCommand,
    ) {
        if Self::is_gesture_blocked(state_machine) {
            return;
        }

        let args = command.event_args;
        if args.device_type != TouchDeviceType::Touch {
            return;
        }
        self.transition_to_touch_mode();

        match args.action {
            TouchAction::Pressed => {
                self.active_touches.insert(args.id, args.location);
            }
            TouchAction::Moved => {
                self.active_touches.insert(args.id, args.location);
            }
            TouchAction::Released | TouchAction::Cancelled => {
                self.active_touches.remove(&args.id);
            }
        }

        if self.active_touches.is_empty() {
            self.reset_touch_state(state_machine);
            return;
        }

        if self.active_touches.len() == 1 {
            if self.touch_zoom_active {
                let _ = state_machine.try_apply(&ZoomCompletedTrigger);
                self.touch_zoom_active = false;
                self.last_touch_center = None;
                self.last_touch_distance = None;
            }

            self.handle_single_touch_pan(view_model, state_machine);
            return;
        }

        self.last_single_touch_position = None;
        let points = self
            .active_touches
            .values()
            .take(2)
            .copied()
            .collect::<Vec<_>>();
        let first = points[0];
        let second = points[1];
        let center = Point::new((first.x + second.x) / 2.0, (first.y + second.y) / 2.0);
        let dx = second.x - first.x;
        let dy = second.y - first.y;
        let distance = (dx * dx + dy * dy).sqrt();

        if !self.touch_zoom_active {
            self.touch_zoom_active = true;
            let _ = state_machine.try_apply(&ZoomStartedTrigger);
        }

        if let (Some(last_center), Some(last_distance)) =
            (self.last_touch_center, self.last_touch_distance)
        {
            let delta_x = ((center.x - last_center.x) as f32) * Self::TOUCH_PAN_FACTOR;
            let delta_y = ((center.y - last_center.y) as f32) * Self::TOUCH_PAN_FACTOR;
            Self::apply_translation(view_model, delta_x, delta_y);

            if last_distance > 0.0 {
                let scale = distance / last_distance;
                if scale.is_finite() && scale > 0.0 {
                    let scale_matrix =
                        Matrix::scale(scale as f32, scale as f32, center.x as f32, center.y as f32);
                    Self::apply_transformation(view_model, scale_matrix);
                }
            }
        }

        self.last_touch_center = Some(center);
        self.last_touch_distance = Some(distance);
    }

    fn handle_single_touch_pan(
        &mut self,
        view_model: &mut FloorCanvasViewModel,
        state_machine: &mut ApplicationStateMachine,
    ) {
        let touch_point = *self
            .active_touches
            .values()
            .next()
            .unwrap_or(&Point::new(0.0, 0.0));
        let current = touch_point;

        let Some(last) = self.last_single_touch_position else {
            self.last_single_touch_position = Some(current);
            return;
        };

        if !self.touch_pan_active {
            self.touch_pan_active = true;
            let _ = state_machine.try_apply(&PanStartedTrigger);
        }

        let delta_x = ((current.x - last.x) as f32) * Self::TOUCH_PAN_FACTOR;
        let delta_y = ((current.y - last.y) as f32) * Self::TOUCH_PAN_FACTOR;
        Self::apply_translation(view_model, delta_x, delta_y);
        self.last_single_touch_position = Some(current);
    }

    fn reset_touch_state(&mut self, state_machine: &mut ApplicationStateMachine) {
        if self.touch_pan_active {
            let _ = state_machine.try_apply(&PanCompletedTrigger);
        }

        if self.touch_zoom_active {
            let _ = state_machine.try_apply(&ZoomCompletedTrigger);
        }

        self.touch_pan_active = false;
        self.touch_zoom_active = false;
        self.last_single_touch_position = None;
        self.last_touch_center = None;
        self.last_touch_distance = None;
        self.arbitration_state = GestureArbitrationState::Idle;
        self.block_pointer_temporarily();
    }

    fn apply_translation(view_model: &mut FloorCanvasViewModel, delta_x: f32, delta_y: f32) {
        let translation = Matrix::translation(delta_x, delta_y);
        Self::apply_transformation(view_model, translation);
    }

    fn apply_transformation(view_model: &mut FloorCanvasViewModel, matrix: Matrix) {
        let next = view_model.transformation_matrix.concat(&matrix);
        view_model.transformation_matrix = next;
    }

    fn try_handle_tap_release(
        &mut self,
        view_model: &mut FloorCanvasViewModel,
        command: PointerReleasedCommand,
    ) {
        if command.event_args.button != PointerButton::Primary || self.pointer_moved_since_press {
            return;
        }

        let tap_position = command.event_args.position;
        let now = Instant::now();
        let is_double_tap = self.last_tap_at.is_some_and(|last_tap_at| {
            now.duration_since(last_tap_at) <= Self::DOUBLE_TAP_WINDOW
                && self.last_tap_position.is_some_and(|last_tap_position| {
                    Self::distance_between(last_tap_position, tap_position)
                        <= Self::DOUBLE_TAP_MAX_DISTANCE
                })
        });

        if is_double_tap {
            view_model.reset_viewport();
            self.last_tap_at = None;
            self.last_tap_position = None;
            return;
        }

        self.last_tap_at = Some(now);
        self.last_tap_position = Some(tap_position);
    }

    fn distance_between(first: Point, second: Point) -> f64 {
        let dx = first.x - second.x;
        let dy = first.y - second.y;
        (dx * dx + dy * dy).sqrt()
    }

    fn transition_to_touch_mode(&mut self) {
        self.arbitration_state = GestureArbitrationState::Touch;
        self.block_pointer_temporarily();
        self.last_pointer_position = None;
        self.pointer_press_origin = None;
        self.pointer_moved_since_press = false;
    }

    fn block_pointer_temporarily(&mut self) {
        self.pointer_blocked_until = Some(Instant::now() + Self::POINTER_BLOCK_AFTER_TOUCH);
    }

    fn is_pointer_blocked(&self) -> bool {
        if self.arbitration_state == GestureArbitrationState::Touch {
            return true;
        }

        self.pointer_blocked_until
            .is_some_and(|blocked_until| Instant::now() < blocked_until)
    }

    fn is_gesture_blocked(state_machine: &ApplicationStateMachine) -> bool {
        matches!(
            state_machine.state().kind(),
            StateKind::MovePositionsState
                | StateKind::RotateAroundCenterState
                | StateKind::ScalePositionsState
                | StateKind::ScaleAroundDancerState
        )
    }
}

impl Behavior<FloorCanvasViewModel> for GestureHandlingBehavior {
    fn activate(
        &self,
        view_model: &mut FloorCanvasViewModel,
        disposables: &mut CompositeDisposable,
    ) {
        BehaviorLog::behavior_activated("GestureHandlingBehavior", "FloorCanvasViewModel");
        let Some(state_machine) = self.state_machine.clone() else {
            return;
        };
        let Some(view_model_handle) = view_model.self_handle().and_then(|handle| handle.upgrade())
        else {
            return;
        };
        let Some(pointer_pressed_receiver) = self.pointer_pressed_receiver.clone() else {
            return;
        };
        let Some(pointer_moved_receiver) = self.pointer_moved_receiver.clone() else {
            return;
        };
        let Some(pointer_released_receiver) = self.pointer_released_receiver.clone() else {
            return;
        };
        let Some(pointer_wheel_changed_receiver) = self.pointer_wheel_changed_receiver.clone()
        else {
            return;
        };
        let Some(touch_receiver) = self.touch_receiver.clone() else {
            return;
        };
        let behavior = Rc::new(RefCell::new(self.clone()));
        let timer = slint::Timer::default();
        timer.start(TimerMode::Repeated, Duration::from_millis(16), move || {
            while let Ok(command) = pointer_pressed_receiver.try_recv() {
                let mut behavior = behavior.borrow_mut();
                let mut state_machine = state_machine.borrow_mut();
                behavior.handle_pointer_pressed(&mut state_machine, command);
            }

            while let Ok(command) = pointer_moved_receiver.try_recv() {
                let mut behavior = behavior.borrow_mut();
                let mut view_model = view_model_handle.borrow_mut();
                let mut state_machine = state_machine.borrow_mut();
                behavior.handle_pointer_moved(&mut view_model, &mut state_machine, command);
                view_model.draw_floor();
            }

            while let Ok(command) = pointer_released_receiver.try_recv() {
                let mut behavior = behavior.borrow_mut();
                let mut view_model = view_model_handle.borrow_mut();
                let mut state_machine = state_machine.borrow_mut();
                behavior.handle_pointer_released(&mut view_model, &mut state_machine, command);
            }

            while let Ok(command) = pointer_wheel_changed_receiver.try_recv() {
                let mut behavior = behavior.borrow_mut();
                let mut view_model = view_model_handle.borrow_mut();
                let mut state_machine = state_machine.borrow_mut();
                behavior.handle_pointer_wheel_changed(&mut view_model, &mut state_machine, command);
                view_model.draw_floor();

                let delayed_view_model = Rc::clone(&view_model_handle);
                slint::Timer::single_shot(Duration::from_millis(16), move || {
                    delayed_view_model.borrow_mut().draw_floor();
                });
            }

            while let Ok(command) = touch_receiver.try_recv() {
                let mut behavior = behavior.borrow_mut();
                let mut view_model = view_model_handle.borrow_mut();
                let mut state_machine = state_machine.borrow_mut();
                behavior.handle_touch(&mut view_model, &mut state_machine, command);
                view_model.draw_floor();
            }
        });

        disposables.add(Box::new(TimerDisposable::new(timer)));
    }
}
