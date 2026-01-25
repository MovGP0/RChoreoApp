#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CloseDialogCommand;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OpenSvgFileCommand {
    pub file_path: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShowDialogCommand {
    pub content: Option<String>,
}
