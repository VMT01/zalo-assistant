mod actions;
mod bot_builder;
mod constants;
pub mod types;

use std::fmt::Debug;
use std::time::Duration;

use serde::de::DeserializeOwned;
use serde::Serialize;
use url::Url;

pub use self::bot_builder::BotBuilder;
use crate::bot::types::BotResult;
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

    async fn post<U, T>(
        &self,
        endpoint: String,
        data: Option<U>,
        timeout: Option<Duration>,
    ) -> BotResult<T>
    where
        U: Serialize + Debug,
        T: DeserializeOwned + Debug,
    {
        let url = Url::parse(&format!("{}/{}", self.base_url, endpoint))?;
        self.client
            .post::<U, T>(url, data, timeout)
            .await
            .map_err(Into::into)
    }
}
