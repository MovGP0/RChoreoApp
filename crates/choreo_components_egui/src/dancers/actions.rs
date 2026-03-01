use choreo_master_mobile_json::Color;

#[derive(Debug, Clone, PartialEq)]
pub enum DancersAction {
    LoadFromGlobal,
    ReloadFromGlobal,
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
    SwapDancers,
    ShowDialog { content_id: Option<String> },
    HideDialog,
    Cancel,
    SaveToGlobal,
}
