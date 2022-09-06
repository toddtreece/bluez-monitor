use std::ops::{Deref, DerefMut};

use async_trait::async_trait;

use crate::bluetooth::Device;
use crate::config::HOSTNAME;
use crate::device_writer;

use super::Client;
use super::{EntryAdapter, PushRequest, StreamAdapter};

#[derive(Debug, Clone)]
pub struct Push<C>
where
    C: Client + Send + Sync,
{
    client: C,
}

impl<C> Push<C>
where
    C: Client + Send + Sync,
{
    pub fn new(client: C) -> Push<C> {
        Self { client }
    }
}

#[async_trait]
impl<C> device_writer::DeviceWriter for Push<C>
where
    C: Client + Send + Sync,
{
    async fn write(&mut self, device: Device) {
        let req = PushRequest {
            streams: vec![StreamAdapter {
                labels: Labels::from(device.clone()).0,
                entries: vec![EntryAdapter {
                    timestamp: Some(prost_types::Timestamp::from(device.timestamp)),
                    line: serde_json::to_string(&device).unwrap(),
                }],
                hash: 0,
            }],
        };
        self.client.push(req).await;
    }
}

pub struct Labels(pub String);

impl From<Device> for Labels {
    fn from(device: Device) -> Self {
        #[allow(clippy::unnecessary_to_owned)]
        let host = HOSTNAME.to_string();

        let mut labels = format!("address={:?}, host={:?}", device.address.to_string(), host);
        if let Some(name) = device.name {
            labels = format!("{}, name={:?}", labels, name);
        }
        labels = format!("{{{}}}", labels);

        log::trace!("loki labels: {}", labels);
        Labels(labels)
    }
}

impl Deref for Labels {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl DerefMut for Labels {
    fn deref_mut(&mut self) -> &mut String {
        &mut self.0
    }
}
