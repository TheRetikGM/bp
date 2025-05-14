//! ToPrefSyn trait definition
//!
//! ### Author
//! Jakub Kloub

use crate::notation::{Accidental, NoteName, Pitch};

pub trait ToPrefSynonym {
    /// Convert to preffered equivalent note with given accidental.
    fn to_pref_synonym(&mut self, preffer: Option<Accidental>);
}

impl ToPrefSynonym for Pitch {
    /// Convert to preffered equivalent note with given accidental.
    ///
    /// ## Example
    /// D-flat.to_pref_synonym(Sharp) gives C-sharp
    fn to_pref_synonym(&mut self, preffer: Option<Accidental>) {
        use Accidental::*;
        use NoteName::*;

        match preffer {
            Some(Sharp) => match (self.note_name(), self.accidental()) {
                (C, Some(Flat)) => *self = Pitch::new(B, self.octave.try_prev().unwrap(), None),
                (F, Some(Flat)) => *self = Pitch::new(E, self.octave, None),
                (n, Some(Flat)) => *self = Pitch::new(n.prev(), self.octave, Some(Sharp)),
                _ => {}
            },
            Some(Flat) => match (self.note_name(), self.accidental()) {
                (B, Some(Sharp)) => *self = Pitch::new(C, self.octave.try_next().unwrap(), None),
                (E, Some(Sharp)) => *self = Pitch::new(F, self.octave, None),
                (n, Some(Sharp)) => *self = Pitch::new(n.next(), self.octave, Some(Flat)),
                _ => {}
            },
            None => match (self.note_name(), self.accidental()) {
                (E, Some(Sharp)) => *self = Pitch::new(F, self.octave, None),
                (F, Some(Flat)) => *self = Pitch::new(E, self.octave, None),
                (B, Some(Sharp)) => *self = Pitch::new(C, self.octave.try_next().unwrap(), None),
                (C, Some(Flat)) => *self = Pitch::new(B, self.octave.try_prev().unwrap(), None),
                _ => {}
            },
        };
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        notation::{
            Accidental::{self, *},
            NoteName::{self, *},
            Octave::{self, *},
            Pitch,
        },
        sanitizer::to_pref_synonym::ToPrefSynonym,
    };

    fn pitch_eq(pitch: Pitch, to: (NoteName, Octave, Option<Accidental>)) -> bool {
        pitch.note_name() == to.0 && pitch.octave == to.1 && pitch.accidental() == to.2
    }

    #[test]
    fn preffer_none() {
        // Arrange
        let mut bis = Pitch::new(B, O4, Some(Sharp));
        let mut cis = Pitch::new(C, O4, Some(Sharp));
        let mut eis = Pitch::new(E, O4, Some(Sharp));
        let mut ces = Pitch::new(C, O5, Some(Flat));
        let mut es = Pitch::new(E, O4, Some(Flat));
        let mut fes = Pitch::new(F, O4, Some(Flat));

        // Act
        bis.to_pref_synonym(None);
        cis.to_pref_synonym(None);
        eis.to_pref_synonym(None);
        ces.to_pref_synonym(None);
        es.to_pref_synonym(None);
        fes.to_pref_synonym(None);

        // Assert
        assert!(pitch_eq(bis, (C, O5, None)));
        assert!(pitch_eq(cis, (C, O4, Some(Sharp))));
        assert!(pitch_eq(eis, (F, O4, None)));
        assert!(pitch_eq(ces, (B, O4, None)));
        assert!(pitch_eq(es, (E, O4, Some(Flat))));
        assert!(pitch_eq(fes, (E, O4, None)));
    }

    #[test]
    fn preffer_sharp() {
        // Arrange
        let mut ces = Pitch::new(C, O4, Some(Flat));
        let mut des = Pitch::new(D, O4, Some(Flat));
        let mut fes = Pitch::new(F, O4, Some(Flat));
        let mut e = Pitch::new(E, O4, None);

        // Act
        ces.to_pref_synonym(Some(Sharp));
        des.to_pref_synonym(Some(Sharp));
        fes.to_pref_synonym(Some(Sharp));
        e.to_pref_synonym(Some(Sharp));

        // Assert
        assert!(pitch_eq(ces, (B, O3, None)));
        assert!(pitch_eq(des, (C, O4, Some(Sharp))));
        assert!(pitch_eq(fes, (E, O4, None)));
        assert!(pitch_eq(e, (E, O4, None)));
    }

    #[test]
    fn preffer_flat() {
        // Arrange
        let mut bis = Pitch::new(B, O4, Some(Sharp));
        let mut cis = Pitch::new(C, O4, Some(Sharp));
        let mut eis = Pitch::new(E, O4, Some(Sharp));
        let mut f = Pitch::new(F, O4, None);

        // Act
        bis.to_pref_synonym(Some(Flat));
        cis.to_pref_synonym(Some(Flat));
        eis.to_pref_synonym(Some(Flat));
        f.to_pref_synonym(Some(Flat));

        // Assert
        assert!(pitch_eq(bis, (C, O5, None)));
        assert!(pitch_eq(cis, (D, O4, Some(Flat))));
        assert!(pitch_eq(eis, (F, O4, None)));
        assert!(pitch_eq(f, (F, O4, None)));
    }
}
