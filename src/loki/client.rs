use async_trait::async_trait;
use prost::Message;
use snap::raw::Encoder;

use crate::config;

use super::PushRequest;

#[async_trait]
pub trait Client {
    async fn push(&self, request: PushRequest);
}

#[derive(Clone, Debug)]
pub struct DefaultClient {
    client: reqwest::Client,
    config: config::Loki,
}

impl DefaultClient {
    pub fn new(config: config::Loki) -> Self {
        Self {
            client: reqwest::Client::new(),
            config,
        }
    }
}

#[async_trait]
impl Client for DefaultClient {
    async fn push(&self, request: PushRequest) {
        let client = self.client.clone();
        let url = self.config.url.clone();
        let username = self.config.username.clone();
        let password = self.config.password.clone();
        let body = Encoder::new()
            .compress_vec(&request.encode_to_vec())
            .unwrap_or_else(|_| vec![]);

        if body.is_empty() {
            log::error!("snappy compression failed");
            return;
        }

        let mut req = client
            .post(url)
            .header("Content-Type", "application/x-protobuf")
            .header("Content-Encoding", "snappy")
            .header("User-Agent", "bluez-monitor/0.1.0")
            .body(body);

        if let Some(username) = username {
            req = req.basic_auth(username.clone(), password);
        }

        let res = req.send().await;

        match res {
            Ok(resp) => {
                if !resp.status().is_success() {
                    log::error!("loki push request failed: {:?}", resp.text().await.unwrap());
                }
            }
            Err(e) => {
                log::error!("loki push request failed: {:?}", e);
            }
        }
    }
}
