use std::time::Duration;

use serde::Serialize;

use crate::bot::constants::endpoints::EEndpoint;
use crate::bot::types::{BotResult, Update};
use crate::bot::Bot;

#[derive(Debug, Serialize)]
pub struct GetUpdateRequest {
    pub offset: Option<usize>,
    pub limit: Option<usize>,
    pub timeout: Option<u64>,
    pub allowed_updates: Option<Vec<String>>,
}

impl Bot {
    /// Receive incoming updates using long polling.
    ///
    /// Note:
    /// 1. This method will not work if an outgoing webhook is set up.
    /// 2. In order to avoid getting duplicate updates, recalculate offset after each server response.
    /// 3. To take full advantage of this library take a look at :class:`zalo_bot.ext.Updater`
    pub async fn get_update(&self, request: GetUpdateRequest) -> BotResult<Update> {
        let default_timeout = self.client.get_timeout();
        let timeout = request
            .timeout
            .map_or(default_timeout, |t| t + default_timeout);

        self.post(
            EEndpoint::GetUpdate.into(),
            Some(request),
            Some(Duration::from_secs(timeout)),
        )
        .await
    }
}
