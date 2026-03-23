use choreo_master_mobile_json::Color;

#[derive(Debug, Clone, PartialEq)]
pub enum DancerSettingsPageAction {
    ToggleDancerList,
    CloseDancerList,
    AddDancer,
    DeleteSelectedDancer,
    SelectDancer { index: usize },
    SelectRole { index: usize },
    UpdateDancerName { value: String },
    UpdateDancerShortcut { value: String },
    UpdateDancerColor { value: Color },
    UpdateDancerIcon { value: String },
    UpdateSwapFrom { index: usize },
    UpdateSwapTo { index: usize },
    RequestSwapDancers,
    DismissDialog,
    ConfirmSwapDialog,
    CancelPage,
    SavePage,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SwapDialogAction {
    Cancel,
    Confirm,
}
