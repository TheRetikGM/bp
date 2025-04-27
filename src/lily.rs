//! Lilypond structure and utilities
//!
//! ### Author
//! Jakub Kloub (xkloub03), VUT FIT

mod lily_note;
mod lily_stave;
mod lily_symbol;
mod lilypond;

use std::fmt::Display;

pub use lily_note::LilyNote;
pub use lily_stave::LilyStave;
pub use lily_symbol::LilySymbol;
pub use lilypond::Lilypond;

use crate::notation::{
    Accidental, Clef, KeySignature, KeySignatureType, NoteLength, NoteName, Octave, Pitch,
    TimeSignature,
};

#[derive(Debug, Clone)]
pub enum LilyClef {
    Treble,
    Bass,
}

impl From<Clef> for LilyClef {
    fn from(clef: Clef) -> Self {
        match clef {
            Clef::Treble => LilyClef::Treble,
            Clef::Bass => LilyClef::Bass,
        }
    }
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

impl From<(Option<Accidental>, NoteName)> for LilyNoteName {
    fn from(value: (Option<Accidental>, NoteName)) -> Self {
        let select = |note_names: &[LilyNoteName]| {
            if let Some(acc) = value.0 {
                return match acc {
                    Accidental::Sharp => note_names[1],
                    Accidental::Flat => note_names[2],
                };
            }

            note_names[0]
        };

        use LilyNoteName::*;

        match value.1 {
            NoteName::C => select(&[C, Cis, Ces]),
            NoteName::D => select(&[D, Dis, Des]),
            NoteName::E => select(&[E, Eis, Es]),
            NoteName::F => select(&[F, Fis, Fes]),
            NoteName::G => select(&[G, Gis, Ges]),
            NoteName::A => select(&[A, Ais, As]),
            NoteName::B => select(&[H, His, Hes]),
        }
    }
}

impl From<Pitch> for LilyNoteName {
    fn from(pitch: Pitch) -> Self {
        (pitch.accidental, pitch.note_name).into()
    }
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LilyKeyType {
    Major,
    Minor,
}

impl From<KeySignatureType> for LilyKeyType {
    fn from(sig_type: KeySignatureType) -> Self {
        match sig_type {
            KeySignatureType::Maj => LilyKeyType::Major,
            KeySignatureType::Min => LilyKeyType::Minor,
        }
    }
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

#[derive(Debug, Clone, PartialEq, Eq)]
#[rustfmt::skip]
pub enum LilyNoteLength {
    L1, L2, L4, L8, L16, L32, L64, L128
}

impl From<NoteLength> for LilyNoteLength {
    fn from(duration: NoteLength) -> Self {
        match duration {
            NoteLength::L1 => LilyNoteLength::L1,
            NoteLength::L2 => LilyNoteLength::L2,
            NoteLength::L4 => LilyNoteLength::L4,
            NoteLength::L8 => LilyNoteLength::L8,
            NoteLength::L16 => LilyNoteLength::L16,
            NoteLength::L32 => LilyNoteLength::L32,
            NoteLength::L64 => LilyNoteLength::L64,
            NoteLength::L128 => LilyNoteLength::L128,
        }
    }
}

impl Display for LilyNoteLength {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                LilyNoteLength::L1 => "1",
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
    pub fn c() -> Self {
        Self {
            nom: 4,
            denom: LilyNoteLength::L4,
        }
    }
}

impl From<TimeSignature> for LilyTime {
    fn from(time: TimeSignature) -> Self {
        Self {
            nom: time.beat_count,
            denom: time.single_beat_note.into(),
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

impl From<KeySignature> for LilyKey {
    fn from(key: KeySignature) -> Self {
        Self {
            note: (key.accidental, key.note).into(),
            key_type: key.signature_type.into(),
        }
    }
}

impl Display for LilyKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\\key {} {}", self.note, self.key_type)
    }
}

#[cfg(test)]
impl PartialEq for LilyKey {
    fn eq(&self, other: &Self) -> bool {
        self.note == other.note && self.key_type == other.key_type
    }
}

#[derive(Debug, Clone)]
pub enum OctaveRelative {
    Up(u8),
    Down(u8),
}

impl From<Octave> for OctaveRelative {
    fn from(octave: Octave) -> Self {
        match octave {
            Octave::O0 => OctaveRelative::Down(3),
            Octave::O1 => OctaveRelative::Down(2),
            Octave::O2 => OctaveRelative::Down(1),
            Octave::O3 => OctaveRelative::Up(0),
            Octave::O4 => OctaveRelative::Up(1),
            Octave::O5 => OctaveRelative::Up(2),
            Octave::O6 => OctaveRelative::Up(3),
            Octave::O7 => OctaveRelative::Up(4),
            Octave::O8 => OctaveRelative::Up(5),
            Octave::O9 => OctaveRelative::Up(6),
        }
    }
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

    #[test]
    fn key_into_lily() {
        assert_eq!(
            LilyKey::from(KeySignature {
                note: NoteName::G,
                accidental: None,
                signature_type: KeySignatureType::Maj
            }),
            LilyKey {
                note: LilyNoteName::G,
                key_type: LilyKeyType::Major
            }
        );
        assert_eq!(
            LilyKey::from(KeySignature {
                note: NoteName::F,
                accidental: Some(Accidental::Sharp),
                signature_type: KeySignatureType::Min
            }),
            LilyKey {
                note: LilyNoteName::Fis,
                key_type: LilyKeyType::Minor
            }
        );
        assert_eq!(
            LilyKey::from(KeySignature {
                note: NoteName::B,
                accidental: Some(Accidental::Flat),
                signature_type: KeySignatureType::Maj
            }),
            LilyKey {
                note: LilyNoteName::Hes,
                key_type: LilyKeyType::Major
            }
        );
    }
}
