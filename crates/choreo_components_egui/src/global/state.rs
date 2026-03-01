pub type CanvasPoint = (f64, f64);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SelectionRectangle {
    pub start: CanvasPoint,
    pub end: CanvasPoint,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum InteractionMode {
    #[default]
    View,
    Move,
    RotateAroundCenter,
    RotateAroundDancer,
    Scale,
    LineOfSight,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GlobalState {
    pub selected_scene_id: Option<String>,
    pub selected_position_ids: Vec<String>,
    pub selection_rectangle: Option<SelectionRectangle>,
    pub is_place_mode: bool,
    pub interaction_mode: InteractionMode,
    pub scene_list_scroll_offset: f32,
    pub should_scroll_to_selected_scene: bool,
    pub has_closeable_scene_selection: bool,
    pub is_scene_list_search_open: bool,
    pub has_pending_drag_selection: bool,
    pub drag_selected_scene_id: Option<String>,
    pub is_rendering_floor: bool,
    pub redraw_floor: bool,
    pub svg_file_path: Option<String>,
}

impl Default for GlobalState {
    fn default() -> Self {
        Self {
            selected_scene_id: None,
            selected_position_ids: Vec::new(),
            selection_rectangle: None,
            is_place_mode: false,
            interaction_mode: InteractionMode::View,
            scene_list_scroll_offset: 0.0,
            should_scroll_to_selected_scene: false,
            has_closeable_scene_selection: false,
            is_scene_list_search_open: false,
            has_pending_drag_selection: false,
            drag_selected_scene_id: None,
            is_rendering_floor: false,
            redraw_floor: false,
            svg_file_path: None,
        }
    }
}
