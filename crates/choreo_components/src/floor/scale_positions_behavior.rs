use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

use crossbeam_channel::Receiver;
use crate::behavior::{Behavior, CompositeDisposable};
use crate::behavior::TimerDisposable;
use crate::global::{GlobalStateModel, GlobalStateActor, InteractionMode, SelectionRectangle};
use crate::logging::BehaviorLog;
use choreo_models::PositionModel;
use choreo_state_machine::{
    ApplicationStateMachine,
    ScalePositionsDragCompletedTrigger,
    ScalePositionsDragStartedTrigger,
    ScalePositionsSelectionCompletedTrigger,
    ScalePositionsSelectionStartedTrigger,
    StateKind,
};
use nject::injectable;
use slint::TimerMode;

use super::floor_view_model::FloorCanvasViewModel;
use super::messages::{PointerButton, PointerMovedCommand, PointerPressedCommand, PointerReleasedCommand};
use super::types::Point;

#[derive(Default, Clone)]
#[injectable]
#[inject(
    |global_state: Rc<GlobalStateActor>,
     state_machine: Rc<RefCell<ApplicationStateMachine>>,
     pointer_pressed_receiver: Receiver<PointerPressedCommand>,
     pointer_moved_receiver: Receiver<PointerMovedCommand>,
     pointer_released_receiver: Receiver<PointerReleasedCommand>| {
        Self::new(
            global_state,
            state_machine,
            pointer_pressed_receiver,
            pointer_moved_receiver,
            pointer_released_receiver,
        )
    }
)]
pub struct ScalePositionsBehavior {
    global_state: Option<Rc<GlobalStateActor>>,
    state_machine: Option<Rc<RefCell<ApplicationStateMachine>>>,
    pointer_pressed_receiver: Option<Receiver<PointerPressedCommand>>,
    pointer_moved_receiver: Option<Receiver<PointerMovedCommand>>,
    pointer_released_receiver: Option<Receiver<PointerReleasedCommand>>,
    pointer_pressed_position: Option<Point>,
    pointer_moved: bool,
    selection_active: bool,
    scale_active: bool,
    clear_selection_on_release: bool,
    scale_start_positions: Vec<(usize, Point)>,
    scale_center: Option<Point>,
    scale_start_distance: Option<f64>,
    last_scale_floor_point: Option<Point>,
}

impl ScalePositionsBehavior {
    pub fn new(
        global_state: Rc<GlobalStateActor>,
        state_machine: Rc<RefCell<ApplicationStateMachine>>,
        pointer_pressed_receiver: Receiver<PointerPressedCommand>,
        pointer_moved_receiver: Receiver<PointerMovedCommand>,
        pointer_released_receiver: Receiver<PointerReleasedCommand>,
    ) -> Self
    {
        Self {
            global_state: Some(global_state),
            state_machine: Some(state_machine),
            pointer_pressed_receiver: Some(pointer_pressed_receiver),
            pointer_moved_receiver: Some(pointer_moved_receiver),
            pointer_released_receiver: Some(pointer_released_receiver),
            ..Self::default()
        }
    }

    fn handle_pointer_pressed(
        &mut self,
        view_model: &FloorCanvasViewModel,
        global_state: &mut GlobalStateModel,
        state_machine: &mut ApplicationStateMachine,
        command: PointerPressedCommand,
    ) {
        if command.event_args.button != PointerButton::Primary {
            self.reset_pointer_state();
            return;
        }

        self.pointer_pressed_position = Some(command.event_args.position);
        self.pointer_moved = false;

        if !Self::is_scale_mode_active(global_state, state_machine) {
            return;
        }

        let Some(floor_point) =
            Self::try_get_floor_point(view_model, global_state, command.event_args.position)
        else {
            self.clear_selection_on_release = true;
            return;
        };

        if !global_state.selected_positions.is_empty() && !self.selection_active {
            self.start_scale(global_state, state_machine, floor_point);
            return;
        }

        self.start_selection(global_state, state_machine, floor_point);
    }

