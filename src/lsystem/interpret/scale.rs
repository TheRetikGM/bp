//! Scale definition
//!
//! ### Author
//! Jakub Kloub (xkloub03), VUT FIT

use crate::notation::{KeySignature, KeySignatureType, Octave, Pitch};

/// Scale a structure that helps us with moving notes in given key.
#[derive(Debug, Clone, Copy)]
pub struct Scale {
    pub key: KeySignature,
}

impl Scale {
    /// Create a new scale with given key.
    pub fn new(key: KeySignature) -> Self {
        Self { key }
    }

    /// Move one note up.
    pub fn advance(&self, pitch: &mut Pitch) {
        let ht = Octave::halftone_count();
        let rank = (pitch.value_halftone() + ht - self.key.note.value_halftone()) % ht;

        match self.key.signature_type {
            KeySignatureType::Maj => match rank {
                4 | 11 => pitch.move_halftone_up(),
                _ => pitch.move_tone_up(),
            },
            KeySignatureType::Min => todo!(),
        }
    }

    // Move one note down.
    pub fn recede(&self, pitch: &mut Pitch) {
        let ht = Octave::halftone_count();
        let rank = (pitch.value_halftone() + ht - self.key.note.value_halftone()) % ht;

        match self.key.signature_type {
            KeySignatureType::Maj => match rank {
                0 | 5 => pitch.move_halftone_down(),
                _ => pitch.move_tone_down(),
            },
            KeySignatureType::Min => todo!(),
        }
    }

    /// Move one note up.
    pub fn next(&self, pitch: &Pitch) -> Pitch {
        let mut p = pitch.clone();
        self.advance(&mut p);
        p
    }

    /// Move one note down.
    pub fn prev(&self, pitch: &Pitch) -> Pitch {
        let mut p = pitch.clone();
        self.recede(&mut p);
        p
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::notation::Accidental::*;
    use crate::notation::NoteName::*;
    use crate::notation::Octave::*;
    use KeySignatureType::*;

    #[test]
    fn advance_maj() {
        // Arrange
        let e_dur = KeySignature {
            note: E,
            signature_type: Maj,
            accidental: None,
        };
        let scale = Scale { key: e_dur };
        let n1 = Pitch {
            note_name: G,
            octave: O4,
            accidental: Some(Sharp),
        };

        // Act
        let n2 = scale.next(&n1);
        let n3 = scale.next(&n2);
        let n4 = scale.next(&n3);
        let n5 = scale.next(&n4);
        let n6 = scale.next(&n5);
        let n7 = scale.next(&n6);

        // Assert
        assert_eq!(n2, Pitch::new(A, O4, None));
        assert_eq!(n3, Pitch::new(B, O4, None));
        assert_eq!(n4, Pitch::new(C, O5, Some(Sharp)));
        assert_eq!(n5, Pitch::new(D, O5, Some(Sharp)));
        assert_eq!(n6, Pitch::new(E, O5, None));
        assert_eq!(n7, Pitch::new(F, O5, Some(Sharp)));
    }

    #[test]
    fn recede_maj() {
        // Arrange
        let e_dur = KeySignature {
            note: E,
            signature_type: Maj,
            accidental: None,
        };
        let scale = Scale { key: e_dur };
        let n1 = Pitch {
            note_name: G,
            octave: O5,
            accidental: Some(Sharp),
        };

        // Act
        let n2 = scale.prev(&n1);
        let n3 = scale.prev(&n2);
        let n4 = scale.prev(&n3);
        let n5 = scale.prev(&n4);
        let n6 = scale.prev(&n5);
        let n7 = scale.prev(&n6);

        // Assert
        assert_eq!(n2, Pitch::new(F, O5, Some(Sharp)));
        assert_eq!(n3, Pitch::new(E, O5, None));
        assert_eq!(n4, Pitch::new(D, O5, Some(Sharp)));
        assert_eq!(n5, Pitch::new(C, O5, Some(Sharp)));
        assert_eq!(n6, Pitch::new(B, O4, None));
        assert_eq!(n7, Pitch::new(A, O4, None));
    }
}
