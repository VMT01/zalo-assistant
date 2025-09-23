mod chat;
mod errors;
mod message;
mod update;
mod user;

pub use errors::BotError;
pub use update::Update;
pub use user::User;

pub type BotResult<T> = Result<T, BotError>;

// Request types
pub use crate::bot::actions::get_me::GetMeRequest;
pub use crate::bot::actions::get_update::GetUpdateRequest;
