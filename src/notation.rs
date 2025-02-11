mod score;
mod symbol;
mod track;
pub mod visitor;

#[rustfmt::skip] #[derive(Debug)]
pub enum Clef { Treble, Bass }

#[rustfmt::skip] #[derive(Debug)]
pub enum KeySignatureType { Maj, Min }

#[rustfmt::skip] #[derive(Debug)]
pub enum NoteName { C, D, E, F, G, A, B }

#[rustfmt::skip] #[derive(Debug)]
pub enum Octave { O0, O1, O2, O3, O4, O5, O6, O7, O8, O9 }

#[rustfmt::skip] #[derive(Debug)]
pub enum Accidental { Sharp, Flat }

#[rustfmt::skip] #[derive(Debug)]
pub enum Duration { D1, D2, D4, D8, D16, D32, D64, D128 }

#[derive(Debug)]
pub struct KeySignature {
    note: NoteName,
    signature_type: KeySignatureType,
}

#[derive(Debug)]
pub struct TimeSignature {
    beat_count: u32,
    single_beat_note: Duration,
}
