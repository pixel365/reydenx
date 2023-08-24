pub enum Platform {
    Twitch,
    YouTube,
    GoodGame,
    Trovo,
    VkPlay,
}

impl Platform {
    pub fn slug(&self) -> String {
        match &self {
            Platform::Twitch => String::from("twitch"),
            Platform::YouTube => String::from("youtube"),
            Platform::GoodGame => String::from("goodgame"),
            Platform::Trovo => String::from("trovo"),
            Platform::VkPlay => String::from("vkplay"),
        }
    }
}
