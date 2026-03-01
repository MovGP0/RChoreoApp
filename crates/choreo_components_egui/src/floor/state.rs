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
        }
    }
}
