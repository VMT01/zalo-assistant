pub(in crate::bot) enum EEndpoint {
    GetMe,
}

impl From<EEndpoint> for String {
    fn from(value: EEndpoint) -> Self {
        match value {
            EEndpoint::GetMe => "getMe".to_string(),
        }
    }
}
