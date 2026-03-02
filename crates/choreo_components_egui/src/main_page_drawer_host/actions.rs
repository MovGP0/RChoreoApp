#[derive(Debug, Clone, PartialEq)]
pub enum MainPageDrawerHostAction {
    Initialize,
    SetInlineLeft {
        inline: bool,
    },
    SetLeftOpen {
        is_open: bool,
    },
    SetRightOpen {
        is_open: bool,
    },
    SetTopInset {
        top_inset: f32,
    },
    OverlayClicked,
}
