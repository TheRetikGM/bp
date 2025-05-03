//! Sanitizer definition
//!
//! ### Author
//! Jakub Kloub

use crate::notation::{Accidental, KeySignature, KeySignatureType, NoteName, Score, Stave, Symbol};
use crate::{
    error::{AppError, Result},
    notation::ExtNoteName,
};

mod to_pref_synonym;
pub use to_pref_synonym::ToPrefSynonym;

use Accidental::*;
use KeySignatureType::*;
use NoteName::*;

#[rustfmt::skip]
const SHARP_KEYS: [KeySignature; 9] = [
    KeySignature { ext: ExtNoteName { note_name: C, accidental: None }, signature_type: Maj },
    KeySignature { ext: ExtNoteName { note_name: G, accidental: None }, signature_type: Maj },
    KeySignature { ext: ExtNoteName { note_name: D, accidental: None }, signature_type: Maj },
    KeySignature { ext: ExtNoteName { note_name: A, accidental: None }, signature_type: Maj },
    KeySignature { ext: ExtNoteName { note_name: E, accidental: None }, signature_type: Maj },
    KeySignature { ext: ExtNoteName { note_name: B, accidental: None }, signature_type: Maj },
    KeySignature { ext: ExtNoteName { note_name: E, accidental: Some(Sharp) }, signature_type: Maj },
    KeySignature { ext: ExtNoteName { note_name: B, accidental: Some(Sharp) }, signature_type: Maj },
    KeySignature { ext: ExtNoteName { note_name: F, accidental: Some(Flat) }, signature_type: Maj },
];

#[rustfmt::skip]
const FLAT_KEYS: [KeySignature; 12] = [
    KeySignature { ext: ExtNoteName { note_name: F, accidental: None }, signature_type: Maj },
    KeySignature { ext: ExtNoteName { note_name: B, accidental: Some(Flat) }, signature_type: Maj },
    KeySignature { ext: ExtNoteName { note_name: E, accidental: Some(Flat) }, signature_type: Maj },
    KeySignature { ext: ExtNoteName { note_name: A, accidental: Some(Flat) }, signature_type: Maj },
    KeySignature { ext: ExtNoteName { note_name: D, accidental: Some(Flat) }, signature_type: Maj },
    KeySignature { ext: ExtNoteName { note_name: G, accidental: Some(Flat) }, signature_type: Maj },
    KeySignature { ext: ExtNoteName { note_name: C, accidental: Some(Flat) }, signature_type: Maj },
    KeySignature { ext: ExtNoteName { note_name: A, accidental: Some(Sharp) }, signature_type: Maj },
    KeySignature { ext: ExtNoteName { note_name: D, accidental: Some(Sharp) }, signature_type: Maj },
    KeySignature { ext: ExtNoteName { note_name: G, accidental: Some(Sharp) }, signature_type: Maj },
    KeySignature { ext: ExtNoteName { note_name: C, accidental: Some(Sharp) }, signature_type: Maj },
    KeySignature { ext: ExtNoteName { note_name: F, accidental: Some(Sharp) }, signature_type: Maj },
];

pub struct Sanitizer;

impl Sanitizer {
    pub fn sanitize(score: &mut Score) -> Result<()> {
        for stave in score.staves.iter_mut() {
            Sanitizer::sanitize_stave(stave)?;
        }

        Ok(())
    }

    fn pitch_to_pref(pref: Option<Accidental>, pitch: &mut crate::notation::Pitch) {
        match pref {
            Some(Sharp) => match (pitch.note_name(), pitch.accidental()) {
                (C, None) | (F, None) => {}
                (B, Some(Sharp)) | (E, Some(Sharp)) => pitch.to_pref_synonym(None),
                _ => pitch.to_pref_synonym(pref),
            },
            Some(Flat) => match (pitch.note_name(), pitch.accidental()) {
                (E, None) | (B, None) => {}
                (F, Some(Flat)) | (C, Some(Flat)) => pitch.to_pref_synonym(None),
                _ => pitch.to_pref_synonym(pref),
            },
            _ => pitch.to_pref_synonym(pref),
        }
    }

    fn sanitize_stave(stave: &mut Stave) -> Result<()> {
        let (key, key_pos) = Sanitizer::find_first_key(stave)?;
        let mut preffered_accidental = Sanitizer::get_pref_accidental(key);

        for sym in stave.symbols.iter_mut().skip(key_pos + 1) {
            match sym {
                Symbol::KeySignature(sig) => {
                    preffered_accidental = Sanitizer::get_pref_accidental(sig)
                }
                Symbol::Note(note) => {
                    Sanitizer::pitch_to_pref(preffered_accidental, &mut note.pitch)
                }
                Symbol::Chord(chord) => {
                    chord
                        .pitches
                        .iter_mut()
                        .for_each(|p| Sanitizer::pitch_to_pref(preffered_accidental, p));
                }
                _ => {}
            }
        }

        Ok(())
    }

    fn find_first_key(stave: &Stave) -> Result<(&KeySignature, usize)> {
        for (i, sym) in stave.symbols.iter().enumerate() {
            match sym {
                Symbol::Note(_) | Symbol::Chord(_) => return Err(AppError::FoundNoteWithoutKey)?,
                Symbol::KeySignature(sig) => return Ok((sig, i)),
                _ => {}
            }
        }

        Err(AppError::StaveKeyNotFound)?
    }

    fn get_pref_accidental(key: &KeySignature) -> Option<Accidental> {
        if SHARP_KEYS.iter().any(|x| x == key) {
            Some(Accidental::Sharp)
        } else if FLAT_KEYS.iter().any(|x| x == key) {
            Some(Accidental::Flat)
        } else {
            None
        }
    }
}
