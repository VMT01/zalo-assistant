mod actions;
mod bot_builder;
mod constants;
pub mod types;

use std::fmt::Debug;

use serde::de::DeserializeOwned;
use url::Url;

pub use self::bot_builder::BotBuilder;

use crate::bot::types::{BotResult, User};
use crate::request::HttpRequest;

#[derive(Debug)]
pub struct Bot {
    pub(crate) base_url: String,
    pub(crate) client: HttpRequest,
}

impl Bot {
    pub fn builder() -> BotBuilder {
        BotBuilder::default()
    }

    async fn post<T>(&self, endpoint: String, data: Option<serde_json::Value>) -> BotResult<T>
    where
        T: DeserializeOwned + Debug,
    {
        let url = Url::parse(&format!("{}/{}", self.base_url, endpoint))?;
        self.client.post::<T>(url, data).await.map_err(Into::into)
    }
}

#[async_trait::async_trait]
pub trait BotAction {
    /// A simple method for testing your bot's auth token. Requires no parameters.
    ///
    /// # Returns
    ///
    /// A [User] instance representing that bot if the credentials are valid
    async fn get_me(&self) -> BotResult<User>;
}
