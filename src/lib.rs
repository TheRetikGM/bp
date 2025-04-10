#![feature(try_trait_v2)]
#![feature(path_add_extension)]
//! Main libary module
//!
//! ### Author
//! Jakub Kloub (xkloub03), VUT FIT

use crate::error::{AppError, Result};
use std::{env, path::PathBuf};

pub mod error;
pub mod ext;
pub mod gui;
pub mod lily;
pub mod lsystem;
pub mod notation;
pub mod sanitizer;
pub mod utils;

pub struct Arguments {
    pub sound_font_path: PathBuf,
}

impl Arguments {
    pub fn new() -> Result<Self> {
        let args: Vec<String> = env::args().collect();

        if args.len() != 2 {
            Err(AppError::Argument(
                "Invalid number of arguemnts".to_string(),
            ))?;
        }

        let args = Self {
            sound_font_path: PathBuf::from(args[1].clone()),
        };

        Ok(args)
    }
}
