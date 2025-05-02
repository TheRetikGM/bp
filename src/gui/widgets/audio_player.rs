//! Audio player widget definition
//!
//! ### Author
//! Jakub Kloub (xkloub03), VUT FIT

use std::time::Duration;

use egui::{style::HandleShape, Slider};

use crate::error::*;
use crate::utils::AudioController;

fn as_digital_time(dur: &Duration) -> String {
    let mins = dur.as_secs() / 60;
    let secs = dur.as_secs() % 60;
    let mills = dur.as_millis() % 1_000;
    format!("{mins:2}:{secs:02}:{mills:03}")
}

pub struct AudioPlayer<'a> {
    controller: &'a mut AudioController,
}

impl<'a> AudioPlayer<'a> {
    pub fn new(controller: &'a mut AudioController) -> Self {
        AudioPlayer { controller }
    }

    fn show(&mut self, ui: &mut egui::Ui) -> Result<()> {
        // Show toggle button
        let play_button_text = if self.controller.is_playing() {
            ui.ctx().request_repaint();
            "Pause"
        } else {
            "Play"
        };

        if ui.button(play_button_text).clicked() {
            self.controller.toggle()?;
        }

        // Show the slider
        let total_dur_ms = self
            .controller
            .total_duration()
            .ok_or(AppError::AudioPlayer("Cannot get duration".to_string()))?
            .as_millis() as u64;
        let mut pos_ms = self.controller.position().as_millis() as u64;

        ui.style_mut().spacing.slider_width = ui.available_width() - 125.0;
        ui.style_mut().visuals.slider_trailing_fill = true;
        let time_slider = ui.add(
            Slider::new(&mut pos_ms, 0..=total_dur_ms)
                .show_value(false)
                .logarithmic(false)
                .clamping(egui::SliderClamping::Always)
                .trailing_fill(true)
                .handle_shape(HandleShape::Rect { aspect_ratio: 0.5 }),
        );

        if time_slider.dragged() {
            self.controller.try_seek(Duration::from_millis(pos_ms))?;
        }

        // Show the remaining time
        ui.label(format!(
            "{} / {}",
            as_digital_time(&self.controller.position()),
            as_digital_time(&self.controller.total_duration().unwrap())
        ));

        Ok(())
    }
}

impl<'a> egui::Widget for AudioPlayer<'a> {
    fn ui(mut self, ui: &mut egui::Ui) -> egui::Response {
        egui::Frame::new()
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    if let Err(e) = self.show(ui) {
                        log::error!("{e}");
                    }
                });
            })
            .response
    }
}
