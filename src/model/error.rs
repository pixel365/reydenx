use std::fmt::{self, Display, Formatter};

use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct ResponseError {
    pub status: StatusCode,
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Error {
    pub timestamp: String,
    pub errors: Vec<String>,
}

impl Display for ResponseError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Status: {}, Message: {}", self.status, self.message)
    }
}

impl std::error::Error for ResponseError {}
