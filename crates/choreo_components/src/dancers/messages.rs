#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CloseDancerDialogCommand;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShowDancerDialogCommand {
    pub content_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AddDancerCommand;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeleteDancerCommand;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SaveDancerSettingsCommand;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CancelDancerSettingsCommand;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SwapDancersCommand;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpdateSwapSelectionCommand;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DancerSelectionCommand {
    Select(usize),
    Refresh,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SelectRoleCommand {
    pub index: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpdateDancerIconCommand {
    pub value: String,
}
