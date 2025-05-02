//! Score visualizer window definition
//!
//! ### Author
//! Jakub Kloub (xkloub03), VUT FIT

use crate::{
    error::{AppError, Result},
    gui::{
        gui_app::GuiAppState,
        toast,
        utils::{self},
        widgets::AudioPlayer,
        windows::DockableWindow,
    },
    lily::Lilypond,
    lsystem::{
        interpret::{Interpret, MusicInterpret},
        LSystem,
    },
    utils::AudioController,
    Arguments,
};
use egui::Widget;
use std::{path::PathBuf, sync::Arc};

pub struct Texture(Arc<egui::ColorImage>, egui::TextureHandle);

impl std::fmt::Debug for Texture {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Texture").field(&self.0).finish()
    }
}

impl Texture {
    pub fn load_from(path: &PathBuf, ctx: &egui::Context) -> Result<Self> {
        let image = image::open(path).map_err(|e| AppError::build_path(path, &e))?;
        let size = [image.width() as usize, image.height() as usize];
        let image_buffer = image.to_rgba8();
        let pixels: Vec<_> = image_buffer
            .pixels()
            .map(|p| egui::Color32::from_rgba_unmultiplied(p[0], p[1], p[2], p[3]))
            .collect();

        let color_image = Arc::new(egui::ColorImage { size, pixels });
        let handle = ctx.load_texture("score_image", color_image.clone(), Default::default());

        Ok(Self(color_image, handle))
    }
}

pub struct ScoreVisualizer {
    pub image: Option<Texture>,
    last_step_num: i32,
    audio_controller: AudioController,
    sf_path: PathBuf,
}

impl std::fmt::Debug for ScoreVisualizer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ScoreVisualizer")
            .field("image", &self.image)
            .field("last_step_num", &self.last_step_num)
            .field("audio_controller", &self.audio_controller)
            .finish()
    }
}

impl Default for ScoreVisualizer {
    fn default() -> Self {
        let args = Arguments::new().unwrap();
        Self {
            image: Default::default(),
            last_step_num: Default::default(),
            audio_controller: AudioController::new().unwrap(),
            sf_path: args.sound_font_path,
        }
    }
}

impl ScoreVisualizer {
    fn refresh(&mut self, ui: &mut egui::Ui, app_state: &mut GuiAppState) -> Result<()> {
        let state = app_state.l_system.state();
        self.last_step_num = *state.iter_num();
        app_state.dirty = false;

        // Create the score and translate it.
        let score = MusicInterpret::new(app_state.music_int_info.clone())
            .translate(state.word())
            .sanitized()?;

        // Execute lilypond on the generated score.
        let lily_score: Lilypond = score.into();
        let output = crate::gui::utils::lilypond(lily_score.to_string().as_str(), "score")?;

        // Load generated sheet music.
        match Texture::load_from(output.path(), ui.ctx()) {
            Ok(tex) => self.image = Some(tex),
            Err(e) => {
                self.image = None;
                Err(e)?;
            }
        };

        // Create WAV file from generated midi.
        let fluidsynth_output = utils::fluidsynth(&self.sf_path, output.midi_path(), "score")?;

        // Load the result WAV file.
        self.audio_controller.load(fluidsynth_output.wav_path())?;

        Ok(())
    }
}

impl DockableWindow for ScoreVisualizer {
    fn name(&self) -> &'static str {
        "Score visualizer"
    }

    fn show(&mut self, ui: &mut egui::Ui, app_state: &mut GuiAppState) {
        let last_step_num = *app_state.l_system.state().iter_num();
        if app_state.dirty || last_step_num != self.last_step_num {
            if let Err(err) = self.refresh(ui, app_state) {
                toast::show_error(err.as_ref());
                log::error!("{err}")
            }
        }

        if self.audio_controller.is_loaded() {
            ui.add(AudioPlayer::new(&mut self.audio_controller));
        }

        ui.group(|ui| {
            if let Some(Texture(_, tex)) = &self.image {
                ui.centered_and_justified(|ui| {
                    egui::Image::new(tex).shrink_to_fit().ui(ui);
                });
            } else {
                ui.centered_and_justified(|ui| {
                    ui.label("no image");
                });
                ui.allocate_space(ui.available_size());
            }
        });
    }
}
