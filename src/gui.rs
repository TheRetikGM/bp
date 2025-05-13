//! Gui module
//!
//! ### Author
//! Jakub Kloub (xkloub03), VUT FIT

mod gui_app;

pub mod utils;
pub mod widgets;
pub mod windows;

pub use gui_app::GuiApp;
pub use gui_app::GuiAppState;

pub trait View {
    fn ui(&mut self, ui: &mut egui::Ui, app_state: &mut GuiAppState);
}

pub mod toast {
    use egui_notify::Toasts;
    use once_cell::sync::Lazy;
    use parking_lot::Mutex;
    use std::{fmt::Display, time::Duration};

    pub static TOASTS: Lazy<Mutex<Toasts>> = Lazy::new(|| Mutex::new(Toasts::default()));

    pub static DURATION: u64 = 3;

    pub fn show_success<D: Display + ?Sized>(message: &D) {
        let mut toasts = TOASTS.lock();
        toasts
            .success(format!("{message}").as_str())
            .duration(Some(Duration::from_secs(DURATION)));

        log::info!("{message}")
    }

    pub fn show_error<D: Display + ?Sized>(message: &D) {
        let mut toasts = TOASTS.lock();
        toasts
            .error(format!("{message}").as_str())
            .duration(Some(Duration::from_secs(DURATION)));

        log::error!("{message}");
    }

    pub fn show_info(message: &impl Display) {
        let mut toasts = TOASTS.lock();
        toasts
            .info(format!("{message}").as_str())
            .duration(Some(Duration::from_secs(DURATION)));

        log::info!("{message}");
    }

    pub fn show_warn(message: &impl Display) {
        let mut toasts = TOASTS.lock();
        toasts
            .warning(format!("{message}").as_str())
            .duration(Some(Duration::from_secs(DURATION)));

        log::warn!("{message}");
    }
}

pub const DIR_NAME: &str = "music_sheet_gen";
