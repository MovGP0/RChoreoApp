use crossbeam_channel::{
    Receiver,
    Sender
};

use crate::behavior::{
    Behavior,
    CompositeDisposable
};

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

pub struct FloorCanvasViewModel {
    draw_floor_command_sender: Sender<DrawFloorCommand>,
    disposables: CompositeDisposable,
    pub canvas_view: Option<CanvasViewHandle>,
    pub transformation_matrix: Matrix,
    has_floor_bounds: bool,
    floor_bounds: Rect,
    canvas_size: Size,
}

pub struct FloorDependencies {
    pub draw_floor_sender: Sender<DrawFloorCommand>,
    pub draw_floor_receiver: Receiver<DrawFloorCommand>,
    pub render_gate: Option<Box<dyn FloorRenderGate>>,
}

pub fn build_floor_canvas_view_model(deps: FloorDependencies) -> FloorCanvasViewModel {
    let behaviors: Vec<Box<dyn Behavior<FloorCanvasViewModel>>> = vec![
        Box::new(DrawFloorBehavior::new(
            deps.draw_floor_receiver,
            deps.render_gate,
        )),
        Box::new(RedrawFloorBehavior),
        Box::new(GestureHandlingBehavior),
        Box::new(PlacePositionBehavior),
        Box::new(MovePositionsBehavior),
        Box::new(RotateAroundCenterBehavior),
        Box::new(ScalePositionsBehavior),
        Box::new(ScaleAroundDancerBehavior),
    ];

    FloorCanvasViewModel::new(deps.draw_floor_sender, behaviors)
}

impl FloorCanvasViewModel {
    pub const MAX_ZOOM_FACTOR: f32 = 5.0;
    pub const MIN_ZOOM_FACTOR: f32 = 0.2;
    pub const PAN_MARGIN: f32 = 20.0;

    pub fn new(
        draw_floor_command_sender: Sender<DrawFloorCommand>,
        mut behaviors: Vec<Box<dyn Behavior<FloorCanvasViewModel>>>,
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

        let mut disposables = CompositeDisposable::new();
        for behavior in behaviors.drain(..) {
            behavior.activate(&mut view_model, &mut disposables);
        }
        view_model.disposables = disposables;
        view_model
    }

    pub fn dispose(&mut self) {
        self.disposables.dispose_all();
    }

    pub fn draw_floor(&self) {
        let _ = self.draw_floor_command_sender.send(DrawFloorCommand);
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


