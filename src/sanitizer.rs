//! Sanitizer definition
//!
//! ### Author
//! Jakub Kloub

use crate::error::{AppError, Result};
use crate::notation::{Accidental, KeySignature, KeySignatureType, NoteName, Score, Stave, Symbol};

mod to_pref_synonym;
pub use to_pref_synonym::ToPrefSynonym;

use Accidental::*;
use KeySignatureType::*;
use NoteName::*;

#[rustfmt::skip]
const SHARP_KEYS: [KeySignature; 9] = [
    KeySignature { note: C, accidental: None, signature_type: Maj },
    KeySignature { note: G, accidental: None, signature_type: Maj },
    KeySignature { note: D, accidental: None, signature_type: Maj },
    KeySignature { note: A, accidental: None, signature_type: Maj },
    KeySignature { note: E, accidental: None, signature_type: Maj },
    KeySignature { note: B, accidental: None, signature_type: Maj },
    KeySignature { note: E, accidental: Some(Sharp), signature_type: Maj },
    KeySignature { note: B, accidental: Some(Sharp), signature_type: Maj },
    KeySignature { note: F, accidental: Some(Flat), signature_type: Maj },
];

#[rustfmt::skip]
const FLAT_KEYS: [KeySignature; 12] = [
    KeySignature { note: F, accidental: None, signature_type: Maj },
    KeySignature { note: B, accidental: Some(Flat), signature_type: Maj },
    KeySignature { note: E, accidental: Some(Flat), signature_type: Maj },
    KeySignature { note: A, accidental: Some(Flat), signature_type: Maj },
    KeySignature { note: D, accidental: Some(Flat), signature_type: Maj },
    KeySignature { note: G, accidental: Some(Flat), signature_type: Maj },
    KeySignature { note: C, accidental: Some(Flat), signature_type: Maj },
    KeySignature { note: A, accidental: Some(Sharp), signature_type: Maj },
    KeySignature { note: D, accidental: Some(Sharp), signature_type: Maj },
    KeySignature { note: G, accidental: Some(Sharp), signature_type: Maj },
    KeySignature { note: C, accidental: Some(Sharp), signature_type: Maj },
    KeySignature { note: F, accidental: Some(Sharp), signature_type: Maj },
];

pub struct Sanitizer;

impl Sanitizer {
    pub fn sanitize(score: &mut Score) -> Result<()> {
        for stave in score.staves.iter_mut() {
            Sanitizer::sanitize_stave(stave)?;
        }

        Ok(())
    }

    fn sanitize_stave(stave: &mut Stave) -> Result<()> {
        let (key, key_pos) = Sanitizer::find_first_key(stave)?;
        let mut preffered_accidental = Sanitizer::get_pref_accidental(key);

        for sym in stave.symbols.iter_mut().skip(key_pos + 1) {
            match sym {
                Symbol::KeySignature(sig) => {
                    preffered_accidental = Sanitizer::get_pref_accidental(sig)
                }
                Symbol::Note(note) => note.pitch.to_pref_synonym(preffered_accidental),
                Symbol::Chord(chord) => {
                    chord
                        .pitches
                        .iter_mut()
                        .for_each(|p| p.to_pref_synonym(preffered_accidental));
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
