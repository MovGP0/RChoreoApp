pub use super::state::PointerButton;
pub use super::state::PointerEventArgs;
pub use super::state::TouchAction;
pub use super::state::TouchDeviceType;
pub use super::state::TouchEventArgs;

use super::types::CanvasViewHandle;
use super::types::Point;

#[derive(Debug, Clone, PartialEq)]
pub struct DrawFloorCommand;

#[derive(Debug, Clone, PartialEq)]
pub struct PanUpdatedCommand;

#[derive(Debug, Clone, PartialEq)]
pub struct PinchUpdatedCommand;

#[derive(Debug, Clone, PartialEq)]
pub struct PointerPressedCommand {
    pub canvas_view: CanvasViewHandle,
    pub event_args: PointerEventArgs,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PointerMovedCommand {
    pub canvas_view: CanvasViewHandle,
    pub event_args: PointerEventArgs,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PointerReleasedCommand {
    pub canvas_view: CanvasViewHandle,
    pub event_args: PointerEventArgs,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PointerWheelChangedCommand {
    pub canvas_view: CanvasViewHandle,
    pub delta_x: f64,
    pub delta_y: f64,
    pub control_modifier: bool,
    pub position: Option<Point>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TouchCommand {
    pub canvas_view: CanvasViewHandle,
    pub event_args: TouchEventArgs,
}