    fn handle_pointer_moved(
        &mut self,
        view_model: &FloorCanvasViewModel,
        global_state: &mut GlobalStateModel,
        state_machine: &mut ApplicationStateMachine,
        command: PointerMovedCommand,
    ) {
        if self.pointer_pressed_position.is_none() {
            return;
        }

        if !Self::is_scale_mode_active(global_state, state_machine) {
            return;
        }

        let Some(floor_point) =
            Self::try_get_floor_point(view_model, global_state, command.event_args.position)
        else {
            return;
        };

        if let Some(pressed) = self.pointer_pressed_position {
            let delta_x = command.event_args.position.x - pressed.x;
            let delta_y = command.event_args.position.y - pressed.y;
            let distance = (delta_x * delta_x + delta_y * delta_y).sqrt();
            if distance > 0.0 {
                self.pointer_moved = true;
            }
        }

        if self.scale_active && self.pointer_moved {
            self.update_scale(global_state, floor_point);
            return;
        }

        if self.selection_active {
            self.update_selection(global_state, floor_point);
        }
    }

    fn handle_pointer_released(
        &mut self,
        view_model: &FloorCanvasViewModel,
        global_state: &mut GlobalStateModel,
        state_machine: &mut ApplicationStateMachine,
        command: PointerReleasedCommand,
    ) {
        if self.pointer_pressed_position.is_none() {
            return;
        }

        if !Self::is_scale_mode_active(global_state, state_machine) {
            self.reset_pointer_state();
            return;
        }

        if let Some(_floor_point) =
            Self::try_get_floor_point(view_model, global_state, command.event_args.position)
        {
            if self.scale_active {
                if self.pointer_moved {
                    self.complete_scale(global_state, state_machine);
                } else {
                    self.clear_selection(global_state, state_machine);
                }
            } else if self.selection_active {
                self.complete_selection(global_state, state_machine);
            }
        } else {
            if self.clear_selection_on_release {
                self.clear_selection(global_state, state_machine);
            }

            if self.selection_active {
                self.complete_selection(global_state, state_machine);
            }
        }

        self.reset_pointer_state();
    }

    fn start_scale(
        &mut self,
        global_state: &mut GlobalStateModel,
        state_machine: &mut ApplicationStateMachine,
        floor_point: Point,
    ) {
        let Some(scene) = global_state.selected_scene.as_mut() else {
            return;
        };

        let selected_indices = Self::selected_indices(scene, &global_state.selected_positions);
        if selected_indices.is_empty() {
            return;
        }

        self.scale_start_positions.clear();
        for index in &selected_indices {
            let position = &scene.positions[*index];
            self.scale_start_positions
                .push((*index, Point::new(position.x, position.y)));
        }

        let center = Self::calculate_center(&self.scale_start_positions);
        let start_distance = Self::calculate_distance(center, floor_point);

        self.scale_center = Some(center);
        self.scale_start_distance = Some(start_distance);
        self.last_scale_floor_point = Some(floor_point);
        self.scale_active = true;
        self.selection_active = false;
        self.clear_selection_on_release = false;
        global_state.selection_rectangle = None;
        let _ = state_machine.try_apply(&ScalePositionsDragStartedTrigger);
    }

    fn update_scale(&mut self, global_state: &mut GlobalStateModel, floor_point: Point) {
        let Some(center) = self.scale_center else {
            return;
        };
        let Some(start_distance) = self.scale_start_distance else {
            return;
        };
        if start_distance <= 0.0 {
            return;
        }

        let distance = Self::calculate_distance(center, floor_point);
        let factor = distance / start_distance;

        if let Some(scene) = global_state.selected_scene.as_mut() {
            for (index, start_point) in &self.scale_start_positions {
                if let Some(position) = scene.positions.get_mut(*index) {
                    position.x = center.x + (start_point.x - center.x) * factor;
                    position.y = center.y + (start_point.y - center.y) * factor;
                }
            }
            Self::sync_selected_positions(scene, &mut global_state.selected_positions);
        }

        self.last_scale_floor_point = Some(floor_point);
    }

    fn complete_scale(
        &mut self,
        global_state: &mut GlobalStateModel,
        state_machine: &mut ApplicationStateMachine,
    ) {
        if let Some(last_point) = self.last_scale_floor_point {
            self.update_scale(global_state, last_point);
        }

        self.scale_active = false;
        self.scale_start_positions.clear();
        self.scale_center = None;
        self.scale_start_distance = None;
        self.last_scale_floor_point = None;
        let _ = state_machine.try_apply(&ScalePositionsDragCompletedTrigger);
    }

