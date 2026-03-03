use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    #[must_use]
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Matrix {
    pub scale_x: f64,
    pub scale_y: f64,
    pub trans_x: f64,
    pub trans_y: f64,
}

impl Matrix {
    #[must_use]
    pub fn identity() -> Self {
        Self {
            scale_x: 1.0,
            scale_y: 1.0,
            trans_x: 0.0,
            trans_y: 0.0,
        }
    }

    pub fn translate(&mut self, delta_x: f64, delta_y: f64) {
        self.trans_x += delta_x;
        self.trans_y += delta_y;
    }

    #[must_use]
    pub fn translation(delta_x: f32, delta_y: f32) -> Self {
        Self {
            scale_x: 1.0,
            scale_y: 1.0,
            trans_x: f64::from(delta_x),
            trans_y: f64::from(delta_y),
        }
    }

    pub fn set_uniform_scale(&mut self, scale: f64) {
        self.scale_x = scale;
        self.scale_y = scale;
    }
}

impl Default for Matrix {
    fn default() -> Self {
        Self::identity()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InteractionMode {
    None,
    Move,
    RotateAroundCenter,
    RotateAroundDancer,
    Scale,
    Place,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PointerButton {
    Primary,
    Secondary,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct CanvasViewHandle {
    pub id: u64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PointerEventArgs {
    pub position: Point,
    pub button: PointerButton,
    pub is_in_contact: bool,
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
    Mouse,
    Touch,
    Pen,
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
pub struct AxisLabel {
    pub text: String,
    pub position: Point,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LegendEntry {
    pub label: String,
    pub color: [u8; 4],
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RectPrimitive {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

impl RectPrimitive {
    #[must_use]
    pub fn from_xywh(x: f64, y: f64, width: f64, height: f64) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LineSegment {
    pub from: Point,
    pub to: Point,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LabeledPoint {
    pub text: String,
    pub point: Point,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FloorLayer {
    Background,
    GridLines,
    FloorSvg,
    PathSegments,
    PositionCircles,
    PositionNumbers,
    SelectionSegments,
    HeaderOverlay,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FloorPosition {
    pub x: f64,
    pub y: f64,
}

impl FloorPosition {
    #[must_use]
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FloorLayoutMetrics {
    pub header_bottom: f64,
    pub floor_top: f64,
    pub floor_width: f64,
    pub floor_height: f64,
    pub legend_panel_width: f64,
    pub legend_panel_height: f64,
    pub side_label_left_gap: f64,
    pub side_label_right_gap: f64,
    pub top_label_vertical_gap: f64,
    pub bottom_label_vertical_gap: f64,
    pub legend_content_padding_left: f64,
    pub legend_content_padding_top: f64,
    pub legend_content_padding_right: f64,
    pub legend_content_padding_bottom: f64,
    pub legend_first_square_offset_x: f64,
    pub legend_first_square_offset_y: f64,
    pub legend_first_row_offset_x: f64,
    pub legend_first_row_offset_y: f64,
    pub legend_last_row_bottom_gap: f64,
}

impl FloorLayoutMetrics {
    #[must_use]
    pub fn from_zoom(zoom: f64) -> Self {
        Self {
            header_bottom: 60.0 * zoom,
            floor_top: 96.0 * zoom,
            floor_width: 720.0 * zoom,
            floor_height: 480.0 * zoom,
            legend_panel_width: 288.0 * zoom,
            legend_panel_height: 360.0 * zoom,
            side_label_left_gap: 24.0 * zoom,
            side_label_right_gap: 24.0 * zoom,
            top_label_vertical_gap: 24.0 * zoom,
            bottom_label_vertical_gap: 24.0 * zoom,
            legend_content_padding_left: 24.0 * zoom,
            legend_content_padding_top: 24.0 * zoom,
            legend_content_padding_right: 24.0 * zoom,
            legend_content_padding_bottom: 24.0 * zoom,
            legend_first_square_offset_x: 24.0 * zoom,
            legend_first_square_offset_y: 24.0 * zoom,
            legend_first_row_offset_x: 12.0 * zoom,
            legend_first_row_offset_y: 12.0 * zoom,
            legend_last_row_bottom_gap: 24.0 * zoom,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FloorState {
    pub transformation_matrix: Matrix,
    pub interaction_mode: InteractionMode,
    pub positions: Vec<FloorPosition>,
    pub selected_positions: Vec<usize>,
    pub selection_rectangle: Option<(Point, Point)>,
    pub is_place_mode: bool,
    pub snap_to_grid: bool,
    pub grid_resolution: i32,
    pub floor_left: i32,
    pub floor_right: i32,
    pub floor_front: i32,
    pub floor_back: i32,
    pub zoom: f64,
    pub metrics: FloorLayoutMetrics,
    pub draw_count: usize,
    pub render_marked: bool,
    pub render_mark_count: usize,
    pub pointer_anchor: Option<Point>,
    pub last_tap_point: Option<Point>,
    pub active_touches: BTreeMap<i64, Point>,
    pub pinch_distance: Option<f64>,
    pub pivot: Option<Point>,
    pub interpolated_positions: Vec<FloorPosition>,
    pub layout_width_px: f64,
    pub layout_height_px: f64,
    pub content_height_px: f64,
    pub floor_x: f64,
    pub floor_y: f64,
    pub floor_width: f64,
    pub floor_height: f64,
    pub header_height_px: f64,
    pub center_x: f64,
    pub center_y: f64,
    pub axis_labels: Vec<AxisLabel>,
    pub legend_entries: Vec<LegendEntry>,
    pub placement_remaining: Option<u32>,
    pub svg_path: Option<String>,
    pub svg_overlay_bounds: Option<RectPrimitive>,
    pub background_rect: Option<RectPrimitive>,
    pub header_overlay_rect: Option<RectPrimitive>,
    pub legend_panel_rect: Option<RectPrimitive>,
    pub grid_lines: Vec<LineSegment>,
    pub center_mark_segments: Vec<LineSegment>,
    pub path_segments: Vec<LineSegment>,
    pub dashed_path_segments: Vec<LineSegment>,
    pub path_commands: Vec<String>,
    pub dashed_path_commands: Vec<String>,
    pub selection_segments: Vec<LineSegment>,
    pub position_labels: Vec<LabeledPoint>,
    pub position_circles: Vec<Point>,
    pub layer_order: Vec<FloorLayer>,
    pub last_touch_device: Option<TouchDeviceType>,
    pub last_pointer_pressed: Option<PointerEventArgs>,
    pub last_pointer_moved: Option<PointerEventArgs>,
    pub last_pointer_released: Option<PointerEventArgs>,
    pub last_touch_event: Option<TouchEventArgs>,
    pub last_canvas_view: Option<CanvasViewHandle>,
    pub last_wheel_control_modifier: bool,
    pub last_wheel_position: Option<Point>,
}

impl Default for FloorState {
    fn default() -> Self {
        let zoom = 1.0;
        Self {
            transformation_matrix: Matrix::identity(),
            interaction_mode: InteractionMode::None,
            positions: Vec::new(),
            selected_positions: Vec::new(),
            selection_rectangle: None,
            is_place_mode: false,
            snap_to_grid: false,
            grid_resolution: 1,
            floor_left: 5,
            floor_right: 5,
            floor_front: 5,
            floor_back: 5,
            zoom,
            metrics: FloorLayoutMetrics::from_zoom(zoom),
            draw_count: 0,
            render_marked: false,
            render_mark_count: 0,
            pointer_anchor: None,
            last_tap_point: None,
            active_touches: BTreeMap::new(),
            pinch_distance: None,
            pivot: None,
            interpolated_positions: Vec::new(),
            layout_width_px: 960.0,
            layout_height_px: 720.0,
            content_height_px: 660.0,
            floor_x: 120.0,
            floor_y: 150.0,
            floor_width: 720.0,
            floor_height: 480.0,
            header_height_px: 60.0,
            center_x: 480.0,
            center_y: 390.0,
            axis_labels: Vec::new(),
            legend_entries: Vec::new(),
            placement_remaining: None,
            svg_path: None,
            svg_overlay_bounds: None,
            background_rect: None,
            header_overlay_rect: None,
            legend_panel_rect: None,
            grid_lines: Vec::new(),
            center_mark_segments: Vec::new(),
            path_segments: Vec::new(),
            dashed_path_segments: Vec::new(),
            path_commands: Vec::new(),
            dashed_path_commands: Vec::new(),
            selection_segments: Vec::new(),
            position_labels: Vec::new(),
            position_circles: Vec::new(),
            layer_order: vec![
                FloorLayer::Background,
                FloorLayer::GridLines,
                FloorLayer::FloorSvg,
                FloorLayer::PathSegments,
                FloorLayer::PositionCircles,
                FloorLayer::PositionNumbers,
                FloorLayer::SelectionSegments,
                FloorLayer::HeaderOverlay,
            ],
            last_touch_device: None,
            last_pointer_pressed: None,
            last_pointer_moved: None,
            last_pointer_released: None,
            last_touch_event: None,
            last_canvas_view: None,
            last_wheel_control_modifier: false,
            last_wheel_position: None,
        }
    }
}
