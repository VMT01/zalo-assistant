use crate::bot::types::message::Message;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Update {
    message: Message,
}
