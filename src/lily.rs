//! Lilypond structure and utilities
//!
//! ### Author
//! Jakub Kloub (xkloub03), VUT FIT

mod lilypond;
mod score_parser;

pub use score_parser::ScoreParser;

pub trait ToLily {
    fn to_lily(&self) -> String;
}

#[derive(Debug, Clone)]
pub enum LilyClef {
    Treble,
    Bass,
}

impl ToLily for LilyClef {
    fn to_lily(&self) -> String {
        match self {
            LilyClef::Treble => "\\clef treble",
            LilyClef::Bass => "\\clef bass",
        }
        .to_owned()
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

impl ToLily for LilyNoteName {
    fn to_lily(&self) -> String {
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
        .to_owned()
    }
}

#[derive(Debug, Clone)]
pub enum LilyKeyType {
    Major,
    Minor,
}

impl ToLily for LilyKeyType {
    fn to_lily(&self) -> String {
        match self {
            LilyKeyType::Major => "\\major",
            LilyKeyType::Minor => "\\minor",
        }
        .to_owned()
    }
}

#[derive(Debug, Clone)]
#[rustfmt::skip]
pub enum LilyNoteLength {
    L1, L2, L4, L8, L16, L32, L64, L128
}

impl ToLily for LilyNoteLength {
    fn to_lily(&self) -> String {
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
        .to_owned()
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

impl ToLily for LilyTime {
    fn to_lily(&self) -> String {
        format!("\\time {}/{}", self.nom, self.denom.to_lily())
    }
}

#[derive(Debug, Clone)]
pub struct LilyKey {
    pub note: LilyNoteName,
    pub key_type: LilyKeyType,
}

impl ToLily for LilyKey {
    fn to_lily(&self) -> String {
        format!("\\key {} {}", self.note.to_lily(), self.key_type.to_lily())
    }
}

#[derive(Debug, Clone)]
pub enum OctaveRelative {
    Up(u8),
    Down(u8),
}

impl ToLily for OctaveRelative {
    fn to_lily(&self) -> String {
        let (c, times) = match self {
            OctaveRelative::Up(times) => ('\'', times),
            OctaveRelative::Down(times) => (',', times),
        };

        std::iter::repeat(c).take(*times as usize).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_lily_octave_relative_up() {
        assert_eq!(OctaveRelative::Up(0).to_lily(), "");
        assert_eq!(OctaveRelative::Up(1).to_lily(), "'");
        assert_eq!(OctaveRelative::Up(3).to_lily(), "'''");
        assert_eq!(OctaveRelative::Up(5).to_lily(), "'''''");
    }

    #[test]
    fn to_lily_octave_relative_down() {
        assert_eq!(OctaveRelative::Down(0).to_lily(), "");
        assert_eq!(OctaveRelative::Down(1).to_lily(), ",");
        assert_eq!(OctaveRelative::Down(3).to_lily(), ",,,");
        assert_eq!(OctaveRelative::Down(5).to_lily(), ",,,,,");
    }

    #[test]
    fn to_lily_clef() {
        assert_eq!(LilyClef::Treble.to_lily(), "\\clef treble");
        assert_eq!(LilyClef::Bass.to_lily(), "\\clef bass");
    }

    #[test]
    fn to_lily_key() {
        assert_eq!(
            LilyKey {
                note: LilyNoteName::Cis,
                key_type: LilyKeyType::Major
            }
            .to_lily(),
            "\\key cis \\major"
        );

        assert_eq!(
            LilyKey {
                note: LilyNoteName::A,
                key_type: LilyKeyType::Minor
            }
            .to_lily(),
            "\\key a \\minor"
        );
    }

    #[test]
    fn to_lily_time() {
        assert_eq!(
            LilyTime {
                nom: 3,
                denom: LilyNoteLength::L4
            }
            .to_lily(),
            "\\time 3/4"
        );

        assert_eq!(
            LilyTime {
                nom: 6,
                denom: LilyNoteLength::L8
            }
            .to_lily(),
            "\\time 6/8"
        );
    }
}
