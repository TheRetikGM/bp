//! Windows module
//!
//! ### Author
//! Jakub Kloub (xkloub03), VUT FIT

mod control_panel;
mod grammar_edit;
mod interpret_parameters;
mod logger;
mod score_visualizer;
mod statistics;

pub use control_panel::ControlPanel;
pub use grammar_edit::GrammarEdit;
pub use interpret_parameters::InterpretParameteres;
pub use logger::Logger;
pub use score_visualizer::ScoreVisualizer;
pub use statistics::Statistics;

use crate::gui::gui_app::GuiAppState;

pub trait DockableWindow {
    fn name(&self) -> &'static str;
    fn show(&mut self, ui: &mut egui::Ui, app_state: &mut GuiAppState);
}
