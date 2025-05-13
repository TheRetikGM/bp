//! Scale definition
//!
//! ### Author
//! Jakub Kloub (xkloub03), VUT FIT

use serde::{Deserialize, Serialize};

use crate::notation::{KeySignature, KeySignatureType, Octave, Pitch};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScaleType {
    Basic,
    JazzLike,
}

/// Scale a trait that helps us with moving notes in given key.
pub trait Scale: std::fmt::Debug {
    /// Move one note up.
    fn advance(&self, pitch: &mut Pitch);

    // Move one note down.
    fn recede(&self, pitch: &mut Pitch);

    /// Move one note up.
    #[cfg(test)]
    fn next(&self, pitch: &Pitch) -> Pitch {
        let mut p = pitch.clone();
        self.advance(&mut p);
        p
    }

    /// Move one note down.
    #[cfg(test)]
    fn prev(&self, pitch: &Pitch) -> Pitch {
        let mut p = pitch.clone();
        self.recede(&mut p);
        p
    }
}

#[derive(Debug, Clone, Copy)]
pub struct BasicScale {
    pub key: KeySignature,
}

impl BasicScale {
    /// Create a new scale with given key.
    pub fn new(key: KeySignature) -> Self {
        Self { key }
    }
}

impl Scale for BasicScale {
    /// Move one note up.
    fn advance(&self, pitch: &mut Pitch) {
        let ht = Octave::halftone_count();
        let rank = (pitch.value_halftone() + ht - self.key.ext.value_halftone()) % ht;

        match self.key.signature_type {
            KeySignatureType::Maj => match rank {
                4 | 11 => pitch.move_halftone_up(),
                _ => pitch.move_tone_up(),
            },
            // Natural minor
            KeySignatureType::Min => match rank {
                2 | 7 => pitch.move_halftone_up(),
                8 => {
                    pitch.move_tone_up();
                    pitch.move_halftone_up();
                }
                _ => pitch.move_tone_up(),
            },
        }
    }

    // Move one note down.
    fn recede(&self, pitch: &mut Pitch) {
        let ht = Octave::halftone_count();
        let rank = (pitch.value_halftone() + ht - self.key.ext.value_halftone()) % ht;

        match self.key.signature_type {
            KeySignatureType::Maj => match rank {
                0 | 5 => pitch.move_halftone_down(),
                _ => pitch.move_tone_down(),
            },
            KeySignatureType::Min => match rank {
                0 | 3 | 8 => pitch.move_halftone_down(),
                11 => {
                    pitch.move_tone_down();
                    pitch.move_halftone_down();
                }
                _ => pitch.move_tone_down(),
            },
        }
    }
}

#[derive(Debug, Clone)]
pub struct JazzLikeScale {
    pub key: KeySignature,
}

impl JazzLikeScale {
    pub fn new(key: KeySignature) -> Self {
        Self { key }
    }
}

impl Scale for JazzLikeScale {
    fn advance(&self, pitch: &mut Pitch) {
        let ht = Octave::halftone_count();
        let rank = (pitch.value_halftone() + ht - self.key.ext.value_halftone()) % ht;

        match self.key.signature_type {
            KeySignatureType::Maj => match rank {
                2 | 9 => {
                    pitch.move_tone_up();
                    pitch.move_halftone_up();
                }
                7 => pitch.move_halftone_up(),
                8 => pitch.move_halftone_up(),
                _ => pitch.move_tone_up(),
            },
            KeySignatureType::Min => todo!(),
        }
    }

