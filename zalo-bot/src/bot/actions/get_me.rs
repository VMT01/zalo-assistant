use serde::Serialize;

use crate::bot::constants::endpoints::EEndpoint;
use crate::bot::types::{BotResult, User};
use crate::bot::Bot;

#[derive(Debug, Serialize)]
pub struct GetMeRequest;

impl Bot {
    /// A simple method for testing your bot's auth token. Requires no parameters.
    ///
    /// # Returns
    ///
    /// A [User] instance representing that bot if the credentials are valid
    pub async fn get_me(&self) -> BotResult<User> {
        self.post::<GetMeRequest, User>(EEndpoint::GetMe.into(), None, None)
            .await
    }
}
