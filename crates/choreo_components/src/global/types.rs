use std::rc::Rc;

use choreo_models::{ChoreographyModel, PositionModel, SceneModel};

use crate::scenes::SceneViewModel;

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

#[derive(Debug, Default)]
pub struct GlobalStateModel {
    pub choreography: ChoreographyModel,
    pub scenes: Vec<SceneViewModel>,
    pub selected_scene: Option<SceneViewModel>,
    pub selected_positions: Vec<PositionModel>,
    pub selected_positions_snapshot: Vec<PositionModel>,
    pub is_place_mode: bool,
    pub interaction_mode: InteractionMode,
    pub main_canvas_view: Option<Rc<crate::floor::CanvasViewHandle>>,
    pub main_canvas_view_matrix: crate::floor::Matrix,
    pub scene_list_scroll_offset: f32,
    pub should_scroll_to_selected_scene: bool,
    pub has_closeable_scene_selection: bool,
    pub selected_scene_model: Option<SceneModel>,
    pub is_scene_list_search_open: bool,
    pub has_pending_drag_selection: bool,
    pub drag_selected_scene_id: Option<choreo_master_mobile_json::SceneId>,
    pub pending_scale_center: Option<(f64, f64)>,
    pub selected_dancer_role_color: Option<choreo_master_mobile_json::Color>,
    pub is_rendering_floor: bool,
    pub redraw_floor: bool,
    pub svg_file_path: Option<String>,
}
