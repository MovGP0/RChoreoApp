use super::state::FloorPosition;
use super::state::InteractionMode;
use super::state::Point;
use super::state::TouchAction;

#[derive(Debug, Clone, PartialEq)]
pub enum FloorAction {
    Initialize,
    DrawFloor,
    RedrawFloor,
    SetInteractionMode {
        mode: InteractionMode,
    },
    SetPositions {
        positions: Vec<FloorPosition>,
    },
    SelectRectangle {
        start: Point,
        end: Point,
    },
    MoveSelectedByDelta {
        delta_x: f64,
        delta_y: f64,
    },
    RotateSelectedAroundCenter {
        start: Point,
        end: Point,
    },
    SetPivotFromPoint {
        point: Point,
    },
    RotateSelectedAroundPivot {
        start: Point,
        end: Point,
    },
    ScaleSelected {
        start: Point,
        end: Point,
    },
    PlacePosition {
        point: Point,
    },
    ClearSelection,
    PointerPressed {
        point: Point,
    },
    PointerMoved {
        point: Point,
    },
    PointerReleased {
        point: Point,
    },
    PointerWheelChanged {
        delta_x: f64,
        delta_y: f64,
        ctrl: bool,
    },
    Touch {
        id: i64,
        action: TouchAction,
        point: Point,
        is_in_contact: bool,
    },
    ResetViewport,
    SetZoom {
        zoom: f64,
    },
    SetSnapToGrid {
        enabled: bool,
        resolution: i32,
    },
    InterpolateAudioPosition {
        from: Vec<FloorPosition>,
        to: Vec<FloorPosition>,
        progress: f64,
    },
}
