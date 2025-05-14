//! Score sanitizer definition
//!
//! ### Author
//! Jakub Kloub (xkloub03), VUT FIT

use crate::{
    error::*,
    notation::{Accidental, KeySignature, NoteName, Score, Stave, Symbol},
    sanitizer::{Sanitizer, ToPrefSynonym, FLAT_KEYS, SHARP_KEYS},
};

use Accidental::*;
use NoteName::*;

pub struct ScoreSanitizer;

/// Score internal structure sanitizer.
impl Sanitizer<Score> for ScoreSanitizer {
    fn sanitize(&self, score: &mut Score) -> Result<()> {
        for stave in score.staves.iter_mut() {
            self.sanitize_stave(stave)?;
        }

        Ok(())
    }
}

impl ScoreSanitizer {
    /// Convert pitch to its preffered accidental.
    fn pitch_to_pref(&self, pref: Option<Accidental>, pitch: &mut crate::notation::Pitch) {
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

    /// Sanitize all stave symbols.
    fn sanitize_stave(&self, stave: &mut Stave) -> Result<()> {
        let (key, key_pos) = self.find_first_key(stave)?;
        let mut preffered_accidental = self.get_pref_accidental(key);

        for sym in stave.symbols.iter_mut().skip(key_pos + 1) {
            match sym {
                Symbol::KeySignature(sig) => preffered_accidental = self.get_pref_accidental(sig),
                Symbol::Note(note) => self.pitch_to_pref(preffered_accidental, &mut note.pitch),
                Symbol::Chord(chord) => {
                    chord
                        .pitches
                        .iter_mut()
                        .for_each(|p| self.pitch_to_pref(preffered_accidental, p));
                }
                _ => {}
            }
        }

        Ok(())
    }

    /// Find first key signature in the stave.
    fn find_first_key<'a>(&self, stave: &'a Stave) -> Result<(&'a KeySignature, usize)> {
        for (i, sym) in stave.symbols.iter().enumerate() {
            match sym {
                Symbol::Note(_) | Symbol::Chord(_) => return Err(AppError::FoundNoteWithoutKey)?,
                Symbol::KeySignature(sig) => return Ok((sig, i)),
                _ => {}
            }
        }

        Err(AppError::StaveKeyNotFound)?
    }

    /// Get preffered accidental for given key.
    fn get_pref_accidental(&self, key: &KeySignature) -> Option<Accidental> {
        if SHARP_KEYS.iter().any(|x| x == key) {
            Some(Accidental::Sharp)
        } else if FLAT_KEYS.iter().any(|x| x == key) {
            Some(Accidental::Flat)
        } else {
            None
        }
    }
}
