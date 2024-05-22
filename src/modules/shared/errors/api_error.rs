use std::{error::Error, fmt};

use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct APIError {
    pub msg: String,
    pub code: u16,
}

impl APIError {
    pub fn new(msg: String, code: u16) -> Self {
        APIError { msg, code }
    }
}

impl Error for APIError {}

impl fmt::Display for APIError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}
