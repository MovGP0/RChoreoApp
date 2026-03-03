use super::state::FloorPosition;
use super::state::InteractionMode;
use super::state::Point;
use super::state::TouchDeviceType;
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
        cursor: Option<Point>,
    },
    Touch {
        id: i64,
        action: TouchAction,
        point: Point,
        is_in_contact: bool,
        device: TouchDeviceType,
    },
    SetLayout {
        width_px: f64,
        height_px: f64,
    },
    SetAxisLabels {
        x_axis: String,
        y_axis: String,
    },
    SetLegendEntries {
        entries: Vec<(String, [u8; 4])>,
    },
    SetPlacementRemaining {
        count: Option<u32>,
    },
    SetSvgOverlay {
        svg_path: Option<String>,
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
