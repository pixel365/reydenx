use std::fmt;

use serde::{
    ser::{Error, SerializeStruct},
    Deserialize, Serialize,
};

use super::error::ValueError;

#[derive(Deserialize, Debug)]
pub enum LaunchMode {
    Auto,
    Manual,
    Delay,
}

impl Serialize for LaunchMode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match &self {
            LaunchMode::Auto => serializer.serialize_str("auto"),
            LaunchMode::Manual => serializer.serialize_str("manual"),
            LaunchMode::Delay => serializer.serialize_str("delay"),
        }
    }
}

impl fmt::Display for LaunchMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            LaunchMode::Auto => write!(f, "auto"),
            LaunchMode::Manual => write!(f, "manual"),
            LaunchMode::Delay => write!(f, "delay"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Parameters {
    pub launch_mode: String,
    pub work_mode: String,
    pub delay: bool,
    pub delay_time: u32,
    pub even_distribution: bool,
    pub even_distribution_time: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AvgValues {
    pub in_settings: f64,
    pub in_fact: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Average {
    pub online: AvgValues,
    pub session_in_seconds: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Statistics {
    pub active_time_in_seconds: u32,
    pub views: u32,
    pub clicks: u32,
    pub ctr: f64,
    pub average: Average,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Order {
    pub id: u32,
    pub created_at: String,
    pub updated_at: String,
    pub uuid: String,
    pub status: String,
    pub ordered_view_qty: u32,
    pub price_per_view: f64,
    pub is_autostart: bool,
    pub online_users_limit: u32,
    pub platform: String,
    pub content_type: String,
    pub parameters: Parameters,
    pub statistics: Option<Statistics>,
    pub content_classification_labels: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OnlineStats {
    pub created_at: String,
    pub in_settings: f64,
    pub in_fact: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DateAndQuantity {
    pub date: String,
    pub quantity: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IdAndQuantity {
    pub id: u32,
    pub quantity: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SiteStats {
    pub domain: String,
    pub views: i32,
    pub clicks: u32,
    pub ctr: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Payment {
    pub id: u32,
    pub created_at: String,
    pub updated_at: String,
    pub payed_at: String,
    pub amount: u32,
    pub external_id: String,
    pub uuid: String,
    pub receipt: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SmoothGain {
    pub enabled: bool,
    pub minutes: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TwitchPayload {
    pub price_id: u32,
    pub number_of_views: u32,
    pub number_of_viewers: u32,
    pub launch_mode: String,
    pub smooth_gain: SmoothGain,
    pub delay_time: u32,
    pub twitch_id: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct YouTubePayload {
    pub price_id: u32,
    pub number_of_views: u32,
    pub number_of_viewers: u32,
    pub launch_mode: String,
    pub smooth_gain: SmoothGain,
    pub delay_time: u32,
    pub channel_url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Identifiers {
    pub identifiers: Vec<u32>,
}

#[derive(Deserialize, Debug)]
pub struct LaunchParams {
    pub mode: LaunchMode,
    pub delay_time: u8,
}

impl Serialize for LaunchParams {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = serializer.serialize_struct("LaunchParams", 2)?;
        match self.mode {
            LaunchMode::Delay => {
                if self.delay_time < 5 || self.delay_time > 240 {
                    return Err(ValueError {
                        message: String::from(
                            "The number of minutes for delayed start should be from 5 to 240",
                        ),
                    })
                    .map_err(S::Error::custom);
                }
                s.serialize_field("delay_time", &self.delay_time)?;
            }
            _ => {
                s.serialize_field("delay_time", &0)?;
            }
        }
        s.serialize_field("mode", &self.mode)?;
        s.end()
    }
}
