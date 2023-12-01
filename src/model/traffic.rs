use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Traffic {
    pub code: String,
    pub quantity: u32,
}
