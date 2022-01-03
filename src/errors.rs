use std::{error::Error, fmt};

#[derive(Debug)]
pub enum PasteErrorKind {
    FileNotFound,
    FileRead,
    FileWrite,
    InvalidRequest,
}

#[derive(Debug)]
pub struct PasteError {
    pub error_kind: PasteErrorKind,
    pub details: String,
}

impl PasteError {
    pub fn new(error_kind: PasteErrorKind, msg: &str) -> PasteError {
        PasteError {
            error_kind,
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for PasteError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for PasteError {
    fn description(&self) -> &str {
        &self.details
    }
}

impl warp::reject::Reject for PasteError {}
