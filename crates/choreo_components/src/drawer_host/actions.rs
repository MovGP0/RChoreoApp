#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DrawerHostAction {
    OverlayClicked {
        close_left: bool,
        close_right: bool,
        close_top: bool,
        close_bottom: bool,
    },
}
