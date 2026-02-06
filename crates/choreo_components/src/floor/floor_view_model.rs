use crossbeam_channel::Sender;
use nject::injectable;
use std::cell::RefCell;
use std::rc::{Rc, Weak};

use crate::behavior::{Behavior, CompositeDisposable};

use super::messages::{
    DrawFloorCommand,
    PointerMovedCommand,
    PointerPressedCommand,
    PointerReleasedCommand,
    PointerWheelChangedCommand,
    TouchCommand,
};

use super::types::{CanvasViewHandle, Matrix, Rect, Size};

#[injectable]
#[inject(
    |draw_floor_command_sender: Sender<DrawFloorCommand>, event_senders: FloorPointerEventSenders| {
        Self::new(draw_floor_command_sender, event_senders)
    }
)]
pub struct FloorCanvasViewModel {
    draw_floor_command_sender: Sender<DrawFloorCommand>,
    disposables: CompositeDisposable,
    pointer_pressed_senders: Vec<Sender<PointerPressedCommand>>,
    pointer_moved_senders: Vec<Sender<PointerMovedCommand>>,
    pointer_released_senders: Vec<Sender<PointerReleasedCommand>>,
    pointer_wheel_changed_senders: Vec<Sender<PointerWheelChangedCommand>>,
    touch_senders: Vec<Sender<TouchCommand>>,
    on_redraw: Option<Rc<dyn Fn()>>,
    self_handle: Option<Weak<RefCell<FloorCanvasViewModel>>>,
    pub canvas_view: Option<CanvasViewHandle>,
    pub transformation_matrix: Matrix,
    has_floor_bounds: bool,
    floor_bounds: Rect,
    canvas_size: Size,
}

impl FloorCanvasViewModel {
    pub const MAX_ZOOM_FACTOR: f32 = 5.0;
    pub const MIN_ZOOM_FACTOR: f32 = 0.2;
    pub const PAN_MARGIN: f32 = 20.0;

    pub fn new(
        draw_floor_command_sender: Sender<DrawFloorCommand>,
        event_senders: FloorPointerEventSenders,
    ) -> Self {
        Self {
            draw_floor_command_sender,
            disposables: CompositeDisposable::new(),
            pointer_pressed_senders: event_senders.pointer_pressed_senders,
            pointer_moved_senders: event_senders.pointer_moved_senders,
            pointer_released_senders: event_senders.pointer_released_senders,
            pointer_wheel_changed_senders: event_senders.pointer_wheel_changed_senders,
            touch_senders: event_senders.touch_senders,
            on_redraw: None,
            self_handle: None,
            canvas_view: None,
            transformation_matrix: Matrix::identity(),
            has_floor_bounds: false,
            floor_bounds: Rect::default(),
            canvas_size: Size::default(),
        }
    }

    pub fn activate(
        view_model: &Rc<RefCell<FloorCanvasViewModel>>,
        behaviors: Vec<Box<dyn Behavior<FloorCanvasViewModel>>>,
    ) {
        let mut disposables = CompositeDisposable::new();
        {
            let mut view_model = view_model.borrow_mut();
            for behavior in behaviors.iter() {
                behavior.activate(&mut view_model, &mut disposables);
            }
        }

        view_model.borrow_mut().disposables = disposables;
    }

    pub fn draw_floor(&mut self) {
        let _ = self.draw_floor_command_sender.send(DrawFloorCommand);
        self.request_redraw();
    }

    pub fn request_redraw(&self) {
        if let Some(handler) = self.on_redraw.as_ref() {
            handler();
        }
    }

    pub fn draw_floor_command_sender(&self) -> &Sender<DrawFloorCommand> {
        &self.draw_floor_command_sender
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

    pub fn pointer_pressed(&self, command: PointerPressedCommand) -> PointerPressedCommand {
        Self::try_send_all(&self.pointer_pressed_senders, &command);
        command
    }

    pub fn pointer_moved(&self, command: PointerMovedCommand) -> PointerMovedCommand {
        Self::try_send_all(&self.pointer_moved_senders, &command);
        command
    }

    pub fn pointer_released(&self, command: PointerReleasedCommand) -> PointerReleasedCommand {
        Self::try_send_all(&self.pointer_released_senders, &command);
        command
    }

    pub fn pointer_wheel_changed(
        &self,
        command: PointerWheelChangedCommand,
    ) -> PointerWheelChangedCommand {
        Self::try_send_all(&self.pointer_wheel_changed_senders, &command);
        command
    }

    pub fn touch(&self, command: TouchCommand) -> TouchCommand {
        Self::try_send_all(&self.touch_senders, &command);
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

    fn try_send_all<T: Clone>(senders: &[Sender<T>], command: &T) {
        for sender in senders {
            let _ = sender.try_send(command.clone());
        }
    }
}

impl Drop for FloorCanvasViewModel {
    fn drop(&mut self) {
        self.disposables.dispose_all();
    }
}

pub struct FloorPointerEventSenders {
    pub pointer_pressed_senders: Vec<Sender<PointerPressedCommand>>,
    pub pointer_moved_senders: Vec<Sender<PointerMovedCommand>>,
    pub pointer_released_senders: Vec<Sender<PointerReleasedCommand>>,
    pub pointer_wheel_changed_senders: Vec<Sender<PointerWheelChangedCommand>>,
    pub touch_senders: Vec<Sender<TouchCommand>>,
}
