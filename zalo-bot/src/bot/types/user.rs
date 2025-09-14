use serde::Deserialize;

/// This object represents a Zalo bot
#[derive(Debug, Deserialize)]
pub struct User {
    /// Unique identifier for this user
    id: String,

    /// Name of the account
    account_name: Option<String>,

    // NOTE: THIS CAN BE AN ENUM
    /// Type of the  account
    account_type: Option<String>,

    /// `True` if the bot can be invited to groups
    /// Returned only in [Bot::get_me]
    can_join_groups: Option<bool>,

    /// Display name of the user
    display_name: Option<String>,

    /// `True` if the user is a bot
    is_bot: Option<bool>,
}
