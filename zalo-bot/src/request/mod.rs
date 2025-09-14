use reqwest::Body;
use serde::de::DeserializeOwned;
use std::fmt::Debug;

use crate::request::http_request_builder::HttpRequestBuilder;
use crate::request::types::errors::RequestError;
use crate::request::types::response_dto::ResponseDTO;
use crate::request::types::RequestResult;

mod http_request_builder;

pub(crate) mod types;

#[derive(Debug)]
pub(crate) struct HttpRequest {
    client: reqwest::Client,
}

impl HttpRequest {
    pub(crate) fn builder() -> HttpRequestBuilder {
        HttpRequestBuilder::default()
    }

    pub(crate) async fn post<T>(
        &self,
        url: reqwest::Url,
        data: Option<serde_json::Value>,
    ) -> RequestResult<T>
    where
        T: DeserializeOwned + Debug,
    {
        let body = data.map_or(Body::default(), |data| data.to_string().into());
        let response = self
            .client
            .post(url)
            .body(body)
            .send()
            .await?
            .error_for_status()?;

        let data = response.json::<ResponseDTO<T>>().await?;
        if !data.ok {
            let error_code = data
                .error_code
                .ok_or_else(|| RequestError::MissingData("error_code".to_string()))?;
            let description = data
                .description
                .ok_or_else(|| RequestError::MissingData("description".to_string()))?;
            return Err(RequestError::Api {
                code: error_code,
                message: description,
            });
        }

        data.result
            .ok_or_else(|| RequestError::MissingData("result".to_string()))
    }
}
