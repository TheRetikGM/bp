//! Struct definitions
//!
//! ### Author
//! Jakub Kloub (xkloub03), VUT FIT

use crate::notation::stave::Stave;

type Tempo = u32;

#[derive(Debug)]
pub struct ScoreInfo {
    pub name: Option<String>,
    pub author: Option<String>,
    pub transcriber: Option<String>,
}

#[derive(Debug)]
pub struct Score {
    pub tracks: Vec<Stave>,
    pub info: ScoreInfo,
    pub tempo: Tempo,
}
