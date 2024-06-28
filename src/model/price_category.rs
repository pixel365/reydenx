use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PriceCategory {
    pub id: u32,
    pub is_active: bool,
    pub name: String,
    pub description: String,
}
