// #[derive(Debug, thiserror::Error)]
// pub enum HttpError {
//     #[error("Request error: {0}")]
//     Request(#[from] reqwest::Error),
//     #[error("Missing data in response: {0}")]
//     MissingData(String),
//     #[error("API error [{code}]: {message}")]
//     Api { code: usize, message: String },
//     #[error("Unknown error: {0}")]
//     Other(String),
// }
//
// #[derive(Debug, thiserror::Error)]
// pub enum ZaloRequestError {
//     #[error("{0}")]
//     Build(#[from] reqwest::Error),
// }

#[derive(Debug, thiserror::Error)]
pub enum RequestError {
    #[error("reqwest error: {0}")]
    Request(#[from] reqwest::Error),
    #[error("Missing data in response: {0}")]
    MissingData(String),
    #[error("API error [{code}]: {message}")]
    Api { code: usize, message: String },
}
