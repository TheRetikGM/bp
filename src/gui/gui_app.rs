//! Main GUI application definition
//!
//! **NOTE**: Uses eframe_template as a reference
//!
//! ### Author
//! Jakub Kloub (xkloub03), VUT FIT

use std::collections::HashMap;

use crate::{
    error::Result,
    gui::{toast, windows::*},
    lsystem::{interpret::MusicIntInfo, CSSLRuleSet, CSSLSystem},
};
use egui_dock::{DockArea, DockState, TabViewer};

#[derive(Debug)]
pub struct GuiAppState {
    pub rules: CSSLRuleSet,
    pub axiom: String,

    pub music_int_info: MusicIntInfo,

    pub l_system: CSSLSystem,
    pub dirty: bool,
}

impl GuiAppState {
    pub fn apply_changes(&mut self) -> Result<()> {
        self.l_system = CSSLSystem::new(self.axiom.clone(), self.rules.clone());

        Ok(())
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
struct GuiAppDocked {
    #[serde(skip)]
    app_state: GuiAppState,

    #[serde(skip)]
    pub tabs: HashMap<&'static str, Box<dyn DockableWindow>>,
}

impl Default for GuiAppDocked {
    fn default() -> Self {
        let rules = CSSLRuleSet::from_str_rules(&[
            "F -> F % 1/2",
            "F -> FF % 1/15",
            "F -> F+F % 1/15",
            "F -> F-F % 1/15",
            "FF -> [Fd+F-F] % 1/40",
            "FF -> [Fd-F+F] % 1/40",
            "FF -> [dF+F]F % 1/40",
            "FF -> [dF-F]F % 1/40",
            "F+F -> [Fd+F+F] % 1/40",
            "F-F -> [Fd-F-F] % 1/40",
            "F+F -> [dF+F]++F % 1/40",
            "F-F -> [dF-F]--F % 1/40",
            "F-F -> [Fd++F]--F % 1/40",
            "F-F -> [Fd-F]++F % 1/40",
            "F+F -> [Fd+++F--F] % 1/40",
            "F+F -> [Fd----F++F] % 1/40",
        ])
        .unwrap();
        let axiom = "F++++F--F++F".to_owned();

        let app_state = GuiAppState {
            l_system: CSSLSystem::new(axiom.clone(), rules.clone()),
            rules,
            axiom,
            dirty: true,
            music_int_info: MusicIntInfo::default(),
        };

        let tabs: Vec<Box<dyn DockableWindow>> = vec![
            Box::new(Logger {}),
            Box::new(GrammarEdit::new(&app_state)),
            Box::new(ScoreVisualizer::default()),
            Box::new(ControlPanel::new(&app_state)),
            Box::new(InterpretParameteres {}),
        ];

        Self {
            tabs: tabs.into_iter().map(|tab| (tab.name(), tab)).collect(),
            app_state,
        }
    }
}

type Tab = String;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct GuiApp {
    dock_state: DockState<Tab>,
    app_docked: GuiAppDocked,
}

impl Default for GuiApp {
    fn default() -> Self {
        let app_docked = GuiAppDocked::default();

        Self {
            dock_state: DockState::new(
                app_docked.tabs.iter().map(|(&n, _)| n.to_owned()).collect(),
            ),
            app_docked,
        }
    }
}

impl GuiApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for GuiApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }

                egui::widgets::global_theme_preference_buttons(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |_ui| {
            DockArea::new(&mut self.dock_state)
                .style(egui_dock::Style::from_egui(ctx.style().as_ref()))
                .show(ctx, &mut self.app_docked);

            toast::TOASTS.lock().unwrap().show(ctx);
        });
    }
}

impl TabViewer for GuiAppDocked {
    type Tab = Tab;

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        self.tabs[tab.as_str()].name().into()
    }

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        if let Some(dockable_window) = self.tabs.get_mut(tab.as_str()) {
            dockable_window.show(ui, &mut self.app_state);
        } else {
            panic!("Trying to draw unknown tab '{tab}'.")
        }
    }
}
