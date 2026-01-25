use thiserror::Error;

#[derive(Debug, Error)]
pub enum ChoreoJsonError {
    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("expected object for {0}")]
    ExpectedObject(&'static str),
    #[error("expected array for {0}")]
    ExpectedArray(&'static str),
    #[error("missing field {0}")]
    MissingField(&'static str),
    #[error("invalid reference id {0}")]
    InvalidReference(String),
}
