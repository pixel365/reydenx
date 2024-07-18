use std::fmt::{self, Display, Formatter};

use reqwest::StatusCode;
use serde::{ser, Deserialize, Serialize};

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

#[derive(Debug)]
pub struct ValueError {
    pub message: String,
}

impl Display for ValueError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:?}", self.message)
    }
}

impl std::error::Error for ValueError {}

impl ser::Error for ValueError {
    fn custom<T: Display>(msg: T) -> Self {
        ValueError {
            message: msg.to_string(),
        }
    }
}
