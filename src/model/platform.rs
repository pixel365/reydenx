use std::fmt::{Debug, Display};

#[derive(Debug)]
pub enum Platform {
    Twitch,
    YouTube,
    GoodGame,
    Trovo,
    VkPlay,
}

impl Display for Platform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Platform::Twitch => write!(f, "twitch"),
            Platform::YouTube => write!(f, "youtube"),
            Platform::GoodGame => write!(f, "goodgame"),
            Platform::Trovo => write!(f, "trovo"),
            Platform::VkPlay => write!(f, "vkplay"),
        }
    }
}
