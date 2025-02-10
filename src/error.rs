use std::{error::Error, fmt::Debug};

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("CSSRule has invalid format: '{0}'")]
    CSSRuleParse(String),
    #[error("Error: {0}")]
    Other(String),
}
