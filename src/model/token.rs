use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Token {
    pub access_token: String,
    pub expires_in: String,
}

impl Token {
    pub fn is_valid(&self) -> bool {
        if self.access_token.is_empty() || self.expires_in.is_empty() {
            return false;
        }
        !self.access_token.is_empty() && !self.is_expired()
    }

    pub fn is_expired(&self) -> bool {
        if !self.expires_in.is_empty() {
            let now = Utc::now();
            let expires_in = DateTime::parse_from_rfc3339(&self.expires_in).unwrap();
            return now.timestamp() <= expires_in.timestamp();
        }
        true
    }
}
