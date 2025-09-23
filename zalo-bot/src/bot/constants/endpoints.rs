pub(in crate::bot) enum EEndpoint {
    GetMe,
    GetUpdate,
}

impl From<EEndpoint> for String {
    fn from(value: EEndpoint) -> Self {
        match value {
            EEndpoint::GetMe => "getMe".to_string(),
            EEndpoint::GetUpdate => "getUpdates".to_string(),
        }
    }
}
