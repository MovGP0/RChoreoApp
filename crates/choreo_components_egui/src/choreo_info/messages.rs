#[derive(Debug, Clone, PartialEq)]
pub enum ChoreoInfoAction {
    UpdateComment(String),
    UpdateName(String),
    UpdateSubtitle(String),
    UpdateDate {
        year: i32,
        month: u8,
        day: u8,
    },
    UpdateVariation(String),
    UpdateAuthor(String),
    UpdateDescription(String),
}
