use clap::Parser;

use crate::bot::constants::BASE_URL;
use crate::bot::types::BotResult;
use crate::bot::Bot;
use crate::request::HttpRequest;

#[derive(Debug, Parser)]
pub struct BotBuilder {
    /// Zalo BOT token
    ///
    /// Get the token [here](https://bot.zapps.me/docs/create-bot/)
    #[arg(long, env = "TOKEN")]
    token: String,

    /// Zalo API url
    #[arg(
        long,
        env = "BASE_URL",
        default_value_t = default_base_url()
    )]
    base_url: String,
}

fn default_base_url() -> String {
    BASE_URL.to_string()
}

impl Default for BotBuilder {
    fn default() -> Self {
        Self::parse()
    }
}

impl BotBuilder {
    pub fn new(token: String, base_url: String) -> Self {
        Self { token, base_url }
    }

    pub fn token(mut self, token: &str) -> Self {
        self.token = token.to_string();
        self
    }

    pub fn base_url(mut self, base_url: &str) -> Self {
        self.base_url = base_url.to_string();
        self
    }

    pub fn build(&self) -> BotResult<Bot> {
        let base_url = format!("{}/bot{}", self.base_url, self.token);
        let client = HttpRequest::builder().build()?;
        Ok(Bot { base_url, client })
    }
}

#[cfg(test)]
mod tests {
    use crate::bot::bot_builder::BotBuilder;

    fn parse_env() -> (String, String) {
        dotenvy::dotenv().ok();

        let token = std::env::var("TOKEN").expect("token is missing");
        let base_url = std::env::var("BASE_URL").expect("base_url is missing");

        (token, base_url)
    }

    #[test]
    fn should_parse_env() {
        let (token, base_url) = parse_env();
        let _ = BotBuilder::new(token, base_url);
    }

    #[test]
    fn should_build_zalo_bot() {
        let (token, base_url) = parse_env();
        let _ = BotBuilder::new(token, base_url).build();
    }
}
