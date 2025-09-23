use chrono::{DateTime, Local};
use serde::{Deserialize, Deserializer};

use crate::bot::types::chat::Chat;
use crate::bot::types::User;

#[derive(Debug, Deserialize)]
pub struct Message {
    message_id: String,
    #[serde(deserialize_with = "from_millis")]
    date: DateTime<Local>,
    chat: Chat,
    message_type: Option<String>,
    text: Option<String>,
    sticker: Option<String>,
    photo_url: Option<String>,
    from: Option<User>,
}

fn from_millis<'de, D>(deserializer: D) -> Result<DateTime<Local>, D::Error>
where
    D: Deserializer<'de>,
{
    let millis: i64 = Deserialize::deserialize(deserializer)?;
    let secs = millis / 1000;
    let nsecs = ((millis % 1000) * 1_000_000) as u32;

    match DateTime::from_timestamp(secs, nsecs) {
        Some(utc_dt) => {
            // Try to convert to local timezone, fallback to treating as local time
            Ok(utc_dt.with_timezone(&Local))
        }
        None => Err(serde::de::Error::custom("invalid timestamp")),
    }
}
