use std::{collections::HashMap, error::Error, fmt};

use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct DetailedAPIError {
    pub msg: String,
    pub code: u16,
    pub field_validations: Option<HashMap<String, String>>,
}

impl DetailedAPIError {
    pub fn new(msg: String, code: u16, field_validations: Option<HashMap<String, String>>) -> Self {
        DetailedAPIError {
            msg,
            code,
            field_validations,
        }
    }
}

impl Error for DetailedAPIError {}

impl fmt::Display for DetailedAPIError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}
