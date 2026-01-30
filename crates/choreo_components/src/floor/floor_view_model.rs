use crossbeam_channel::{Receiver, Sender};
use nject::injectable;
use rxrust::prelude::{LocalSubject, Observer};
use std::cell::RefCell;
use std::rc::{Rc, Weak};

use crate::behavior::{Behavior, CompositeDisposable};
use crate::global::GlobalStateModel;
use crate::preferences::Preferences;
use choreo_state_machine::ApplicationStateMachine;

use super::draw_floor_behavior::DrawFloorBehavior;
use super::gesture_handling_behavior::GestureHandlingBehavior;
use super::move_positions_behavior::MovePositionsBehavior;
use super::place_position_behavior::PlacePositionBehavior;
use super::redraw_floor_behavior::RedrawFloorBehavior;
use super::rotate_around_center_behavior::RotateAroundCenterBehavior;
use super::scale_around_dancer_behavior::ScaleAroundDancerBehavior;
use super::scale_positions_behavior::ScalePositionsBehavior;

use super::messages::{
    DrawFloorCommand,
    PanUpdatedCommand,
    PinchUpdatedCommand,
    PointerMovedCommand,
    PointerPressedCommand,
    PointerReleasedCommand,
    PointerWheelChangedCommand,
    TouchCommand,
};

use super::types::{
    CanvasViewHandle,
    FloorRenderGate,
    Matrix,
    Rect,
    Size,
};

#[injectable]
#[inject(
    |draw_floor_command_sender: Sender<DrawFloorCommand>,
     behaviors: Vec<Box<dyn Behavior<FloorCanvasViewModel>>>| {
        Self::new(draw_floor_command_sender, behaviors)
    }
)]
pub struct FloorCanvasViewModel {
    draw_floor_command_sender: Sender<DrawFloorCommand>,
    disposables: CompositeDisposable,
    draw_floor_subject: LocalSubject<'static, DrawFloorCommand, ()>,
    pan_updated_subject: LocalSubject<'static, PanUpdatedCommand, ()>,
    pinch_updated_subject: LocalSubject<'static, PinchUpdatedCommand, ()>,
    pointer_pressed_subject: LocalSubject<'static, PointerPressedCommand, ()>,
    pointer_moved_subject: LocalSubject<'static, PointerMovedCommand, ()>,
    pointer_released_subject: LocalSubject<'static, PointerReleasedCommand, ()>,
    pointer_wheel_changed_subject: LocalSubject<'static, PointerWheelChangedCommand, ()>,
    touch_subject: LocalSubject<'static, TouchCommand, ()>,
    on_redraw: Option<Rc<dyn Fn()>>,
    self_handle: Option<Weak<RefCell<FloorCanvasViewModel>>>,
    pub canvas_view: Option<CanvasViewHandle>,
    pub transformation_matrix: Matrix,
    has_floor_bounds: bool,
    floor_bounds: Rect,
    canvas_size: Size,
}

pub struct FloorDependencies {
    pub global_state: Rc<RefCell<GlobalStateModel>>,
    pub state_machine: Rc<RefCell<ApplicationStateMachine>>,
    pub preferences: Rc<dyn Preferences>,
    pub draw_floor_sender: Sender<DrawFloorCommand>,
    pub draw_floor_receiver: Receiver<DrawFloorCommand>,
    pub redraw_receiver: Receiver<crate::choreography_settings::RedrawFloorCommand>,
    pub render_gate: Option<Box<dyn FloorRenderGate>>,
}

pub fn build_floor_canvas_view_model(deps: FloorDependencies) -> Rc<RefCell<FloorCanvasViewModel>> {
    let view_model = Rc::new(RefCell::new(FloorCanvasViewModel::new(
        deps.draw_floor_sender,
    )));
    view_model
        .borrow_mut()
        .set_self_handle(Rc::downgrade(&view_model));

    let behaviors: Vec<Box<dyn Behavior<FloorCanvasViewModel>>> = vec![
        Box::new(DrawFloorBehavior::new(
            deps.draw_floor_receiver,
            deps.render_gate.map(Rc::from),
        )),
        Box::new(RedrawFloorBehavior::new(
            deps.redraw_receiver,
        )),
        Box::new(GestureHandlingBehavior::new(
            Rc::clone(&deps.state_machine),
        )),
        Box::new(PlacePositionBehavior::new(
            Rc::clone(&deps.global_state),
            Rc::clone(&deps.state_machine),
        )),
        Box::new(MovePositionsBehavior::new(
            Rc::clone(&deps.global_state),
            Rc::clone(&deps.state_machine),
        )),
        Box::new(RotateAroundCenterBehavior::new(
            Rc::clone(&deps.global_state),
            Rc::clone(&deps.state_machine),
        )),
        Box::new(ScalePositionsBehavior::new(
            Rc::clone(&deps.global_state),
            Rc::clone(&deps.state_machine),
        )),
        Box::new(ScaleAroundDancerBehavior::new(
            Rc::clone(&deps.global_state),
            Rc::clone(&deps.state_machine),
        )),
    ];

    FloorCanvasViewModel::bind_behaviors(&view_model, behaviors);
    view_model
}

impl FloorCanvasViewModel {
    pub const MAX_ZOOM_FACTOR: f32 = 5.0;
    pub const MIN_ZOOM_FACTOR: f32 = 0.2;
    pub const PAN_MARGIN: f32 = 20.0;

