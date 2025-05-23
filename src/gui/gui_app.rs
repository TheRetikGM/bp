//! Main GUI application definition
//!
//! **NOTE**: Uses eframe_template as a reference
//!
//! ### Author
//! Jakub Kloub (xkloub03), VUT FIT

use std::{
    collections::HashMap,
    ffi::OsStr,
    fs::File,
    path::{Path, PathBuf},
    rc::Rc,
};

use crate::{
    error::Result,
    gui::{toast, windows::*},
    lsystem::{interpret::MusicIntInfo, CSSLRuleSet, CSSLSystem},
    sanitizer::LilySanitizer,
};
use egui_dock::{DockArea, DockState, TabViewer};
use egui_file_dialog::FileDialog;

/// Holds the shared state of GUI application, which is passed
/// between the windows.
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct GuiAppState {
    /// Currently edited ruleset - not applied
    pub rules: CSSLRuleSet,
    /// Currently edited axiom
    pub axiom: String,
    /// Currently edited interpreter parameters
    pub music_int_info: MusicIntInfo,
    /// Currently edited lily sanitizer (max line bars etc.)
    pub lily_sanitizer: LilySanitizer,

    /// Currently used L-system for generation
    pub l_system: CSSLSystem,
    /// Used rules in all iterations.
    pub used_rules_history: Vec<Vec<Rc<crate::lsystem::CSSLRule>>>,

    /// Currently displayed score image path.
    pub score_images: Option<Vec<PathBuf>>,
    /// Currently played audio path.
    pub score_audio: Option<PathBuf>,

    /// Flag used to trigger repaint and re-interpration.
    #[serde(skip)]
    #[serde(default = "default_dirty")]
    pub dirty: bool,
}

fn default_dirty() -> bool {
    true
}

impl GuiAppState {
    /// Reset the currently used L-system
    pub fn reset(&mut self) {
        self.l_system = CSSLSystem::new(self.axiom.clone(), self.rules.clone());
        self.used_rules_history.clear();
        self.score_images = None;
        self.score_audio = None;
        self.dirty = true;
    }

    /// Apply changes to L-system.
    pub fn apply_changes(&mut self) -> Result<()> {
        self.l_system = CSSLSystem::new(self.axiom.clone(), self.rules.clone());

        Ok(())
    }
}

impl Default for GuiAppState {
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

        Self {
            l_system: CSSLSystem::new(axiom.clone(), rules.clone()),
            rules,
            axiom,
            dirty: true,
            music_int_info: MusicIntInfo::default(),
            used_rules_history: Vec::default(),
            score_images: None,
            score_audio: None,
            lily_sanitizer: LilySanitizer::default(),
        }
    }
}

/// Struct managing the docked windows. Acts as a TabViewer for egui-dock
/// and passes the GuiAppState to them.
#[derive(serde::Serialize)]
#[serde(default)]
struct GuiAppDocked {
    app_state: GuiAppState,

    #[serde(skip)]
    pub tabs: HashMap<&'static str, Box<dyn DockableWindow>>,
}

/// Helper struct for deserialization
#[derive(serde::Deserialize)]
struct GuiAppDockedDe {
    app_state: GuiAppState,
}

/// Create all tabs with given app_state.
fn create_tabs(app_state: &GuiAppState) -> HashMap<&'static str, Box<dyn DockableWindow>> {
    let tabs: Vec<Box<dyn DockableWindow>> = vec![
        Box::new(Logger {}),
        Box::new(GrammarEdit::new(app_state)),
        Box::new(ScoreVisualizer::default()),
        Box::new(ControlPanel::new(app_state)),
        Box::new(InterpretParameteres {}),
        Box::new(Statistics {}),
    ];

    tabs.into_iter().map(|tab| (tab.name(), tab)).collect()
}

/// Deserialize GuiAppDocked. This is customized so that the docked windows are created
/// with the deserialized GuiAppDocked instead of the default and thus load their
/// saved state correctly.
impl<'de> serde::Deserialize<'de> for GuiAppDocked {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let app_state = GuiAppDockedDe::deserialize(deserializer)?;

        Ok(Self {
            tabs: create_tabs(&app_state.app_state),
            app_state: app_state.app_state,
        })
    }
}

impl Default for GuiAppDocked {
    fn default() -> Self {
        let app_state = GuiAppState::default();

        Self {
            tabs: create_tabs(&app_state),
            app_state,
        }
    }
}

/// Tab type used for TabViewer is the DockableWindow::name(), which is a string.
type Tab = String;

/// Main application used by EFrame. Holds all the egui relevant stuff along with the state.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct GuiApp {
    /// egui-dock state
    dock_state: DockState<Tab>,
    /// Manager of docked windows and state.
    app_docked: GuiAppDocked,

    #[serde(skip)]
    file_dialog: FileDialog,
}

impl Default for GuiApp {
    fn default() -> Self {
        let app_docked = GuiAppDocked::default();
        let dock_state =
            DockState::new(app_docked.tabs.iter().map(|(&n, _)| n.to_owned()).collect());

        Self {
            dock_state,
            app_docked,
            file_dialog: FileDialog::new(),
        }
    }
}

impl GuiApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Load previous app state (if any).
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }

    /// Export current score to TAR archive to the given path.
    pub fn export(&self, path: &Path) -> crate::error::Result<()> {
        let path = if path.extension().unwrap_or(OsStr::new("")) != "tar" {
            path.with_added_extension("tar")
        } else {
            path.to_path_buf()
        };

        let file = File::create(path)?;
        let mut builder = tar::Builder::new(file);
        for p in self
            .app_docked
            .app_state
            .score_images
            .as_ref()
            .unwrap_or(&vec![])
        {
            builder.append_path_with_name(p, p.file_name().unwrap())?;
        }

        if let Some(audio_path) = self.app_docked.app_state.score_audio.as_ref() {
            builder.append_path_with_name(audio_path, audio_path.file_name().unwrap())?;
        }

        Ok(())
    }
}

impl eframe::App for GuiApp {
    /// Called by the framework to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Export..").clicked() {
                        self.file_dialog.save_file();
                    };
                    ui.separator();
                    if ui.button("Quit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
                ui.add_space(16.0);

                egui::widgets::global_theme_preference_buttons(ui);
            });
        });

        // Main content
        egui::CentralPanel::default().show(ctx, |_ui| {
            // Show all dockable windows.
            DockArea::new(&mut self.dock_state)
                .style(egui_dock::Style::from_egui(ctx.style().as_ref()))
                .show_close_buttons(false)
                .show_leaf_close_all_buttons(false)
                .show(ctx, &mut self.app_docked);

            // Show all toasts.
            toast::TOASTS.lock().show(ctx);

            // Show active dialog if any.
            self.file_dialog.update(ctx);
            if let Some(path) = self.file_dialog.take_picked() {
                match self.export(&path) {
                    Ok(_) => {
                        toast::show_success(
                            format!("File exported to: {}", path.display()).as_str(),
                        );
                    }
                    Err(e) => {
                        toast::show_error(format!("Export failed: {e}").as_str());
                    }
                };
            }
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
