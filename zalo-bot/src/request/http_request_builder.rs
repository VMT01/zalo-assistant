use std::time::Duration;

use clap::Parser;

use crate::request::types::RequestResult;
use crate::request::HttpRequest;

#[derive(Debug, Parser)]
pub(crate) struct HttpRequestBuilder {
    /// User agent for Bot API requests.
    #[arg(long, default_value_t = default_user_agent())]
    user_agent: String,

    /// The URL to a proxy server.
    ///
    /// For example `'http://127.0.0.1:3128'` or `'socks5://127.0.0.1:3128'`.
    ///
    /// Defaults to [None].
    #[clap(long)]
    proxy: Option<String>,

    /// The maximum amount of time (in seconds) to wait for a writing operation
    /// to complete. This value is used unless a different value is passed to
    /// [do_request].
    ///
    /// Defaults to `10`.
    #[clap(long, default_value_t = 10)]
    pub(super) timeout: u64,
}

fn default_user_agent() -> String {
    format!("zalo-bot-rs v{}", env!("CARGO_PKG_VERSION"))
}

fn default_timeout() -> Duration {
    Duration::from_secs(10)
}

impl Default for HttpRequestBuilder {
    fn default() -> Self {
        Self::parse()
    }
}

impl HttpRequestBuilder {
    pub(crate) fn build(self) -> RequestResult<HttpRequest> {
        let mut cb = reqwest::ClientBuilder::new();

        cb = cb.user_agent(self.user_agent.clone());

        if let Some(proxy) = self.proxy.clone() {
            let proxy = if proxy.starts_with("http://") {
                reqwest::Proxy::http(proxy)?
            } else if proxy.starts_with("https://") {
                reqwest::Proxy::https(proxy)?
            } else if proxy.starts_with("socks5://") {
                reqwest::Proxy::all(proxy)?
            } else {
                reqwest::Proxy::http(format!("http://{}", proxy))?
            };

            cb = cb.proxy(proxy);
        }

        cb = cb.timeout(Duration::from_secs(self.timeout));

        let client = cb.build()?;

        Ok(HttpRequest {
            config: self,
            client,
        })
    }
}
