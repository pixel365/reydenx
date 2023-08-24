use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: u32,
    pub username: String,
    pub date_joined: String,
    pub email: String,
    pub is_active: bool,
    pub is_blocked: bool,
    pub has_image: bool,
    pub image_url: String,
    pub currency_id: u32,
    pub discount_value: u32,
    pub is_reseller: bool,
    pub twitch_id: u32,
    pub twitch_login: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Balance {
    pub id: u32,
    pub amount: u32,
    pub currency_id: u32,
    pub user_id: u32,
    pub formatted_amount: u32,
    pub currency: String,
}