    fn start_selection(
        &mut self,
        global_state: &mut GlobalStateModel,
        state_machine: &mut ApplicationStateMachine,
        floor_point: Point,
    ) {
        self.selection_active = true;
        self.scale_active = false;
        self.clear_selection_on_release = false;
        global_state.selected_positions.clear();
        global_state.selection_rectangle = Some(SelectionRectangle {
            start: floor_point,
            end: floor_point,
        });
        let _ = state_machine.try_apply(&ScalePositionsSelectionStartedTrigger);
    }

    fn update_selection(&mut self, global_state: &mut GlobalStateModel, floor_point: Point) {
        let rectangle = global_state.selection_rectangle.unwrap_or(SelectionRectangle {
            start: floor_point,
            end: floor_point,
        });
        let updated = SelectionRectangle {
            start: rectangle.start,
            end: floor_point,
        };
        global_state.selection_rectangle = Some(updated);

        if let Some(scene) = global_state.selected_scene.as_ref() {
            let dancer_size = global_state.choreography.settings.dancer_size;
            let selected = Self::positions_in_rectangle(scene, dancer_size, updated);
            global_state.selected_positions = selected;
        }
    }

    fn complete_selection(
        &mut self,
        global_state: &mut GlobalStateModel,
        state_machine: &mut ApplicationStateMachine,
    ) {
        global_state.selection_rectangle = None;
        self.selection_active = false;
        let _ = state_machine.try_apply(&ScalePositionsSelectionCompletedTrigger);
    }

    fn clear_selection(
        &mut self,
        global_state: &mut GlobalStateModel,
        state_machine: &mut ApplicationStateMachine,
    ) {
        global_state.selected_positions.clear();
        global_state.selection_rectangle = None;
        self.selection_active = false;
        self.scale_active = false;
        self.scale_start_positions.clear();
        self.scale_center = None;
        self.scale_start_distance = None;
        self.last_scale_floor_point = None;
        self.reset_pointer_state();
        let _ = state_machine.try_apply(&ScalePositionsSelectionCompletedTrigger);
    }

    fn reset_pointer_state(&mut self) {
        self.pointer_pressed_position = None;
        self.pointer_moved = false;
        self.clear_selection_on_release = false;
    }

    fn is_scale_mode_active(
        global_state: &GlobalStateModel,
        state_machine: &ApplicationStateMachine,
    ) -> bool {
        if global_state.interaction_mode != InteractionMode::Scale {
            return false;
        }

        matches!(
            state_machine.state().kind(),
            StateKind::ScalePositionsState
                | StateKind::ScalePositionsSelectionStartState
                | StateKind::ScalePositionsSelectionEndState
                | StateKind::ScalePositionsDragStartState
                | StateKind::ScalePositionsDragEndState
        )
    }

    fn calculate_center(positions: &[(usize, Point)]) -> Point {
        let mut sum_x = 0.0;
        let mut sum_y = 0.0;
        for (_, point) in positions {
            sum_x += point.x;
            sum_y += point.y;
        }
        let count = positions.len() as f64;
        if count <= 0.0 {
            return Point::new(0.0, 0.0);
        }
        Point::new(sum_x / count, sum_y / count)
    }

    fn calculate_distance(center: Point, point: Point) -> f64 {
        let dx = point.x - center.x;
        let dy = point.y - center.y;
        (dx * dx + dy * dy).sqrt()
    }

    fn positions_in_rectangle(
        scene: &crate::scenes::SceneViewModel,
        dancer_size: f64,
        rectangle: SelectionRectangle,
    ) -> Vec<PositionModel> {
        let min_x = rectangle.start.x.min(rectangle.end.x);
        let max_x = rectangle.start.x.max(rectangle.end.x);
        let min_y = rectangle.start.y.min(rectangle.end.y);
        let max_y = rectangle.start.y.max(rectangle.end.y);

        let half_size = dancer_size / 2.0;
        let mut selected = Vec::new();
        for candidate in &scene.positions {
            let candidate_min_x = candidate.x - half_size;
            let candidate_max_x = candidate.x + half_size;
            let candidate_min_y = candidate.y - half_size;
            let candidate_max_y = candidate.y + half_size;

            let intersects = candidate_max_x >= min_x
                && candidate_min_x <= max_x
                && candidate_max_y >= min_y
                && candidate_min_y <= max_y;
            if intersects {
                selected.push(candidate.clone());
            }
        }

        selected
    }

