use std::error::Error;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AlgorithmError {
    SizeMismatch(&'static str),
    InvalidParameter(&'static str),
    InvalidNode(&'static str),
    NoPerfectAssignment(&'static str),
}

impl fmt::Display for AlgorithmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AlgorithmError::SizeMismatch(message)
            | AlgorithmError::InvalidParameter(message)
            | AlgorithmError::InvalidNode(message)
            | AlgorithmError::NoPerfectAssignment(message) => write!(f, "{message}"),
        }
    }
}

impl Error for AlgorithmError {}
