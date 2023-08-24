use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct MinMaxStep {
    pub min: u32,
    pub max: u32,
    pub step: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Price {
    pub id: u32,
    pub name: String,
    pub format: String,
    pub price: f64,
    pub description: String,
    pub views: MinMaxStep,
    pub online_viewers: MinMaxStep,
}