    fn selected_indices(
        scene: &crate::scenes::SceneViewModel,
        selected_positions: &[PositionModel],
    ) -> Vec<usize> {
        let mut indices = Vec::new();
        for (index, position) in scene.positions.iter().enumerate() {
            if selected_positions.iter().any(|selected| selected == position) {
                indices.push(index);
            }
        }
        indices
    }

    fn sync_selected_positions(
        scene: &crate::scenes::SceneViewModel,
        selected_positions: &mut Vec<PositionModel>,
    ) {
        let indices = Self::selected_indices(scene, selected_positions);
        selected_positions.clear();
        for index in indices {
            if let Some(position) = scene.positions.get(index) {
                selected_positions.push(position.clone());
            }
        }
    }

    fn try_get_floor_point(
        view_model: &FloorCanvasViewModel,
        global_state: &GlobalStateModel,
        view_point: Point,
    ) -> Option<Point> {
        if !view_model.has_floor_bounds() {
            return None;
        }

        let floor_bounds = view_model.floor_bounds();
        let inverse = view_model.transformation_matrix.invert()?;
        let transformed = inverse.map_point(view_point);
        if !floor_bounds.contains(transformed) {
            return None;
        }

        let floor = &global_state.choreography.floor;
        let width = floor_bounds.width() as f64;
        let height = floor_bounds.height() as f64;
        let floor_width = (floor.size_left + floor.size_right) as f64;
        let floor_height = (floor.size_front + floor.size_back) as f64;

        if floor_width <= 0.0 || floor_height <= 0.0 || width <= 0.0 || height <= 0.0 {
            return None;
        }

        let scale = (width / floor_width).min(height / floor_height);
        if scale <= 0.0 || !scale.is_finite() {
            return None;
        }

        let origin_x = floor_bounds.left as f64 + floor.size_left as f64 * scale;
        let origin_y = floor_bounds.top as f64 + floor.size_front as f64 * scale;

        let position_x = (transformed.x - origin_x) / scale;
        let position_y = (origin_y - transformed.y) / scale;
        Some(Point::new(position_x, position_y))
    }
}

impl Behavior<FloorCanvasViewModel> for ScalePositionsBehavior {
    fn activate(
        &self,
        view_model: &mut FloorCanvasViewModel,
        disposables: &mut CompositeDisposable,
    ) {
        BehaviorLog::behavior_activated("ScalePositionsBehavior", "FloorCanvasViewModel");
        let Some(global_state) = self.global_state.clone() else {
            return;
        };
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
        let behavior = Rc::new(RefCell::new(self.clone()));
        let timer = slint::Timer::default();
        timer.start(TimerMode::Repeated, Duration::from_millis(16), move || {
            while let Ok(command) = pointer_pressed_receiver.try_recv() {
                let _ = global_state.try_update(|state| {
                    let mut behavior = behavior.borrow_mut();
                    let view_model_ref = view_model_handle.borrow();
                    let mut state_machine = state_machine.borrow_mut();
                    behavior.handle_pointer_pressed(
                        &view_model_ref,
                        state,
                        &mut state_machine,
                        command,
                    );
                });
            }

            while let Ok(command) = pointer_moved_receiver.try_recv() {
                let _ = global_state.try_update(|state| {
                    let mut behavior = behavior.borrow_mut();
                    let view_model_ref = view_model_handle.borrow();
                    let mut state_machine = state_machine.borrow_mut();
                    behavior.handle_pointer_moved(
                        &view_model_ref,
                        state,
                        &mut state_machine,
                        command,
                    );
                });
                view_model_handle.borrow_mut().draw_floor();
            }

            while let Ok(command) = pointer_released_receiver.try_recv() {
                let _ = global_state.try_update(|state| {
                    let mut behavior = behavior.borrow_mut();
                    let view_model_ref = view_model_handle.borrow();
                    let mut state_machine = state_machine.borrow_mut();
                    behavior.handle_pointer_released(
                        &view_model_ref,
                        state,
                        &mut state_machine,
                        command,
                    );
                });
                view_model_handle.borrow_mut().draw_floor();
            }
        });

        disposables.add(Box::new(TimerDisposable::new(timer)));
    }
}
