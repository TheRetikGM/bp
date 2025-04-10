//! Texture util definition
//!
//! ### Author
//! Jakub Kloub (xkloub03), VUT FIT

use crate::error::{AppError, Result};
use std::{path::PathBuf, sync::Arc};

pub struct Texture(pub Arc<egui::ColorImage>, pub egui::TextureHandle);

impl std::fmt::Debug for Texture {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Texture").field(&self.0).finish()
    }
}

impl Texture {
    pub fn load_from(path: &PathBuf, ctx: &egui::Context) -> Result<Self> {
        Ok(Texture::load_from_memory(
            InMemoryTexture::load_from(path)?,
            ctx,
        ))
    }

    pub fn load_from_memory(mem_tex: InMemoryTexture, ctx: &egui::Context) -> Self {
        let color_image = Arc::new(egui::ColorImage {
            size: mem_tex.size,
            pixels: mem_tex.pixels,
        });

        let handle = ctx.load_texture("score_image", color_image.clone(), Default::default());

        Self(color_image, handle)
    }
}

pub struct InMemoryTexture {
    pub pixels: Vec<egui::Color32>,
    pub size: [usize; 2],
}

impl InMemoryTexture {
    pub fn load_from(path: &PathBuf) -> Result<Self> {
        let image = image::open(path).map_err(|e| AppError::build_path(path, &e))?;
        let size = [image.width() as usize, image.height() as usize];
        let image_buffer = image.to_rgba8();
        let pixels: Vec<_> = image_buffer
            .pixels()
            .map(|p| egui::Color32::from_rgba_unmultiplied(p[0], p[1], p[2], p[3]))
            .collect();

        Ok(Self { size, pixels })
    }
}
