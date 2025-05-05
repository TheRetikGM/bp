//! Symbol definitions
//!
//! ### Author
//! Jakub Kloub

use crate::notation::*;

#[derive(Debug, Clone, Eq)]
pub struct Pitch {
    pub ext: ExtNoteName,
    pub octave: Octave,
}

impl Pitch {
    pub fn new(note_name: NoteName, octave: Octave, accidental: Option<Accidental>) -> Self {
        Self {
            ext: ExtNoteName {
                note_name,
                accidental,
            },
            octave,
        }
    }

    pub fn note_name(&self) -> NoteName {
        self.ext.note_name
    }

    pub fn accidental(&self) -> Option<Accidental> {
        self.ext.accidental
    }

    /// Halftone value after applying accidental
    ///
    /// # EXAMPLE
    /// ```
    /// # use music_sheet_gen::notation::*;
    ///
    /// let b = Pitch::new(NoteName::C, Octave::O4, Some(Accidental::Flat));
    /// let cis = Pitch::new(NoteName::C, Octave::O4, Some(Accidental::Sharp));
    /// let des = Pitch::new(NoteName::D, Octave::O4, Some(Accidental::Flat));
    /// let c = Pitch::new(NoteName::B, Octave::O3, Some(Accidental::Sharp));
    /// let ais = Pitch::new(NoteName::A, Octave::O3, Some(Accidental::Sharp));
    ///
    /// assert_eq!(b.value_halftone(), 11);
    /// assert_eq!(cis.value_halftone(), 1);
    /// assert_eq!(cis.value_halftone(), des.value_halftone());
    /// assert_eq!(c.value_halftone(), 0);
    /// assert_eq!(ais.value_halftone(), 10);
    /// ```
    pub fn value_halftone(&self) -> u8 {
        // Add total number of halftones to get rid of negative values (case of C flat)
        let v = self.note_name().value_halftone() + Octave::halftone_count();

        let h = match self.accidental() {
            Some(Accidental::Sharp) => v + 1,
            Some(Accidental::Flat) => v - 1,
            None => v,
        };

        h % Octave::halftone_count()
    }

    /// Octave after applying accidentals
    ///
    /// # EXAMPLE
    /// ```
    /// # use music_sheet_gen::notation::*;
    ///
    /// let b = Pitch::new(NoteName::C, Octave::O4, Some(Accidental::Flat));
    /// let cis = Pitch::new(NoteName::C, Octave::O4, Some(Accidental::Sharp));
    /// let c = Pitch::new(NoteName::B, Octave::O3, Some(Accidental::Sharp));
    /// let ais = Pitch::new(NoteName::A, Octave::O3, Some(Accidental::Sharp));
    ///
    /// assert_eq!(b.real_octave(), Octave::O3);
    /// assert_eq!(cis.real_octave(), Octave::O4);
    /// assert_eq!(c.real_octave(), Octave::O4);
    /// assert_eq!(ais.real_octave(), Octave::O3);
    /// ```
    pub fn real_octave(&self) -> Octave {
        use Accidental::*;
        use NoteName::*;

        if self.note_name() == C && self.accidental() == Some(Flat) {
            self.octave.try_prev().unwrap()
        } else if self.note_name() == B && self.accidental() == Some(Sharp) {
            self.octave.try_next().unwrap()
        } else {
            self.octave
        }
    }

    /// Move the pitch by a halftone up
    ///
    /// # EXAMPLES
    /// ```
    /// # use music_sheet_gen::notation::*;
    ///
    /// let mut c = Pitch::new(NoteName::C, Octave::O4, None);
    /// let mut his = Pitch::new(NoteName::B, Octave::O3, Some(Accidental::Sharp));
    /// let mut fes = Pitch::new(NoteName::F, Octave::O4, Some(Accidental::Flat));
    ///
    /// c.move_halftone_up();
    /// his.move_halftone_up();
    /// fes.move_halftone_up();
    ///
    /// assert_eq!(c, Pitch::new(NoteName::C, Octave::O4, Some(Accidental::Sharp)));
    /// assert_eq!(his, Pitch::new(NoteName::C, Octave::O4, Some(Accidental::Sharp)));
    /// assert_eq!(fes, Pitch::new(NoteName::F, Octave::O4, None));
    /// ```
    pub fn move_halftone_up(&mut self) {
        match self.accidental() {
            Some(Accidental::Sharp) => {
                if self.note_name() == NoteName::B {
                    self.octave = self.octave.try_next().unwrap();
                } else if self.note_name() != NoteName::E {
                    self.ext.accidental = None;
                }
                self.ext.note_name = self.note_name().next();
            }
            Some(Accidental::Flat) => self.ext.accidental = None,
            None => self.ext.accidental = Some(Accidental::Sharp),
        }
    }

    /// Move the pitch by a halftone down
    ///
    /// # EXAMPLES
    /// ```
    /// # use music_sheet_gen::notation::*;
    ///
    /// let mut c = Pitch::new(NoteName::C, Octave::O4, None);
    /// let mut his = Pitch::new(NoteName::B, Octave::O3, Some(Accidental::Sharp));
    /// let mut ges = Pitch::new(NoteName::G, Octave::O4, Some(Accidental::Flat));
    ///
    /// c.move_halftone_down();
    /// his.move_halftone_down();
    /// ges.move_halftone_down();
    ///
    /// assert_eq!(c, Pitch::new(NoteName::B, Octave::O3, None));
    /// assert_eq!(his, Pitch::new(NoteName::B, Octave::O3, None));
    /// assert_eq!(ges, Pitch::new(NoteName::F, Octave::O4, None));
    /// ```
    pub fn move_halftone_down(&mut self) {
        match self.accidental() {
            Some(Accidental::Sharp) => self.ext.accidental = None,
            Some(Accidental::Flat) => {
                if self.note_name() == NoteName::C {
                    self.octave = self.octave.try_prev().unwrap();
                } else if self.note_name() != NoteName::F {
                    self.ext.accidental = None;
                }
                self.ext.note_name = self.note_name().prev();
            }
            None => self.ext.accidental = Some(Accidental::Flat),
        }
    }

    /// Move the pitch by a tone up
    pub fn move_tone_up(&mut self) {
        self.move_halftone_up();
        self.move_halftone_up();
    }

    /// Move the pitch by a tone down
    pub fn move_tone_down(&mut self) {
        self.move_halftone_down();
        self.move_halftone_down();
    }
}

impl PartialEq for Pitch {
    fn eq(&self, other: &Self) -> bool {
        self.real_octave() == other.real_octave() && self.value_halftone() == other.value_halftone()
    }
}

#[derive(Debug, Clone)]
pub struct Note {
    pub pitch: Pitch,
    pub duration: NoteLength,
}

/// Represents a chord of notes that have the same length.
#[derive(Debug, Clone)]
pub struct Chord {
    pub pitches: Vec<Pitch>,
    pub duration: NoteLength,
}

#[derive(Debug, Clone)]
pub enum Symbol {
    Clef(Clef),
    TimeSignature(TimeSignature),
    KeySignature(KeySignature),
    Chord(Chord),
    Note(Note),
    Rest(NoteLength),
    Tempo(Tempo),
}
