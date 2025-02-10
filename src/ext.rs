use regex::Captures;

use crate::error::*;

pub trait WithoutWhitespaces {
    fn without_whitespaces(&self) -> String;
}

impl<S: AsRef<str>> WithoutWhitespaces for S {
    fn without_whitespaces(&self) -> String {
        self.as_ref()
            .chars()
            .filter(|c| !c.is_whitespace())
            .collect::<String>()
    }
}

pub trait CapturedStr {
    fn captured_str(&self, i: usize) -> Result<&str>;
}

impl<'a> CapturedStr for Captures<'a> {
    fn captured_str(&self, i: usize) -> Result<&str> {
        self.get(i)
            .map(|m| m.as_str())
            .ok_or(AppError::Other("Invalid regex string capture".to_string()).into())
    }
}
