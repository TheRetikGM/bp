mod scale;

use crate::notation::{Clef, KeySignature, Note, Score, Stave, Symbol, TimeSignature};
use scale::Scale;

pub trait Interpret<T> {
    fn translate(&self, string: &str) -> T;
}

#[derive(Debug, Clone)]
pub struct MusicInterpret {
    int_info: MusicIntInfo,
}

/// Music interpretation info.
///
/// This structure holds initial information needed for translation
/// of L-System string.
///
/// # TODO
/// - Add tact information such as timing, name of the score, author tempo..
#[derive(Debug, Clone)]
pub struct MusicIntInfo {
    pub key: KeySignature,
    pub first_note: Note,
}

#[derive(Debug, Clone)]
struct Context {
    /// Current note that is to be written to the score
    pub note: Note,
    /// Scale, in which the resulting score will be.
    pub scale: Scale,
    /// The resulting stave that we will put into the score.
    pub stave_notes: Vec<Note>,
    /// The stack of notes used for saving state.
    pub stack: Vec<Note>,
}

impl Interpret<Score> for MusicInterpret {
    fn translate(&self, string: &str) -> Score {
        let mut context = Context {
            scale: Scale::new(self.int_info.key),
            note: self.int_info.first_note.clone(),
            stave_notes: Default::default(),
            stack: Default::default(),
        };

        string.chars().for_each(|c| self.action(&mut context, c));

        Score {
            staves: vec![Stave {
                symbols: [
                    Symbol::Clef(Clef::Treble),
                    Symbol::KeySignature(self.int_info.key),
                    Symbol::TimeSignature(TimeSignature::c()),
                ]
                .into_iter()
                .chain(context.stave_notes.into_iter().map(Symbol::Note))
                .collect(),
            }],
            ..Default::default()
        }
    }
}

impl MusicInterpret {
    pub fn new(int_info: MusicIntInfo) -> Self {
        Self { int_info }
    }

    fn action(&self, context: &mut Context, symbol: char) {
        match symbol {
            // Write the current note into the score.
            'F' => context.stave_notes.push(context.note.clone()),
            // Change the current note to the next one in scale.
            '+' => context.scale.advance(&mut context.note.pitch),
            // Change the current note to the previous one in scale.
            '-' => context.scale.recede(&mut context.note.pitch),
            // Half the length of current note.
            // FIXME: Handle the case where duration can no longer be halved.
            //        What do we do then?
            'd' => context.note.duration.halve(),
            // Save current state onto the stack.
            '[' => context.stack.push(context.note.clone()),
            // Pop current state from the stack.
            ']' => context.note = context.stack.pop().unwrap(),
            s => panic!("Invalid symbol: '{:?}'", s),
        }
    }
}