    fn recede(&self, pitch: &mut Pitch) {
        let ht = Octave::halftone_count();
        let rank = (pitch.value_halftone() + ht - self.key.ext.value_halftone()) % ht;

        match self.key.signature_type {
            KeySignatureType::Maj => match rank {
                0 | 5 => {
                    pitch.move_tone_down();
                    pitch.move_halftone_down();
                }
                9 => pitch.move_halftone_down(),
                8 => pitch.move_halftone_down(),
                _ => pitch.move_tone_down(),
            },
            KeySignatureType::Min => todo!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::notation::NoteName::*;
    use crate::notation::Octave::*;
    use crate::notation::{Accidental::*, ExtNoteName};
    use KeySignatureType::*;

    #[test]
    fn advance_maj() {
        // Arrange
        let e_dur = KeySignature {
            ext: ExtNoteName {
                note_name: E,
                accidental: None,
            },
            signature_type: Maj,
        };
        let scale = BasicScale { key: e_dur };
        let n1 = Pitch {
            ext: ExtNoteName {
                note_name: G,
                accidental: Some(Sharp),
            },
            octave: O4,
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
            ext: ExtNoteName {
                note_name: E,
                accidental: None,
            },
            signature_type: Maj,
        };
        let scale = BasicScale { key: e_dur };
        let n1 = Pitch {
            ext: ExtNoteName {
                note_name: G,
                accidental: Some(Sharp),
            },
            octave: O5,
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

    #[test]
    fn advance_min() {
        // Arrange
        let c_min = KeySignature {
            ext: ExtNoteName {
                note_name: C,
                accidental: None,
            },
            signature_type: Min,
        };
        let scale = BasicScale { key: c_min };
        let n1 = Pitch {
            ext: ExtNoteName {
                note_name: C,
                accidental: None,
            },
            octave: O4,
        };

        // Act
        let n2 = scale.next(&n1);
        let n3 = scale.next(&n2);
        let n4 = scale.next(&n3);
        let n5 = scale.next(&n4);
        let n6 = scale.next(&n5);
        let n7 = scale.next(&n6);

        // Assert
        assert_eq!(n2, Pitch::new(D, O4, None));
        assert_eq!(n3, Pitch::new(E, O4, Some(Flat)));
        assert_eq!(n4, Pitch::new(F, O4, None));
        assert_eq!(n5, Pitch::new(G, O4, None));
        assert_eq!(n6, Pitch::new(A, O4, Some(Flat)));
        assert_eq!(n7, Pitch::new(B, O4, None));
    }

    #[test]
    fn recede_min() {
        // Arrange
        let c_min = KeySignature {
            ext: ExtNoteName {
                note_name: C,
                accidental: None,
            },
            signature_type: Min,
        };
        let scale = BasicScale { key: c_min };
        let n1 = Pitch {
            ext: ExtNoteName {
                note_name: C,
                accidental: None,
            },
            octave: O5,
        };

        // Act
        let n2 = scale.prev(&n1);
        let n3 = scale.prev(&n2);
        let n4 = scale.prev(&n3);
        let n5 = scale.prev(&n4);
        let n6 = scale.prev(&n5);
        let n7 = scale.prev(&n6);

        // Assert
        assert_eq!(n2, Pitch::new(B, O4, None));
        assert_eq!(n3, Pitch::new(A, O4, Some(Flat)));
        assert_eq!(n4, Pitch::new(G, O4, None));
        assert_eq!(n5, Pitch::new(F, O4, None));
        assert_eq!(n6, Pitch::new(E, O4, Some(Flat)));
        assert_eq!(n7, Pitch::new(D, O4, None));
    }

    #[test]
    fn advance_jazz() {
        // Arrange
        let c_jazz = KeySignature {
            ext: ExtNoteName {
                note_name: C,
                accidental: Some(Sharp),
            },
            signature_type: Maj,
        };
        let scale = JazzLikeScale { key: c_jazz };
        let n1 = Pitch {
            ext: ExtNoteName {
                note_name: C,
                accidental: Some(Sharp),
            },
            octave: O4,
        };

        // Act
        let n2 = scale.next(&n1);
        let n3 = scale.next(&n2);
        let n4 = scale.next(&n3);
        let n5 = scale.next(&n4);
        let n6 = scale.next(&n5);

        // Assert
        assert_eq!(n2, Pitch::new(D, O4, Some(Sharp)));
        assert_eq!(n3, Pitch::new(F, O4, Some(Sharp)));
        assert_eq!(n4, Pitch::new(G, O4, Some(Sharp)));
        assert_eq!(n5, Pitch::new(A, O4, None));
        assert_eq!(n6, Pitch::new(A, O4, Some(Sharp)));
    }

    #[test]
    fn recede_jazz() {
        // Arrange
        let c_jazz = KeySignature {
            ext: ExtNoteName {
                note_name: C,
                accidental: Some(Sharp),
            },
            signature_type: Maj,
        };
        let scale = JazzLikeScale { key: c_jazz };
        let n1 = Pitch {
            ext: ExtNoteName {
                note_name: C,
                accidental: Some(Sharp),
            },
            octave: O5,
        };

        // Act
        let n2 = scale.prev(&n1);
        let n3 = scale.prev(&n2);
        let n4 = scale.prev(&n3);
        let n5 = scale.prev(&n4);
        let n6 = scale.prev(&n5);

        // Assert
        assert_eq!(n2, Pitch::new(B, O4, Some(Flat)));
        assert_eq!(n3, Pitch::new(A, O4, None));
        assert_eq!(n4, Pitch::new(G, O4, Some(Sharp)));
        assert_eq!(n5, Pitch::new(F, O4, Some(Sharp)));
        assert_eq!(n6, Pitch::new(D, O4, Some(Sharp)));
    }
}
