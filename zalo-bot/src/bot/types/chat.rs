use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Chat {
    id: String,
    chat_type: String, // PRIVATE
}
