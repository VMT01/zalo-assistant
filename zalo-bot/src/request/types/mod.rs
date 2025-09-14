pub(crate) mod errors;
mod request_dto;
pub(in crate::request) mod response_dto;

pub(crate) type RequestResult<T> = Result<T, errors::RequestError>;
