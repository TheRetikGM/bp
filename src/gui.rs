//! Gui module
//!
//! ### Author
//! Jakub Kloub (xkloub03), VUT FIT

mod gui_app;

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
    use std::{sync::Mutex, time::Duration};

    pub static TOASTS: Lazy<Mutex<Toasts>> = Lazy::new(|| Mutex::new(Toasts::default()));

    pub static DURATION: u64 = 3;

    pub fn show_success(message: &str) {
        let mut toasts = TOASTS.lock().unwrap();
        toasts
            .success(message)
            .duration(Some(Duration::from_secs(DURATION)));
    }

    pub fn show_error(message: &str) {
        let mut toasts = TOASTS.lock().unwrap();
        toasts
            .error(message)
            .duration(Some(Duration::from_secs(DURATION)));
    }

    pub fn show_info(message: &str) {
        let mut toasts = TOASTS.lock().unwrap();
        toasts
            .info(message)
            .duration(Some(Duration::from_secs(DURATION)));
    }

    pub fn show_warn(message: &str) {
        let mut toasts = TOASTS.lock().unwrap();
        toasts
            .warning(message)
            .duration(Some(Duration::from_secs(DURATION)));
    }
}
