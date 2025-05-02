//! Widgets module
//!
//! ### Author
//! Jakub Kloub (xkloub03), VUT FIT

mod acc_selector;
mod audio_player;
mod axiom_edit;
mod key_edit;
mod length_selector;
mod note_edit;
mod note_name_selector;
mod octave_selector;
mod rule_edit;
mod rule_sums;

pub use acc_selector::AccSelector;
pub use audio_player::AudioPlayer;
pub use axiom_edit::AxiomEdit;
pub use key_edit::KeyEdit;
pub use length_selector::LengthSelector;
pub use note_edit::NoteEdit;
pub use note_name_selector::NoteNameSelector;
pub use octave_selector::OctaveSelector;
pub use rule_edit::RuleEdit;
pub use rule_edit::RuleEditState;
pub use rule_sums::RuleSums;

const RULE_EPS: f32 = 0.001;
