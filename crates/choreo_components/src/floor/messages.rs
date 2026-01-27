use super::types::{CanvasViewHandle, Point};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PointerButton {
    Primary,
    Secondary,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PointerEventArgs {
    pub position: Point,
    pub button: PointerButton,
    pub is_in_contact: bool,
}

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
    pub delta: f64,
    pub position: Option<Point>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TouchAction {
    Pressed,
    Moved,
    Released,
    Cancelled,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TouchDeviceType {
    Touch,
    Mouse,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TouchEventArgs {
    pub id: i64,
    pub action: TouchAction,
    pub device_type: TouchDeviceType,
    pub location: Point,
    pub in_contact: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TouchCommand {
    pub canvas_view: CanvasViewHandle,
    pub event_args: TouchEventArgs,
}
