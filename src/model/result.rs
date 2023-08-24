use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct StandardResult<T> {
    pub request_id: String,
    pub cached: bool,
    pub cache_expires_at: Option<String>,
    pub cursor: Option<String>,
    pub result: T,
}

impl<T> StandardResult<T> {
    pub fn has_next(&self) -> bool {
        match &self.cursor {
            Some(_) => true,
            None => false,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub id: String,
    pub url: String,
    pub expires_at: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ActionResult {
    pub request_id: String,
    pub order_id: u32,
    pub action: String,
    pub value: u32,
    pub task: Task,
}
