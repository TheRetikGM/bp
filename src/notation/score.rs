//! Struct definitions
//!
//! ### Author
//! Jakub Kloub (xkloub03), VUT FIT

use crate::{notation::stave::Stave, sanitizer::Sanitizer};

type Tempo = u32;

#[derive(Debug, Default, Clone)]
pub struct ScoreInfo {
    pub name: Option<String>,
    pub author: Option<String>,
    pub transcriber: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Score {
    pub staves: Vec<Stave>,
    pub info: ScoreInfo,
    pub tempo: Tempo,
}

impl Default for Score {
    fn default() -> Self {
        Self {
            staves: Default::default(),
            info: Default::default(),
            tempo: 90,
        }
    }
}

impl Score {
    pub fn sanitized(mut self) -> crate::error::Result<Self> {
        Sanitizer::sanitize(&mut self)?;
        Ok(self)
    }
}
