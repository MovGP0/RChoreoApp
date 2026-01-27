use std::collections::HashMap;

use crate::behavior::{Behavior, CompositeDisposable};
use choreo_state_machine::{
    ApplicationStateMachine,
    PanCompletedTrigger,
    PanStartedTrigger,
    StateKind,
    ZoomCompletedTrigger,
    ZoomStartedTrigger,
};
use nject::injectable;

use super::floor_view_model::FloorCanvasViewModel;
use super::messages::{
    PointerButton,
    PointerMovedCommand,
    PointerPressedCommand,
    PointerReleasedCommand,
    PointerWheelChangedCommand,
    TouchAction,
    TouchCommand,
    TouchDeviceType,
};
use super::types::{Matrix, Point};

#[derive(Debug, Default)]
#[injectable]
#[inject(|| Self::default())]
pub struct GestureHandlingBehavior {
    active_touches: HashMap<i64, Point>,
    last_hover_position: Option<Point>,
    last_pointer_position: Option<Point>,
    last_single_touch_position: Option<Point>,
    last_touch_center: Option<Point>,
    last_touch_distance: Option<f64>,
    touch_pan_active: bool,
    touch_zoom_active: bool,
}

impl GestureHandlingBehavior {
    pub const TOUCH_PAN_FACTOR: f32 = 0.5;

    pub fn handle_pointer_pressed(
        &mut self,
        state_machine: &mut ApplicationStateMachine,
        command: PointerPressedCommand,
    ) {
        if Self::is_gesture_blocked(state_machine) {
            return;
        }

        self.last_hover_position = Some(command.event_args.position);

        if command.event_args.button != PointerButton::Primary {
            self.last_pointer_position = None;
            return;
        }

        self.last_pointer_position = Some(command.event_args.position);
    }

    pub fn handle_pointer_moved(
        &mut self,
        view_model: &mut FloorCanvasViewModel,
        state_machine: &mut ApplicationStateMachine,
        command: PointerMovedCommand,
    ) {
        if Self::is_gesture_blocked(state_machine) {
            return;
        }

        self.last_hover_position = Some(command.event_args.position);

        let Some(last) = self.last_pointer_position else {
            return;
        };

        let delta_x = (command.event_args.position.x - last.x) as f32;
        let delta_y = (command.event_args.position.y - last.y) as f32;
        let _ = state_machine.try_apply(&PanStartedTrigger);
        Self::apply_translation(view_model, delta_x, delta_y);
        self.last_pointer_position = Some(command.event_args.position);
    }

    pub fn handle_pointer_released(
        &mut self,
        state_machine: &mut ApplicationStateMachine,
        _command: PointerReleasedCommand,
    ) {
        if Self::is_gesture_blocked(state_machine) {
            return;
        }

        let _ = state_machine.try_apply(&PanCompletedTrigger);
        self.last_pointer_position = None;
    }

    pub fn handle_pointer_wheel_changed(
        &mut self,
        view_model: &mut FloorCanvasViewModel,
        state_machine: &mut ApplicationStateMachine,
        command: PointerWheelChangedCommand,
    ) {
        if Self::is_gesture_blocked(state_machine) {
            return;
        }

        let _ = state_machine.try_apply(&ZoomStartedTrigger);
        let zoom_factor = if command.delta > 0.0 { 1.1 } else { 0.9 };
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

    pub fn handle_touch(
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
        let points = self.active_touches.values().take(2).copied().collect::<Vec<_>>();
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

        if let (Some(last_center), Some(last_distance)) = (self.last_touch_center, self.last_touch_distance) {
            let delta_x = ((center.x - last_center.x) as f32) * Self::TOUCH_PAN_FACTOR;
            let delta_y = ((center.y - last_center.y) as f32) * Self::TOUCH_PAN_FACTOR;
            Self::apply_translation(view_model, delta_x, delta_y);

            if last_distance > 0.0 {
                let scale = distance / last_distance;
                if scale.is_finite() && scale > 0.0 {
                    let scale_matrix = Matrix::scale(
                        scale as f32,
                        scale as f32,
                        center.x as f32,
                        center.y as f32,
                    );
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
        let touch_point = *self.active_touches.values().next().unwrap_or(&Point::new(0.0, 0.0));
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
    }

    fn apply_translation(view_model: &mut FloorCanvasViewModel, delta_x: f32, delta_y: f32) {
        let translation = Matrix::translation(delta_x, delta_y);
        Self::apply_transformation(view_model, translation);
    }

    fn apply_transformation(view_model: &mut FloorCanvasViewModel, matrix: Matrix) {
        let next = view_model.transformation_matrix.concat(&matrix);
        view_model.transformation_matrix = next;
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
    fn activate(&self, _view_model: &mut FloorCanvasViewModel, _disposables: &mut CompositeDisposable) {
    }
}

