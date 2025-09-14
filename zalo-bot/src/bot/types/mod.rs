mod errors;
mod user;

pub use errors::BotError;
pub use user::User;

pub type BotResult<T> = Result<T, BotError>;
