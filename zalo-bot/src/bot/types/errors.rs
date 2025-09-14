use crate::request;

#[derive(Debug, thiserror::Error)]
pub enum BotError {
    #[error("zalo-request error: {0}")]
    Request(#[from] request::types::errors::RequestError),
    #[error("url parse error: {0}")]
    Url(#[from] url::ParseError),
    // #[error("{0}")]
    // ZaloRequestError(#[from] ZaloRequestError),
}
