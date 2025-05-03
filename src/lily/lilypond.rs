//! Lilypond struct definition
//!
//! ### Author
//! Jakub Kloub (xkloub03), VUT FIT

use crate::{lily::lily_stave::LilyStave, notation::Score};
use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Lilypond {
    pub version: String,
    pub language: String,
    pub staves: Vec<LilyStave>,
}

impl Default for Lilypond {
    fn default() -> Self {
        Self {
            version: "2.25.20".to_owned(),
            language: "deutsch".to_owned(),
            staves: Default::default(),
        }
    }
}

impl Display for Lilypond {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let staves: String = self.staves.iter().map(|s| s.to_string()).collect();

        write!(
            f,
            "\\version \"{}\"\n\\language \"{}\"\n\\score{{{}\\layout{{}}\\midi{{}}}}",
            self.version, self.language, staves
        )
    }
}

impl From<Score> for Lilypond {
    fn from(value: Score) -> Self {
        Self {
            staves: value.staves.into_iter().map(Into::into).collect(),
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lily::lily_stave::tests::stave_d_maj;

    #[test]
    fn to_string() {
        // Arrange
        let (d_maj, _) = stave_d_maj();
        let lilypond = Lilypond {
            version: "1.2.3".to_owned(),
            language: "english".to_owned(),
            staves: vec![d_maj.clone(), d_maj.clone()],
        };

        // Act && Assert
        assert_eq!(
            lilypond.to_string().trim(),
            format!("\\version \"1.2.3\"\n\\language \"english\"\n\\score{{{d_maj}{d_maj}\\layout{{}}\\midi{{}}}}")
        );
    }
}
