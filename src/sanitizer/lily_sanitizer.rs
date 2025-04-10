//! Lilypond score sanitizer definition
//!
//! ### Author
//! Jakub Kloub (xkloub03), VUT FIT

use crate::{
    error::*,
    lily::{LilyStave, LilySymbol, LilyTime, Lilypond},
    sanitizer::Sanitizer,
};

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct LilySanitizer {
    pub max_line_notes: u8,
    pub max_line_bars: u8,
}

impl Default for LilySanitizer {
    fn default() -> Self {
        Self {
            max_line_notes: 45,
            max_line_bars: 7,
        }
    }
}

impl Sanitizer<Lilypond> for LilySanitizer {
    fn sanitize(&self, score: &mut Lilypond) -> Result<()> {
        for stave in score.staves.iter_mut() {
            self.sanitize_stave(stave)?;
        }

        Ok(())
    }
}

impl LilySanitizer {
    fn sanitize_stave(&self, stave: &mut LilyStave) -> Result<()> {
        let time = self
            .find_time_sig(stave)
            .ok_or(AppError::MissingTimeSignature)?;

        let total_bar_len = (time.nom as u16) * time.denom.value_128();
        let mut line_notes = 0;
        let mut line_bars = 0;
        let mut current_bar_len = 0;

        let mut breaks: Vec<usize> = vec![];

        for (i, note) in stave
            .symbols
            .iter()
            .enumerate()
            .filter_map(|(i, n)| match n {
                LilySymbol::Note(n) => Some((i, n)),
                _ => None,
            })
        {
            line_notes += 1;
            current_bar_len += note.length.value_128();

            if current_bar_len >= total_bar_len {
                line_bars += 1;
                current_bar_len -= total_bar_len;
            }

            if line_notes >= self.max_line_notes || line_bars >= self.max_line_bars {
                breaks.push(i);
                line_notes = 0;
                line_bars = 0;
            }
        }

        for (i, idx) in breaks.into_iter().enumerate() {
            stave.symbols.insert(idx + i + 1, LilySymbol::Break);
        }

        Ok(())
    }

    fn find_time_sig<'a>(&self, stave: &'a LilyStave) -> Option<&'a LilyTime> {
        stave.symbols.iter().find_map(|s| match s {
            crate::lily::LilySymbol::Time(t) => Some(t),
            _ => None,
        })
    }
}
