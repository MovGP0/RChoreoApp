use super::actions::DrawerHostAction;
use super::state::DrawerHostState;

pub fn reduce(state: &mut DrawerHostState, action: DrawerHostAction) {
    match action {
        DrawerHostAction::OverlayClicked {
            close_left,
            close_right,
            close_top,
            close_bottom,
        } => {
            if close_left {
                state.is_left_open = false;
            }
            if close_right {
                state.is_right_open = false;
            }
            if close_top {
                state.is_top_open = false;
            }
            if close_bottom {
                state.is_bottom_open = false;
            }
        }
    }
}
