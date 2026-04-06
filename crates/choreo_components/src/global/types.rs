use std::rc::Rc;

use choreo_master_mobile_json::Color;
use choreo_master_mobile_json::SceneId;
use choreo_models::ChoreographyModel;
use choreo_models::PositionModel;
use choreo_models::SceneModel;

use crate::floor::state::Matrix;
use crate::floor::state::Point;

pub type SceneViewModel = crate::scene_list_item::SceneItemState;

// egui does not expose a retained canvas handle equivalent to the Slint view handle,
// so parity is modeled as an opaque placeholder until a real handle is needed.
pub type MainCanvasViewHandle = ();

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SelectionRectangle {
    pub start: Point,
    pub end: Point,
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

#[derive(Debug, Default)]
pub struct GlobalStateModel {
    pub choreography: ChoreographyModel,
    pub scenes: Vec<SceneViewModel>,
    pub selected_scene: Option<SceneViewModel>,
    pub selected_positions: Vec<PositionModel>,
    pub selected_positions_snapshot: Vec<PositionModel>,
    pub selection_rectangle: Option<SelectionRectangle>,
    pub is_place_mode: bool,
    pub interaction_mode: InteractionMode,
    pub main_canvas_view: Option<Rc<MainCanvasViewHandle>>,
    pub main_canvas_view_matrix: Matrix,
    pub scene_list_scroll_offset: f32,
    pub should_scroll_to_selected_scene: bool,
    pub has_closeable_scene_selection: bool,
    pub selected_scene_model: Option<SceneModel>,
    pub is_scene_list_search_open: bool,
    pub has_pending_drag_selection: bool,
    pub drag_selected_scene_id: Option<SceneId>,
    pub pending_scale_center: Option<(f64, f64)>,
    pub selected_dancer_role_color: Option<Color>,
    pub is_rendering_floor: bool,
    pub redraw_floor: bool,
    pub svg_file_path: Option<String>,
}

impl choreo_state_machine::GlobalStateModel for GlobalStateModel {}
