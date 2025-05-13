//! Score visualizer window definition
//!
//! ### Author
//! Jakub Kloub (xkloub03), VUT FIT

use crate::{
    error::Result,
    gui::{
        gui_app::GuiAppState,
        toast,
        utils::{
            fluidsynth_async, lilypond_async, AsyncResult, InMemoryTexture, Texture, ToAsyncResult,
        },
        widgets::AudioPlayer,
        windows::DockableWindow,
        TabType,
    },
    lily::Lilypond,
    lsystem::{
        interpret::{Interpret, MusicInterpret},
        LSystem,
    },
    sanitizer::LilySanitizer,
    utils::{AudioController, AudioData},
    Arguments,
};
use egui::Widget;
use parking_lot::Mutex;
use poll_promise::Promise;
use std::{
    cmp::{max, min},
    ops::DerefMut,
    path::PathBuf,
    sync::Arc,
};

#[derive(PartialEq, Debug)]
enum ScoreRefreshState {
    Begin,
    LilyCompilation,
    TextureLoading,
    Fluidsynth,
    AudioLoading,
    Done,
}

impl std::fmt::Display for ScoreRefreshState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ScoreRefreshState::Begin => "",
                ScoreRefreshState::LilyCompilation => "Lilypond compilation",
                ScoreRefreshState::TextureLoading => "Loading generated pages",
                ScoreRefreshState::Fluidsynth => "Converting MIDI to audio",
                ScoreRefreshState::AudioLoading => "Loading audio",
                ScoreRefreshState::Done => "Done",
            }
        )
    }
}

struct RefreshOutput {
    pages_data: Vec<InMemoryTexture>,
    audio_data: AudioData,
    score_image_paths: Vec<PathBuf>,
    score_audio_path: PathBuf,
}

fn refresh_async(
    async_state: Arc<Mutex<ScoreRefreshState>>,
    lily_input: String,
    sf_path: PathBuf,
) -> Promise<AsyncResult<RefreshOutput>> {
    poll_promise::Promise::spawn_thread("refresh_async", move || {
        *async_state.lock().deref_mut() = ScoreRefreshState::LilyCompilation;
        let lily_output = lilypond_async(lily_input, "score".to_owned()).block_and_take()?;

        *async_state.lock().deref_mut() = ScoreRefreshState::TextureLoading;
        let mut textures: Vec<InMemoryTexture> = vec![];
        for t_res in lily_output.pages().iter().map(InMemoryTexture::load_from) {
            textures.push(t_res.into_async_result()?);
        }

        *async_state.lock().deref_mut() = ScoreRefreshState::Fluidsynth;
        let fluid_output =
            fluidsynth_async(&sf_path, lily_output.midi_path(), "score").block_and_take()?;

        *async_state.lock().deref_mut() = ScoreRefreshState::AudioLoading;
        let audio_data = AudioData::load_from(fluid_output.wav_path()).into_async_result()?;

        *async_state.lock().deref_mut() = ScoreRefreshState::Done;

        AsyncResult::Ok(RefreshOutput {
            pages_data: textures,
            audio_data,
            score_image_paths: lily_output.pages().to_owned(),
            score_audio_path: fluid_output.wav_path().to_owned(),
        })
    })
}

pub struct ScoreVisualizer {
    pub images: Option<Vec<Texture>>,
    last_step_num: i32,
    audio_controller: AudioController,
    sf_path: PathBuf,

    selected_image: Option<usize>,
    refresh_state: Option<Arc<Mutex<ScoreRefreshState>>>,
    refresh_promise: Option<Promise<AsyncResult<RefreshOutput>>>,
}

impl std::fmt::Debug for ScoreVisualizer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ScoreVisualizer")
            .field("images", &self.images)
            .field("last_step_num", &self.last_step_num)
            .field("audio_controller", &self.audio_controller)
            .field("sf_path", &self.sf_path)
            .finish()
    }
}

impl Default for ScoreVisualizer {
    fn default() -> Self {
        let args = Arguments::new().unwrap();
        Self {
            images: None,
            selected_image: None,
            last_step_num: Default::default(),
            audio_controller: AudioController::new().unwrap(),
            sf_path: args.sound_font_path,
            refresh_state: None,
            refresh_promise: None,
        }
    }
}

