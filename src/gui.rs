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
    use std::{fmt::Display, sync::Mutex, time::Duration};

    pub static TOASTS: Lazy<Mutex<Toasts>> = Lazy::new(|| Mutex::new(Toasts::default()));

    pub static DURATION: u64 = 3;

    pub fn show_success<D: Display + ?Sized>(message: &D) {
        let mut toasts = TOASTS.lock().unwrap();
        toasts
            .success(format!("{message}").as_str())
            .duration(Some(Duration::from_secs(DURATION)));
    }

    pub fn show_error<D: Display + ?Sized>(message: &D) {
        let mut toasts = TOASTS.lock().unwrap();
        toasts
            .error(format!("{message}").as_str())
            .duration(Some(Duration::from_secs(DURATION)));
    }

    pub fn show_info(message: &impl Display) {
        let mut toasts = TOASTS.lock().unwrap();
        toasts
            .info(format!("{message}").as_str())
            .duration(Some(Duration::from_secs(DURATION)));
    }

    pub fn show_warn(message: &impl Display) {
        let mut toasts = TOASTS.lock().unwrap();
        toasts
            .warning(format!("{message}").as_str())
            .duration(Some(Duration::from_secs(DURATION)));
    }
}

pub const DIR_NAME: &str = "music_sheet_gen";

pub mod utils {
    use derive_getters::Getters;

    use crate::{
        error::{AppError, Result},
        gui::DIR_NAME,
    };
    use std::{
        fs::File,
        io::Write,
        path::{Path, PathBuf},
        process::Command,
    };

    #[derive(Debug, Clone, Getters)]
    pub struct LilyOutput {
        path: PathBuf,
        preview_path: PathBuf,
        midi_path: PathBuf,
    }

    pub fn lilypond(lily_str: &str, filename: &str) -> Result<LilyOutput> {
        let cache_dir = directories::BaseDirs::new()
            .ok_or(AppError::Other("Failed to create BaseDirs".to_string()))?
            .cache_dir()
            .join(DIR_NAME);

        std::fs::create_dir_all(&cache_dir)?;

        let input_path = cache_dir.join("input.ily");
        let output_path = cache_dir.join(format!("{filename}.png"));
        let output_preview_path = cache_dir.join(format!("{filename}.preview.png"));
        let output_midi_path = cache_dir.join(format!("{filename}.midi"));

        let mut input_file =
            File::create(&input_path).map_err(|e| AppError::build_path(&input_path, &e))?;
        input_file.write_all(lily_str.as_bytes())?;

        let mut cmd = Command::new("lilypond");
        cmd.current_dir(cache_dir).args([
            "--png",
            "-dpreview",
            "-o",
            filename,
            input_path.display().to_string().as_str(),
        ]);
        let output = cmd.output()?;

        if !output.status.success() {
            Err(AppError::Lily(format!(
                "Exit code: {0}, Stdout: \n{1}",
                output.status.code().unwrap(),
                String::from_utf8(output.stdout).unwrap()
            )))?;
        }

        if !output_path.exists() || !output_preview_path.exists() {
            Err(AppError::Lily("Not all files were generated.".to_string()))?;
        }

        Ok(LilyOutput {
            path: output_path,
            preview_path: output_preview_path,
            midi_path: output_midi_path,
        })
    }

    #[derive(Debug, Clone, Getters)]
    pub struct FluidsynthOutput {
        wav_path: PathBuf,
    }

    pub fn fluidsynth(
        sf_path: impl AsRef<Path>,
        midi_path: impl AsRef<Path>,
        filename: &str,
    ) -> Result<FluidsynthOutput> {
        let cache_dir = directories::BaseDirs::new()
            .ok_or(AppError::Other("Failed to create BaseDirs".to_string()))?
            .cache_dir()
            .join(DIR_NAME);

        std::fs::create_dir_all(&cache_dir)?;

        let filename_wav = format!("{filename}.wav");
        let wav_path = cache_dir.join(&filename_wav);

        let output = Command::new("fluidsynth")
            .current_dir(cache_dir)
            .args([
                "-ni",
                sf_path.as_ref().to_string_lossy().to_string().as_str(),
                midi_path.as_ref().to_string_lossy().to_string().as_str(),
                "-F",
                filename_wav.as_str(),
                "-r",
                "44100",
            ])
            .output()?;

        if !output.status.success() {
            Err(AppError::Lily(format!(
                "Exit code: {0}, Stdout: \n{1}",
                output.status.code().unwrap(),
                String::from_utf8(output.stdout).unwrap()
            )))?;
        }

        if !wav_path.exists() {
            Err(AppError::Fluidsynth(
                "WAV file was not generated.".to_string(),
            ))?;
        }

        Ok(FluidsynthOutput { wav_path })
    }
}
