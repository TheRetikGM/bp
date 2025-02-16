//! Lilypond structure and utilities
//!
//! ### Author
//! Jakub Kloub (xkloub03), VUT FIT

mod lilypond;
mod score_parser;

use std::fmt::Display;

pub use lilypond::LilyNote;
pub use lilypond::LilyStave;
pub use lilypond::LilySymbol;
pub use lilypond::Lilypond;
pub use score_parser::ScoreParser;

#[derive(Debug, Clone)]
pub enum LilyClef {
    Treble,
    Bass,
}

impl Display for LilyClef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                LilyClef::Treble => "\\clef treble",
                LilyClef::Bass => "\\clef bass",
            }
        )
    }
}

#[derive(Debug, Clone)]
#[rustfmt::skip]
pub enum LilyNoteName {
    Ces, C, Cis,
    Des, D, Dis,
    Es, E, Eis,
    Fes, F, Fis,
    Ges, G, Gis,
    As, A, Ais,
    Hes, H, His,
}

impl Display for LilyNoteName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                LilyNoteName::Ces => "ces",
                LilyNoteName::C => "c",
                LilyNoteName::Cis => "cis",
                LilyNoteName::Des => "des",
                LilyNoteName::D => "d",
                LilyNoteName::Dis => "dis",
                LilyNoteName::Es => "es",
                LilyNoteName::E => "e",
                LilyNoteName::Eis => "eis",
                LilyNoteName::Fes => "fes",
                LilyNoteName::F => "f",
                LilyNoteName::Fis => "fis",
                LilyNoteName::Ges => "ges",
                LilyNoteName::G => "g",
                LilyNoteName::Gis => "gis",
                LilyNoteName::As => "as",
                LilyNoteName::A => "a",
                LilyNoteName::Ais => "ais",
                LilyNoteName::Hes => "hes",
                LilyNoteName::H => "h",
                LilyNoteName::His => "his",
            }
        )
    }
}

#[derive(Debug, Clone)]
pub enum LilyKeyType {
    Major,
    Minor,
}

impl Display for LilyKeyType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                LilyKeyType::Major => "\\major",
                LilyKeyType::Minor => "\\minor",
            }
        )
    }
}

#[derive(Debug, Clone)]
#[rustfmt::skip]
pub enum LilyNoteLength {
    L1, L2, L4, L8, L16, L32, L64, L128
}

impl Display for LilyNoteLength {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                LilyNoteLength::L1 => "",
                LilyNoteLength::L2 => "2",
                LilyNoteLength::L4 => "4",
                LilyNoteLength::L8 => "8",
                LilyNoteLength::L16 => "16",
                LilyNoteLength::L32 => "32",
                LilyNoteLength::L64 => "64",
                LilyNoteLength::L128 => "128",
            }
        )
    }
}

#[derive(Debug, Clone)]
pub struct LilyTime {
    pub nom: u8,
    pub denom: LilyNoteLength,
}

impl LilyTime {
    fn c() -> Self {
        Self {
            nom: 4,
            denom: LilyNoteLength::L4,
        }
    }
}

impl Display for LilyTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\\time {}/{}", self.nom, self.denom)
    }
}

#[derive(Debug, Clone)]
pub struct LilyKey {
    pub note: LilyNoteName,
    pub key_type: LilyKeyType,
}

impl Display for LilyKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\\key {} {}", self.note, self.key_type)
    }
}

#[derive(Debug, Clone)]
pub enum OctaveRelative {
    Up(u8),
    Down(u8),
}

impl Display for OctaveRelative {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (c, times) = match self {
            OctaveRelative::Up(times) => ('\'', times),
            OctaveRelative::Down(times) => (',', times),
        };

        write!(
            f,
            "{}",
            std::iter::repeat(c)
                .take(*times as usize)
                .collect::<String>()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_string_octave_relative_up() {
        assert_eq!(OctaveRelative::Up(0).to_string(), "");
        assert_eq!(OctaveRelative::Up(1).to_string(), "'");
        assert_eq!(OctaveRelative::Up(3).to_string(), "'''");
        assert_eq!(OctaveRelative::Up(5).to_string(), "'''''");
    }

    #[test]
    fn to_string_octave_relative_down() {
        assert_eq!(OctaveRelative::Down(0).to_string(), "");
        assert_eq!(OctaveRelative::Down(1).to_string(), ",");
        assert_eq!(OctaveRelative::Down(3).to_string(), ",,,");
        assert_eq!(OctaveRelative::Down(5).to_string(), ",,,,,");
    }

    #[test]
    fn to_string_clef() {
        assert_eq!(LilyClef::Treble.to_string(), "\\clef treble");
        assert_eq!(LilyClef::Bass.to_string(), "\\clef bass");
    }

    #[test]
    fn to_string_key() {
        assert_eq!(
            LilyKey {
                note: LilyNoteName::Cis,
                key_type: LilyKeyType::Major
            }
            .to_string(),
            "\\key cis \\major"
        );

        assert_eq!(
            LilyKey {
                note: LilyNoteName::A,
                key_type: LilyKeyType::Minor
            }
            .to_string(),
            "\\key a \\minor"
        );
    }

    #[test]
    fn to_string_time() {
        assert_eq!(
            LilyTime {
                nom: 3,
                denom: LilyNoteLength::L4
            }
            .to_string(),
            "\\time 3/4"
        );

        assert_eq!(
            LilyTime {
                nom: 6,
                denom: LilyNoteLength::L8
            }
            .to_string(),
            "\\time 6/8"
        );
    }
}
