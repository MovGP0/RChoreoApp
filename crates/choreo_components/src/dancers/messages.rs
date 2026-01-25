#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CloseDancerDialogCommand;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShowDancerDialogCommand {
    pub content_id: Option<String>,
}
