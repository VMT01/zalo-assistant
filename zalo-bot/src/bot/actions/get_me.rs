use crate::bot::constants::endpoints::EEndpoint;
use crate::bot::types::{BotResult, User};
use crate::bot::{Bot, BotAction};

#[async_trait::async_trait]
impl BotAction for Bot {
    async fn get_me(&self) -> BotResult<User> {
        self.post(EEndpoint::GetMe.into(), None).await
    }
}
