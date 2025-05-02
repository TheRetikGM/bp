//! Audio controller util definition
//!
//! ### Author
//! Jakub Kloub (xkloub03), VUT FIT

use crate::error::*;
use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink, Source};
use std::{
    io::{BufReader, Cursor, Read},
    path::Path,
    sync::Arc,
    time::Duration,
};

pub struct AudioController {
    _output_stream: (OutputStream, OutputStreamHandle),
    sink: Sink,
    total_duration: Option<Duration>,
    audio_data: Option<Arc<[u8]>>,
}

impl std::fmt::Debug for AudioController {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AudioController")
            .field("total_duration", &self.total_duration)
            .field(
                "audio_data",
                &match &self.audio_data {
                    Some(d) => format!("Some( ... {} bytes ... )", d.len()),
                    None => "None".to_string(),
                },
            )
            .finish()
    }
}

impl AudioController {
    pub fn new() -> Result<Self> {
        let output_stream =
            OutputStream::try_default().map_err(|e| AppError::Audio(e.to_string()))?;

        Ok(Self {
            sink: Sink::try_new(&output_stream.1).map_err(|e| AppError::Audio(e.to_string()))?,
            _output_stream: output_stream,
            total_duration: None,
            audio_data: None,
        })
    }

    pub fn is_loaded(&self) -> bool {
        self.audio_data.is_some()
    }

    pub fn reset(&mut self) {
        self.sink.clear();
        self.audio_data = None;
        self.total_duration = None;
    }

    pub fn with_loaded<P: AsRef<Path>>(mut self, music_file: P) -> Result<Self> {
        self.load(music_file).map(|_| self)
    }

    pub fn load<P: AsRef<Path>>(&mut self, music_file: P) -> Result<()> {
        let mut file =
            std::fs::File::open(music_file).map_err(|e| AppError::Audio(e.to_string()))?;
        let mut data: Vec<u8> = vec![];
        file.read_to_end(&mut data)?;

        self.audio_data = Some(Arc::from(data));
        self.renew_source()?;

        Ok(())
    }

    pub fn renew_source(&mut self) -> Result<()> {
        let data = match self.audio_data.clone() {
            Some(d) => d,
            None => {
                return Err(AppError::Audio(
                    "Cannot renew source. No audio is loaded.".to_string(),
                ))?;
            }
        };

        let cursor = Cursor::new(data.clone());
        let source =
            Decoder::new(BufReader::new(cursor)).map_err(|e| AppError::Audio(e.to_string()))?;
        self.total_duration = source.total_duration();

        self.sink.clear();
        self.sink.append(source);

        Ok(())
    }

    pub fn play(&mut self) -> Result<()> {
        if self.sink.empty() {
            self.renew_source()?;
        }

        self.sink.play();

        Ok(())
    }

    pub fn pause(&mut self) {
        self.sink.pause();
    }

    pub fn toggle(&mut self) -> Result<()> {
        if self.is_playing() {
            self.pause();
        } else {
            self.play()?;
        }

        Ok(())
    }

    pub fn is_playing(&self) -> bool {
        !self.sink.empty() && !self.sink.is_paused()
    }

    pub fn try_seek(&mut self, position: Duration) -> Result<()> {
        self.sink.try_seek(position)?;
        Ok(())
    }

    pub fn position(&self) -> Duration {
        self.sink.get_pos()
    }

    pub fn total_duration(&self) -> &Option<Duration> {
        &self.total_duration
    }

    pub fn sleep_until_end(&self) {
        self.sink.sleep_until_end();
    }
}