    pub fn new(
        draw_floor_command_sender: Sender<DrawFloorCommand>,
    ) -> Self {
        Self {
            draw_floor_command_sender,
            disposables: CompositeDisposable::new(),
            draw_floor_subject: LocalSubject::new(),
            pan_updated_subject: LocalSubject::new(),
            pinch_updated_subject: LocalSubject::new(),
            pointer_pressed_subject: LocalSubject::new(),
            pointer_moved_subject: LocalSubject::new(),
            pointer_released_subject: LocalSubject::new(),
            pointer_wheel_changed_subject: LocalSubject::new(),
            touch_subject: LocalSubject::new(),
            on_redraw: None,
            self_handle: None,
            canvas_view: None,
            transformation_matrix: Matrix::identity(),
            has_floor_bounds: false,
            floor_bounds: Rect::default(),
            canvas_size: Size::default(),
        }
    }

    pub fn dispose(&mut self) {
        self.disposables.dispose_all();
    }

    pub fn bind_behaviors(
        view_model: &Rc<RefCell<FloorCanvasViewModel>>,
        mut behaviors: Vec<Box<dyn Behavior<FloorCanvasViewModel>>>,
    ) {
        let mut disposables = CompositeDisposable::new();
        {
            let mut view_model = view_model.borrow_mut();
            for behavior in behaviors.iter() {
                behavior.initialize(&mut view_model, &mut disposables);
            }
        }

        for behavior in behaviors.drain(..) {
            behavior.bind(view_model, &mut disposables);
        }

        view_model.borrow_mut().disposables = disposables;
    }

    pub fn draw_floor(&mut self) {
        self.draw_floor_subject.next(DrawFloorCommand);
        let _ = self.draw_floor_command_sender.send(DrawFloorCommand);
        if let Some(handler) = self.on_redraw.as_ref() {
            handler();
        }
    }

    pub fn draw_floor_command_sender(&self) -> &Sender<DrawFloorCommand> {
        &self.draw_floor_command_sender
    }

    pub fn draw_floor_subject(&self) -> LocalSubject<'static, DrawFloorCommand, ()> {
        self.draw_floor_subject.clone()
    }

    pub fn pan_updated_subject(&self) -> LocalSubject<'static, PanUpdatedCommand, ()> {
        self.pan_updated_subject.clone()
    }

    pub fn pinch_updated_subject(&self) -> LocalSubject<'static, PinchUpdatedCommand, ()> {
        self.pinch_updated_subject.clone()
    }

    pub fn pointer_pressed_subject(&self) -> LocalSubject<'static, PointerPressedCommand, ()> {
        self.pointer_pressed_subject.clone()
    }

    pub fn pointer_moved_subject(&self) -> LocalSubject<'static, PointerMovedCommand, ()> {
        self.pointer_moved_subject.clone()
    }

    pub fn pointer_released_subject(&self) -> LocalSubject<'static, PointerReleasedCommand, ()> {
        self.pointer_released_subject.clone()
    }

    pub fn pointer_wheel_changed_subject(
        &self,
    ) -> LocalSubject<'static, PointerWheelChangedCommand, ()> {
        self.pointer_wheel_changed_subject.clone()
    }

    pub fn touch_subject(&self) -> LocalSubject<'static, TouchCommand, ()> {
        self.touch_subject.clone()
    }

    pub fn set_on_redraw(&mut self, handler: Option<Rc<dyn Fn()>>) {
        self.on_redraw = handler;
    }

    pub fn set_self_handle(&mut self, handle: Weak<RefCell<FloorCanvasViewModel>>) {
        self.self_handle = Some(handle);
    }

    pub fn self_handle(&self) -> Option<Weak<RefCell<FloorCanvasViewModel>>> {
        self.self_handle.clone()
    }

    pub fn pan_updated(&mut self, command: PanUpdatedCommand) -> PanUpdatedCommand {
        self.pan_updated_subject.next(command.clone());
        command
    }

    pub fn pinch_updated(&mut self, command: PinchUpdatedCommand) -> PinchUpdatedCommand {
        self.pinch_updated_subject.next(command.clone());
        command
    }

    pub fn pointer_pressed(&mut self, command: PointerPressedCommand) -> PointerPressedCommand {
        self.pointer_pressed_subject.next(command.clone());
        command
    }

    pub fn pointer_moved(&mut self, command: PointerMovedCommand) -> PointerMovedCommand {
        self.pointer_moved_subject.next(command.clone());
        command
    }

    pub fn pointer_released(&mut self, command: PointerReleasedCommand) -> PointerReleasedCommand {
        self.pointer_released_subject.next(command.clone());
        command
    }

    pub fn pointer_wheel_changed(
        &mut self,
        command: PointerWheelChangedCommand,
    ) -> PointerWheelChangedCommand {
        self.pointer_wheel_changed_subject.next(command.clone());
        command
    }

    pub fn touch(&mut self, command: TouchCommand) -> TouchCommand {
        self.touch_subject.next(command.clone());
        command
    }

    pub fn set_floor_bounds(&mut self, bounds: Rect) {
        self.floor_bounds = bounds;
        self.has_floor_bounds = true;
    }

    pub fn set_canvas_size(&mut self, size: Size) {
        self.canvas_size = size;
    }

    pub fn set_transformation_matrix(&mut self, matrix: Matrix) {
        self.transformation_matrix = matrix;
    }

    pub fn floor_bounds(&self) -> Rect {
        self.floor_bounds
    }

    pub fn canvas_size(&self) -> Size {
        self.canvas_size
    }

    pub fn has_floor_bounds(&self) -> bool {
        self.has_floor_bounds
    }
}


