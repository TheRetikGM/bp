use std::{error::Error, fmt::Debug};

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("CSSRule has invalid format: '{0}'")]
    CSSRuleParse(String),
    #[error("Cannot find Key symbol in stave.")]
    StaveKeyNotFound,
    #[error("Found a note that isn't bound by any key.")]
    FoundNoteWithoutKey,
    #[error("Error: {0}")]
    Other(String),
}

impl AppError {
    pub fn boxed(self) -> Box<dyn Error> {
        Box::new(self)
    }
}
