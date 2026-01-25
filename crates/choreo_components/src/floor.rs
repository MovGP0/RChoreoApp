use crossbeam_channel::{Receiver, Sender};

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

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RgbaColor {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
    pub alpha: f32,
}

impl RgbaColor {
    pub fn to_rgba8(self) -> [u8; 4] {
        [
            float_to_u8(self.red),
            float_to_u8(self.green),
            float_to_u8(self.blue),
            float_to_u8(self.alpha),
        ]
    }
}

fn float_to_u8(value: f32) -> u8 {
    let clamped = value.clamp(0.0, 1.0);
    (clamped * 255.0).round() as u8
}

pub trait FloorRenderGate: Send + Sync {
    fn is_rendered(&self) -> bool;
    fn mark_rendered(&self);
    fn wait_for_first_render(&self);
}

#[derive(Debug, Default)]
pub struct FloorRenderGateImpl {
    state: std::sync::Arc<(std::sync::Mutex<bool>, std::sync::Condvar)>,
}

impl FloorRenderGateImpl {
    pub fn new() -> Self {
        Self::default()
    }
}

impl FloorRenderGate for FloorRenderGateImpl {
    fn is_rendered(&self) -> bool {
        let (lock, _) = &*self.state;
        *lock.lock().unwrap_or_else(|guard| guard.into_inner())
    }

    fn mark_rendered(&self) {
        let (lock, condvar) = &*self.state;
        let mut rendered = lock.lock().unwrap_or_else(|guard| guard.into_inner());
        if *rendered {
            return;
        }
        *rendered = true;
        condvar.notify_all();
    }

    fn wait_for_first_render(&self) {
        let (lock, condvar) = &*self.state;
        let mut rendered = lock.lock().unwrap_or_else(|guard| guard.into_inner());
        while !*rendered {
            rendered = condvar
                .wait(rendered)
                .unwrap_or_else(|guard| guard.into_inner());
        }
    }
}

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

        let mut disposables = CompositeDisposable::new();
        for behavior in behaviors {
            behavior.activate(&mut view_model, &mut disposables);
        }
        view_model.disposables = disposables;

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

pub struct DrawFloorBehavior {
    receiver: crossbeam_channel::Receiver<DrawFloorCommand>,
    render_gate: Option<Box<dyn FloorRenderGate>>,
    has_rendered: bool,
}

impl DrawFloorBehavior {
    pub fn new(
        receiver: crossbeam_channel::Receiver<DrawFloorCommand>,
        render_gate: Option<Box<dyn FloorRenderGate>>,
    ) -> Self {
        Self {
            receiver,
            render_gate,
            has_rendered: false,
        }
    }

    pub fn try_handle(&mut self) -> bool {
        match self.receiver.try_recv() {
            Ok(_) => {
                if !self.has_rendered {
                    self.has_rendered = true;
                    if let Some(render_gate) = &self.render_gate {
                        render_gate.mark_rendered();
                    }
                }
                true
            }
            Err(_) => false,
        }
    }
}

impl Behavior<FloorCanvasViewModel> for DrawFloorBehavior {
    fn activate(&self, _view_model: &mut FloorCanvasViewModel, _disposables: &mut CompositeDisposable) {}
}

pub struct GestureHandlingBehavior;

impl GestureHandlingBehavior {
    pub const TOUCH_PAN_FACTOR: f32 = 0.5;
}

impl Behavior<FloorCanvasViewModel> for GestureHandlingBehavior {
    fn activate(&self, _view_model: &mut FloorCanvasViewModel, _disposables: &mut CompositeDisposable) {}
}

pub struct MovePositionsBehavior;

impl Behavior<FloorCanvasViewModel> for MovePositionsBehavior {
    fn activate(&self, _view_model: &mut FloorCanvasViewModel, _disposables: &mut CompositeDisposable) {}
}

pub struct PlacePositionBehavior;

impl Behavior<FloorCanvasViewModel> for PlacePositionBehavior {
    fn activate(&self, _view_model: &mut FloorCanvasViewModel, _disposables: &mut CompositeDisposable) {}
}

pub struct RedrawFloorBehavior;

impl Behavior<FloorCanvasViewModel> for RedrawFloorBehavior {
    fn activate(&self, _view_model: &mut FloorCanvasViewModel, _disposables: &mut CompositeDisposable) {}
}

pub struct RotateAroundCenterBehavior;

impl Behavior<FloorCanvasViewModel> for RotateAroundCenterBehavior {
    fn activate(&self, _view_model: &mut FloorCanvasViewModel, _disposables: &mut CompositeDisposable) {}
}

pub struct ScaleAroundDancerBehavior;

impl Behavior<FloorCanvasViewModel> for ScaleAroundDancerBehavior {
    fn activate(&self, _view_model: &mut FloorCanvasViewModel, _disposables: &mut CompositeDisposable) {}
}

pub struct ScalePositionsBehavior;

impl Behavior<FloorCanvasViewModel> for ScalePositionsBehavior {
    fn activate(&self, _view_model: &mut FloorCanvasViewModel, _disposables: &mut CompositeDisposable) {}
}
