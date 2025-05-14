//! Sanitizer definition
//!
//! ### Author
//! Jakub Kloub

mod lily_sanitizer;
mod score_sanitizer;
mod to_pref_synonym;

pub use lily_sanitizer::LilySanitizer;
pub use score_sanitizer::ScoreSanitizer;
pub use to_pref_synonym::ToPrefSynonym;

use crate::notation::KeySignature;
use crate::notation::{Accidental::*, KeySignatureType::*, NoteName::*};
use crate::{error::Result, notation::ExtNoteName};

#[rustfmt::skip]
const SHARP_KEYS: [KeySignature; 10] = [
    KeySignature { ext: ExtNoteName { note_name: C, accidental: None }, signature_type: Maj },
    KeySignature { ext: ExtNoteName { note_name: G, accidental: None }, signature_type: Maj },
    KeySignature { ext: ExtNoteName { note_name: D, accidental: None }, signature_type: Maj },
    KeySignature { ext: ExtNoteName { note_name: A, accidental: None }, signature_type: Maj },
    KeySignature { ext: ExtNoteName { note_name: E, accidental: None }, signature_type: Maj },
    KeySignature { ext: ExtNoteName { note_name: B, accidental: None }, signature_type: Maj },
    KeySignature { ext: ExtNoteName { note_name: E, accidental: Some(Sharp) }, signature_type: Maj },
    KeySignature { ext: ExtNoteName { note_name: B, accidental: Some(Sharp) }, signature_type: Maj },
    KeySignature { ext: ExtNoteName { note_name: F, accidental: Some(Flat) }, signature_type: Maj },
    KeySignature { ext: ExtNoteName { note_name: C, accidental: Some(Sharp) }, signature_type: Maj },
];

#[rustfmt::skip]
const FLAT_KEYS: [KeySignature; 11] = [
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
    KeySignature { ext: ExtNoteName { note_name: F, accidental: Some(Sharp) }, signature_type: Maj },
];

/// Generic sanitizer trait used by all sanitizers.
pub trait Sanitizer<T> {
    fn sanitize(&self, score: &mut T) -> Result<()>;
}
