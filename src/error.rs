//! Common application errors and results
//!
//! ### Author
//! Jakub Kloub (xkloub03), VUT FIT

pub struct ControlPanel;
use std::error::Error;
use std::path::Path;

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("CSSRule has invalid format: '{0}'")]
    CSSRuleParse(String),

    #[error("CSSRule has invalid format: {1}: '{0}'")]
    CSSRuleParseNum(String, String),

    #[error("Cannot find Key symbol in stave.")]
    StaveKeyNotFound,

    #[error("Found a note that isn't bound by any key.")]
    FoundNoteWithoutKey,

    #[error("CSSRule probability sum isn't 1 for context char '{0}'. Tolerance is: {1:e}")]
    CSSRuleSumNotOne(char, f32),

    #[error("Lilypond translation failed: {0}")]
    Lily(String),

    #[error("Fluidsynth translation failed: {0}")]
    Fluidsynth(String),

    #[error("Path '{0}': {1}")]
    Path(String, String),

    #[error("Audio error: {0}")]
    Audio(String),

    #[error("AudioPlayer: {0}")]
    AudioPlayer(String),

    #[error("Argument error: {0}")]
    Argument(String),

    #[error("Score is missing a time signature.")]
    MissingTimeSignature,

    #[error("Error: {0}")]
    Other(String),
}

impl AppError {
    pub fn boxed(self) -> Box<dyn Error> {
        Box::new(self)
    }

    pub fn build_path<T: std::fmt::Display + ?Sized>(path: &Path, err: &T) -> Self {
        Self::Path(path.display().to_string(), err.to_string())
    }
}
