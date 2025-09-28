use futures::{StreamExt, pin_mut};
use http::{HeaderMap, StatusCode};
use reqwest::Body;

use crate::Output;

struct HttpInput {
    endpoint: String,
    method: http::method::Method,
    codec: String,
}

struct HttpOutput {
    endpoint: reqwest::Url,
    method: http::method::Method,
    client: reqwest::Client,
}

impl HttpOutput {
    pub fn new<T: Into<reqwest::Url>>(
        endpoint: T,
        method: http::method::Method,
        default_headers: Option<HeaderMap>,
    ) -> Self {
        let client = {
            let mut client_builder = reqwest::ClientBuilder::new();
            if let Some(headers) = default_headers {
                client_builder = client_builder.default_headers(headers);
            }
            client_builder.build().unwrap()
        };
        Self {
            endpoint: endpoint.into(),
            method,
            client,
        }
    }
}

impl<T: Into<Body> + Send> Output<T> for HttpOutput {
    fn output<S>(
        &self,
        stream: S,
    ) -> impl std::future::Future<Output = Result<(), std::io::Error>> + Send
    where
        S: futures::Stream<Item = T> + Send,
    {
        async move {
            pin_mut!(stream);
            while let Some(item) = stream.next().await {
                loop {
                    match self.client.post(self.endpoint.clone())
                    .body(item.into()).send().await {
                        Ok(response) => {
                            let status = response.status();
                            if status.is_success() {
                                break;
                            }
                            match status {
                                StatusCode::TOO_MANY_REQUESTS => {
                                    tracing::warn!("[429] backoff");
                                    break;
                                }
                                _ => {
                                    tracing::error!("unexpected status: [{}], response: {:?}", status, response.text().await);
                                    break;
                                }
                            }
                        },
                        Err(e) => {
                            tracing::error!("error: {e:?}");
                            break;
                        },
                    }
                }
            }
            Ok(())
        }
    }
}

