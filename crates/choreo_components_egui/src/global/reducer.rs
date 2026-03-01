use super::actions::GlobalAction;
use super::state::GlobalState;

pub fn reduce(state: &mut GlobalState, action: GlobalAction) {
    match action {
        GlobalAction::Initialize => *state = GlobalState::default(),
        GlobalAction::SetInteractionMode { mode } => state.interaction_mode = mode,
        GlobalAction::SetPlaceMode { is_place_mode } => state.is_place_mode = is_place_mode,
        GlobalAction::SetSelectionRectangle { rectangle } => state.selection_rectangle = rectangle,
        GlobalAction::SetSelectedSceneId { scene_id } => state.selected_scene_id = scene_id,
        GlobalAction::SetSceneListScrollOffset { scroll_offset } => {
            state.scene_list_scroll_offset = scroll_offset;
        }
        GlobalAction::RequestScrollToSelectedScene => state.should_scroll_to_selected_scene = true,
        GlobalAction::CompleteScrollToSelectedScene => {
            state.should_scroll_to_selected_scene = false;
        }
        GlobalAction::SetPendingDragSelection {
            has_pending_drag_selection,
            drag_selected_scene_id,
        } => {
            state.has_pending_drag_selection = has_pending_drag_selection;
            state.drag_selected_scene_id = drag_selected_scene_id;
        }
        GlobalAction::SetFloorRendering { is_rendering_floor } => {
            state.is_rendering_floor = is_rendering_floor;
        }
        GlobalAction::RequestFloorRedraw => state.redraw_floor = true,
        GlobalAction::CompleteFloorRedraw => state.redraw_floor = false,
        GlobalAction::SetSvgFilePath { svg_file_path } => state.svg_file_path = svg_file_path,
    }
}