impl ScoreVisualizer {
    fn start_refresh(&mut self, app_state: &mut GuiAppState) -> Result<()> {
        let state = app_state.l_system.state();
        self.last_step_num = *state.iter_num();
        app_state.dirty = false;

        // Create the score and translate it.
        let score = MusicInterpret::new(app_state.music_int_info.clone())
            .translate(state.word())
            .sanitized()?;
        let lily_score = Lilypond::from(score).sanitized_with(LilySanitizer::default())?;

        self.images = None;
        self.selected_image = None;
        self.refresh_state = Some(Arc::new(Mutex::new(ScoreRefreshState::Begin)));
        self.refresh_promise = Some(refresh_async(
            self.refresh_state.clone().unwrap(),
            lily_score.to_string(),
            self.sf_path.clone(),
        ));

        Ok(())
    }
}

impl DockableWindow for ScoreVisualizer {
    fn name(&self) -> &'static str {
        TabType::ScoreVisualizer.as_str()
    }

    fn show(&mut self, ui: &mut egui::Ui, app_state: &mut GuiAppState) {
        let last_step_num = *app_state.l_system.state().iter_num();
        if app_state.dirty || last_step_num != self.last_step_num {
            if let Err(e) = self.start_refresh(app_state) {
                toast::show_error(format!("Start refresh failed: {e}").as_str());
            }
        }

        if self.audio_controller.is_loaded() {
            ui.add(AudioPlayer::new(&mut self.audio_controller));
        }

        if let Some(refresh_promise) = self.refresh_promise.take() {
            match refresh_promise.try_take() {
                Ok(refresh_output) => match refresh_output {
                    AsyncResult::Ok(refresh_output) => {
                        self.images = Some(
                            refresh_output
                                .pages_data
                                .into_iter()
                                .map(|data| Texture::load_from_memory(data, ui.ctx()))
                                .collect(),
                        );
                        self.selected_image = Some(0);
                        self.audio_controller
                            .load_from_data(refresh_output.audio_data)
                            .unwrap();
                        app_state.score_images = Some(refresh_output.score_image_paths);
                        app_state.score_audio = Some(refresh_output.score_audio_path);
                        ui.ctx().request_repaint();
                    }
                    AsyncResult::Err(e) => {
                        toast::show_error(format!("Refresh failed: {e}").as_str());
                    }
                },
                Err(promise) => {
                    ui.centered_and_justified(|ui| {
                        egui::Frame::new().show(ui, |ui| {
                            ui.set_width(100.0);
                            ui.horizontal(|ui| {
                                ui.add(egui::Spinner::new().size(10.0));
                                ui.label(self.refresh_state.as_ref().unwrap().lock().to_string());
                            });
                        });
                    });

                    self.refresh_promise = Some(promise);
                }
            };
        } else if let (Some(images), Some(selected_image)) = (&self.images, self.selected_image) {
            ui.group(|ui| {
                ui.vertical_centered(|ui| {
                    ui.set_height(ui.available_height() - 28.0);
                    let Texture(_, tex) = &images[selected_image];
                    egui::Image::new(tex).shrink_to_fit().ui(ui);
                });
                ui.add_space(5.0);
                ui.vertical_centered(|ui| {
                    ui.set_width(100.0);

                    ui.horizontal(|ui| {
                        if ui.button("<").clicked() {
                            self.selected_image = Some(max(0, selected_image as i32 - 1) as usize);
                            ui.ctx().request_repaint();
                        }

                        ui.label(format!("{} / {}", selected_image + 1, images.len()));

                        if ui.button(">").clicked() {
                            self.selected_image = Some(min(images.len() - 1, selected_image + 1));
                            ui.ctx().request_repaint();
                        }
                    })
                })
            });
        } else {
            ui.group(|ui| {
                ui.centered_and_justified(|ui| {
                    ui.label("no image");
                });
                ui.allocate_space(ui.available_size());
            });
        }
    }
}
