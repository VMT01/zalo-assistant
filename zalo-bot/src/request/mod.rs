use std::fmt::Debug;
use std::time::Duration;

use reqwest::Body;
use serde::Serialize;
use serde::de::DeserializeOwned;

use crate::request::http_request_builder::HttpRequestBuilder;
use crate::request::types::RequestResult;
use crate::request::types::errors::RequestError;
use crate::request::types::response_dto::ResponseDTO;

mod http_request_builder;

pub(crate) mod types;

#[derive(Debug)]
pub(crate) struct HttpRequest {
    config: HttpRequestBuilder,
    client: reqwest::Client,
}

impl HttpRequest {
    pub(crate) fn builder() -> HttpRequestBuilder {
        HttpRequestBuilder::default()
    }

    pub(crate) fn get_timeout(&self) -> u64 {
        self.config.timeout
    }

    pub(crate) async fn post<U, T>(
        &self,
        url: reqwest::Url,
        data: Option<U>,
        timeout: Option<Duration>,
    ) -> RequestResult<T>
    where
        U: Serialize + Debug,
        T: DeserializeOwned + Debug,
    {
        let body = match data {
            Some(data) => match serde_json::to_vec(&data) {
                Ok(serialized) => Body::from(serialized),
                Err(_) => Body::default(),
            },
            None => Body::default(),
        };
        let mut request = self.client.post(url).body(body);
        if let Some(timeout) = timeout {
            request = request.timeout(timeout);
        }
        let response = request.send().await?.error_for_status()?;

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
