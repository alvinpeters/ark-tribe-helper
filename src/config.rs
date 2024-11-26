use dotenvy_macro::dotenv;

// Let's use constants for now
pub(crate) struct Config {
    /// Discord bot token
    pub(crate) bot_token: String,
    /// Interactions endpoint public key
    pub(crate) public_key: String,
}

impl Config {
    pub(crate) fn new() -> Self {
        // Ignore errors here due to the dotenvy_macro bug
        Self {
            bot_token: dotenv!("BOT_TOKEN").to_string(),
            public_key: dotenv!("PUBLIC_KEY").to_string(),
        }
    }
}
