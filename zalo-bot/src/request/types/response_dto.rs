use serde::de::DeserializeOwned;
use serde::Deserialize;
use std::fmt::Debug;

#[derive(Debug, Deserialize)]
#[serde(bound = "T: DeserializeOwned")]
pub(crate) struct ResponseDTO<T>
where
    T: DeserializeOwned + Debug,
{
    pub(crate) ok: bool,
    pub(crate) result: Option<T>,
    pub(crate) description: Option<String>,
    pub(crate) error_code: Option<usize>,
}
