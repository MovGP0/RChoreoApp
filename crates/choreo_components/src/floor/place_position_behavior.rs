use std::cell::RefCell;
use std::rc::Rc;

use crate::behavior::{Behavior, CompositeDisposable};
use crate::behavior::SubscriptionDisposable;
use crate::global::GlobalStateModel;
use crate::logging::BehaviorLog;
use choreo_models::PositionModel;
use choreo_state_machine::{ApplicationStateMachine, StateKind};
use nject::injectable;
use rxrust::observable::SubscribeNext;

use super::floor_view_model::FloorCanvasViewModel;
use super::messages::{PointerButton, PointerMovedCommand, PointerPressedCommand, PointerReleasedCommand};
use super::types::Point;

#[derive(Default, Clone)]
#[injectable]
#[inject(
    |global_state: Rc<RefCell<GlobalStateModel>>,
     state_machine: Rc<RefCell<ApplicationStateMachine>>,
     view_model: Rc<RefCell<FloorCanvasViewModel>>| {
        Self::new(global_state, state_machine, view_model)
    }
)]
pub struct PlacePositionBehavior {
    global_state: Option<Rc<RefCell<GlobalStateModel>>>,
    state_machine: Option<Rc<RefCell<ApplicationStateMachine>>>,
    view_model: Option<Rc<RefCell<FloorCanvasViewModel>>>,
    pointer_pressed_position: Option<Point>,
    pointer_moved: bool,
}

impl PlacePositionBehavior {
    pub fn new(
        global_state: Rc<RefCell<GlobalStateModel>>,
        state_machine: Rc<RefCell<ApplicationStateMachine>>,
        view_model: Rc<RefCell<FloorCanvasViewModel>>,
    ) -> Self
    {
        Self {
            global_state: Some(global_state),
            state_machine: Some(state_machine),
            view_model: Some(view_model),
            ..Self::default()
        }
    }

    pub fn handle_pointer_pressed(&mut self, command: PointerPressedCommand) {
        if command.event_args.button != PointerButton::Primary {
            self.pointer_pressed_position = None;
            self.pointer_moved = false;
            return;
        }

        self.pointer_pressed_position = Some(command.event_args.position);
        self.pointer_moved = false;
    }

    pub fn handle_pointer_moved(&mut self, command: PointerMovedCommand) {
        let Some(pressed) = self.pointer_pressed_position else {
            return;
        };

        let delta_x = command.event_args.position.x - pressed.x;
        let delta_y = command.event_args.position.y - pressed.y;
        let distance = (delta_x * delta_x + delta_y * delta_y).sqrt();
        if distance > 0.0 {
            self.pointer_moved = true;
        }
    }

    pub fn handle_pointer_released(
        &mut self,
        view_model: &FloorCanvasViewModel,
        global_state: &mut GlobalStateModel,
        state_machine: &mut ApplicationStateMachine,
        command: PointerReleasedCommand,
    ) {
        if self.pointer_pressed_position.is_none() {
            return;
        }

        let should_place = !self.pointer_moved;
        self.pointer_pressed_position = None;
        self.pointer_moved = false;

        if should_place {
            self.try_place_position(view_model, global_state, state_machine, command.event_args.position);
        }
    }

    fn try_place_position(
        &self,
        view_model: &FloorCanvasViewModel,
        global_state: &mut GlobalStateModel,
        state_machine: &mut ApplicationStateMachine,
        view_point: Point,
    ) {
        if state_machine.state().kind() != StateKind::PlacePositionsState {
            return;
        }

        if !global_state.is_place_mode {
            return;
        }

        let Some(floor_point) = Self::try_get_floor_point(view_model, global_state, view_point) else {
            return;
        };

        let Some(scene_view_model) = global_state.selected_scene.as_mut() else {
            return;
        };

        if scene_view_model.positions.len() >= global_state.choreography.dancers.len() {
            return;
        }

        let mut position_x = floor_point.x;
        let mut position_y = floor_point.y;
        Self::snap_to_grid(&global_state.choreography, &mut position_x, &mut position_y);
        let new_position = PositionModel {
            dancer: None,
            orientation: None,
            x: position_x,
            y: position_y,
            curve1_x: None,
            curve1_y: None,
            curve2_x: None,
            curve2_y: None,
            movement1_x: None,
            movement1_y: None,
            movement2_x: None,
            movement2_y: None,
        };

        scene_view_model.positions.push(new_position.clone());
        if let Some(scene_model) = global_state
            .choreography
            .scenes
            .iter_mut()
            .find(|scene| scene.scene_id == scene_view_model.scene_id)
        {
            scene_model.positions.push(new_position);
        }
    }

    fn snap_to_grid(choreography: &choreo_models::ChoreographyModel, position_x: &mut f64, position_y: &mut f64) {
        if !choreography.settings.snap_to_grid {
            return;
        }

        let resolution = choreography.settings.resolution;
        if resolution <= 0 {
            return;
        }

        let step = 1.0 / resolution as f64;
        *position_x = (*position_x / step).round() * step;
        *position_y = (*position_y / step).round() * step;
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

        let center_x = floor_bounds.left as f64 + width / 2.0;
        let center_y = floor_bounds.top as f64 + height / 2.0;

        let position_x = (transformed.x - center_x) / scale;
        let position_y = (center_y - transformed.y) / scale;
        Some(Point::new(position_x, position_y))
    }
}

impl Behavior<FloorCanvasViewModel> for PlacePositionBehavior {
    fn activate(&self, _view_model: &mut FloorCanvasViewModel, disposables: &mut CompositeDisposable) {
        BehaviorLog::behavior_activated("PlacePositionBehavior", "FloorCanvasViewModel");
        let Some(global_state) = self.global_state.clone() else {
            return;
        };
        let Some(state_machine) = self.state_machine.clone() else {
            return;
        };
        let Some(view_model) = self.view_model.clone() else {
            return;
        };

        let behavior = Rc::new(RefCell::new(self.clone()));

        {
            let behavior = Rc::clone(&behavior);
            let subject = view_model.borrow().pointer_pressed_subject();
            let subscription = subject.subscribe(move |command| {
                let mut behavior = behavior.borrow_mut();
                behavior.handle_pointer_pressed(command);
            });
            disposables.add(Box::new(SubscriptionDisposable::new(subscription)));
        }

        {
            let behavior = Rc::clone(&behavior);
            let subject = view_model.borrow().pointer_moved_subject();
            let subscription = subject.subscribe(move |command| {
                let mut behavior = behavior.borrow_mut();
                behavior.handle_pointer_moved(command);
            });
            disposables.add(Box::new(SubscriptionDisposable::new(subscription)));
        }

        {
            let behavior = Rc::clone(&behavior);
            let view_model = Rc::clone(&view_model);
            let state_machine = Rc::clone(&state_machine);
            let global_state = Rc::clone(&global_state);
            let subject = view_model.borrow().pointer_released_subject();
            let subscription = subject.subscribe(move |command| {
                let mut behavior = behavior.borrow_mut();
                let view_model_ref = view_model.borrow();
                let mut global_state = global_state.borrow_mut();
                let mut state_machine = state_machine.borrow_mut();
                behavior.handle_pointer_released(
                    &view_model_ref,
                    &mut global_state,
                    &mut state_machine,
                    command,
                );
                drop(view_model_ref);
                view_model.borrow_mut().draw_floor();
            });
            disposables.add(Box::new(SubscriptionDisposable::new(subscription)));
        }
    }
}

