use std::{error::Error, fmt};

use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct SimpleAPIError {
    pub msg: String,
    pub code: u16,
}

impl SimpleAPIError {
    pub fn new(msg: String, code: u16) -> Self {
        SimpleAPIError { msg, code }
    }
}

impl Error for SimpleAPIError {}

impl fmt::Display for SimpleAPIError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}
