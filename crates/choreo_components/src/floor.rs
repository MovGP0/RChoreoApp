use crossbeam_channel::Sender;

use crate::behavior::{Behavior, CompositeDisposable};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rect {
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
}

impl Rect {
    pub fn new(left: f32, top: f32, right: f32, bottom: f32) -> Self {
        Self {
            left,
            top,
            right,
            bottom,
        }
    }
}

impl Default for Rect {
    fn default() -> Self {
        Self::new(0.0, 0.0, 0.0, 0.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Size {
    pub width: f32,
    pub height: f32,
}

impl Size {
    pub fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }
}

impl Default for Size {
    fn default() -> Self {
        Self::new(0.0, 0.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Matrix {
    pub values: [f32; 9],
}

impl Matrix {
    pub fn identity() -> Self {
        Self {
            values: [1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0],
        }
    }
}

impl Default for Matrix {
    fn default() -> Self {
        Self::identity()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DrawFloorCommand;

#[derive(Debug, Clone, PartialEq)]
pub struct PanUpdatedCommand;

#[derive(Debug, Clone, PartialEq)]
pub struct PinchUpdatedCommand;

#[derive(Debug, Clone, PartialEq)]
pub struct PointerPressedCommand;

#[derive(Debug, Clone, PartialEq)]
pub struct PointerMovedCommand;

#[derive(Debug, Clone, PartialEq)]
pub struct PointerReleasedCommand;

#[derive(Debug, Clone, PartialEq)]
pub struct PointerWheelChangedCommand;

#[derive(Debug, Clone, PartialEq)]
pub struct TouchCommand;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CanvasViewHandle;

pub struct FloorCanvasViewModel {
    draw_floor_command_sender: Sender<DrawFloorCommand>,
    disposables: CompositeDisposable,
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
        behaviors: Vec<Box<dyn Behavior<FloorCanvasViewModel>>>,
    ) -> Self {
        let mut view_model = Self {
            draw_floor_command_sender,
            disposables: CompositeDisposable::new(),
            canvas_view: None,
            transformation_matrix: Matrix::identity(),
            has_floor_bounds: false,
            floor_bounds: Rect::default(),
            canvas_size: Size::default(),
        };

        for behavior in behaviors {
            behavior.activate(&mut view_model, &mut view_model.disposables);
        }

        view_model
    }

    pub fn has_floor_bounds(&self) -> bool {
        self.has_floor_bounds
    }

    pub fn floor_bounds(&self) -> Rect {
        self.floor_bounds
    }

    pub fn canvas_size(&self) -> Size {
        self.canvas_size
    }

    pub fn update_floor_bounds(&mut self, floor_bounds: Rect, canvas_size: Size) {
        self.floor_bounds = floor_bounds;
        self.canvas_size = canvas_size;
        self.has_floor_bounds = true;
    }

    pub fn draw_floor_command_sender(&self) -> &Sender<DrawFloorCommand> {
        &self.draw_floor_command_sender
    }

    pub fn pan_updated(&self, command: PanUpdatedCommand) -> PanUpdatedCommand {
        command
    }

    pub fn pinch_updated(&self, command: PinchUpdatedCommand) -> PinchUpdatedCommand {
        command
    }

    pub fn pointer_pressed(&self, command: PointerPressedCommand) -> PointerPressedCommand {
        command
    }

    pub fn pointer_moved(&self, command: PointerMovedCommand) -> PointerMovedCommand {
        command
    }

    pub fn pointer_released(&self, command: PointerReleasedCommand) -> PointerReleasedCommand {
        command
    }

    pub fn pointer_wheel_changed(
        &self,
        command: PointerWheelChangedCommand,
    ) -> PointerWheelChangedCommand {
        command
    }

    pub fn touch(&self, command: TouchCommand) -> TouchCommand {
        command
    }
}
