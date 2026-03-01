use super::state::{InteractionMode, SelectionRectangle};

#[derive(Debug, Clone, PartialEq)]
pub enum GlobalAction {
    Initialize,
    SetInteractionMode {
        mode: InteractionMode,
    },
    SetPlaceMode {
        is_place_mode: bool,
    },
    SetSelectionRectangle {
        rectangle: Option<SelectionRectangle>,
    },
    SetSelectedSceneId {
        scene_id: Option<String>,
    },
    SetSceneListScrollOffset {
        scroll_offset: f32,
    },
    RequestScrollToSelectedScene,
    CompleteScrollToSelectedScene,
    SetPendingDragSelection {
        has_pending_drag_selection: bool,
        drag_selected_scene_id: Option<String>,
    },
    SetFloorRendering {
        is_rendering_floor: bool,
    },
    RequestFloorRedraw,
    CompleteFloorRedraw,
    SetSvgFilePath {
        svg_file_path: Option<String>,
    },
}
