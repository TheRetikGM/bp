//! Various utils used in GUI
//!
//! ### Author
//! Jakub Kloub (xkloub03), VUT FIT

mod texture;
use once_cell::sync::Lazy;
use poll_promise::Promise;
use regex::Regex;
pub use texture::InMemoryTexture;
pub use texture::Texture;

use derive_getters::Getters;

use crate::{
    error::{AppError, Result},
    ext::CapturedStr,
    gui::DIR_NAME,
};
use std::{
    cmp::Ordering,
    fs::File,
    io::Write,
    path::{Path, PathBuf},
    process::Command,
};

#[derive(Debug, Clone, Getters)]
pub struct LilyOutput {
    pages: Vec<PathBuf>,
    preview_path: PathBuf,
    midi_path: PathBuf,
}

pub fn lilypond(lily_str: &str, filename: &str) -> Result<LilyOutput> {
    let cache_dir = directories::BaseDirs::new()
        .ok_or(AppError::Other("Failed to create BaseDirs".to_string()))?
        .cache_dir()
        .join(DIR_NAME);

    if cache_dir.exists() {
        std::fs::remove_dir_all(&cache_dir)?;
    }
    std::fs::create_dir_all(&cache_dir)?;

    let input_path = cache_dir.join("input.ily");
    let output_preview_path = cache_dir.join(format!("{filename}.preview.png"));
    let output_midi_path = cache_dir.join(format!("{filename}.midi"));

    let mut input_file =
        File::create(&input_path).map_err(|e| AppError::build_path(&input_path, &e))?;
    input_file.write_all(lily_str.as_bytes())?;

    let mut cmd = Command::new("lilypond");
    cmd.current_dir(&cache_dir).args([
        "--png",
        "-dpreview",
        "-o",
        filename,
        input_path.display().to_string().as_str(),
    ]);
    let output = cmd.output()?;

    if !output.status.success() {
        Err(AppError::Lily(format!(
            "Exit code: {0}, Stderr: \n{1}",
            output.status.code().unwrap(),
            String::from_utf8(output.stderr).unwrap()
        )))?;
    }

    let mut pages: Vec<PathBuf> = std::fs::read_dir(&cache_dir)?
        .filter_map(|entry| {
            let path = entry.ok()?.path();
            let fname = path.file_name()?.to_str()?;
            if fname.starts_with(filename) && fname.ends_with(".png") && !fname.contains("preview")
            {
                Some(path)
            } else {
                None
            }
        })
        .collect();

    pages.sort_by(page_num_asc);

    if !output_midi_path.exists() || !output_preview_path.exists() || pages.is_empty() {
        Err(AppError::Lily("Not all files were generated.".to_string()))?;
    }

    Ok(LilyOutput {
        pages,
        preview_path: output_preview_path,
        midi_path: output_midi_path,
    })
}

#[allow(clippy::ptr_arg)]
fn page_num_asc(a: &PathBuf, b: &PathBuf) -> Ordering {
    static PAGE_REG: Lazy<Regex> = Lazy::new(|| Regex::new(r"^.*?(\d+)\.png$").unwrap());

    let af = a.file_name().unwrap().to_str().unwrap();
    let bf = b.file_name().unwrap().to_str().unwrap();

    if !af.contains("page") {
        return Ordering::Less;
    }

    if !bf.contains("page") {
        return Ordering::Greater;
    }

    match (PAGE_REG.captures(af), PAGE_REG.captures(bf)) {
        (Some(ac), Some(bc)) => {
            let pa: i32 = ac.captured_str(1).unwrap().parse().unwrap();
            let pb: i32 = bc.captured_str(1).unwrap().parse().unwrap();
            pa.cmp(&pb)
        }
        _ => Ordering::Equal,
    }
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

pub fn section_name(ui: &mut egui::Ui, name: &str) {
    ui.label(
        egui::RichText::new(name)
            .strong()
            .font(egui::FontId::proportional(16.0)),
    );
    ui.end_row();
}

#[derive(Debug, Clone)]
pub enum AsyncResult<T> {
    Ok(T),
    Err(String),
}

impl<T, R: ToString> From<std::result::Result<T, R>> for AsyncResult<T> {
    fn from(value: std::result::Result<T, R>) -> Self {
        match value {
            Ok(v) => AsyncResult::Ok(v),
            Err(e) => AsyncResult::Err(e.to_string()),
        }
    }
}

impl<T> std::ops::FromResidual for AsyncResult<T> {
    fn from_residual(residual: AsyncResult<core::convert::Infallible>) -> Self {
        match residual {
            AsyncResult::Err(e) => AsyncResult::Err(e),
            _ => unreachable!(),
        }
    }
}

impl<T> std::ops::Try for AsyncResult<T> {
    type Output = T;
    type Residual = AsyncResult<core::convert::Infallible>;

    fn from_output(output: Self::Output) -> Self {
        AsyncResult::Ok(output)
    }

    fn branch(self) -> std::ops::ControlFlow<Self::Residual, Self::Output> {
        match self {
            AsyncResult::Ok(v) => std::ops::ControlFlow::Continue(v),
            AsyncResult::Err(e) => std::ops::ControlFlow::Break(AsyncResult::Err(e)),
        }
    }
}

pub trait ToAsyncResult<T> {
    fn into_async_result(self) -> AsyncResult<T>;
}

impl<T, R: ToString> ToAsyncResult<T> for std::result::Result<T, R> {
    fn into_async_result(self) -> AsyncResult<T> {
        self.into()
    }
}

pub fn lilypond_async(lily_str: String, filename: String) -> Promise<AsyncResult<LilyOutput>> {
    poll_promise::Promise::spawn_thread("lilypond_command", move || {
        lilypond(lily_str.as_ref(), filename.as_ref()).into()
    })
}

pub fn fluidsynth_async(
    sf_path: &impl AsRef<Path>,
    midi_path: &impl AsRef<Path>,
    filename: &str,
) -> Promise<AsyncResult<FluidsynthOutput>> {
    let sf_path = sf_path.as_ref().to_path_buf();
    let midi_path = midi_path.as_ref().to_path_buf();
    let filename = filename.to_owned();

    poll_promise::Promise::spawn_thread("lilypond_command", move || {
        fluidsynth(sf_path, midi_path, filename.as_ref()).into()
    })
}
