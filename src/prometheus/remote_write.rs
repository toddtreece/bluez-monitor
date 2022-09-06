use std::time::UNIX_EPOCH;

use async_trait::async_trait;

use crate::bluetooth::Device;
use crate::config::HOSTNAME;
use crate::device_writer;

use super::{
    metric_metadata::MetricType, Client, Label, MetricMetadata, Sample, TimeSeries, WriteRequest,
};

#[derive(Debug, Clone)]
struct Labels(Vec<Label>);

impl From<Device> for Labels {
    fn from(device: Device) -> Self {
        let mut labels = vec![
            Label {
                name: "address".to_owned(),
                value: device.address.to_string(),
            },
            Label {
                name: "host".to_owned(),
                value: HOSTNAME.to_string(),
            },
        ];

        if let Some(name) = device.name {
            labels.push(Label {
                name: "name".to_owned(),
                value: name,
            });
        }
        Labels(labels)
    }
}

#[derive(Debug, Clone)]
pub struct RemoteWrite<C>
where
    C: Client,
{
    client: C,
}

impl<C> RemoteWrite<C>
where
    C: Client,
{
    pub fn new(client: C) -> RemoteWrite<C> {
        Self { client }
    }

    fn get_rssi(&self, device: Device) -> (TimeSeries, MetricMetadata) {
        let mut labels = Labels::from(device.clone());
        labels.0.push(Label {
            name: "__name__".to_owned(),
            value: "bluetooth_rssi".to_owned(),
        });
        let series = TimeSeries {
            labels: labels.0,
            samples: vec![Sample {
                timestamp: device
                    .timestamp
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_millis() as i64,
                value: device.rssi.unwrap_or(0) as f64,
            }],
            exemplars: vec![],
        };
        let metadata = MetricMetadata {
            r#type: MetricType::Gauge.into(),
            metric_family_name: "bluetooth_rssi".to_owned(),
            help: "The Received Signal Strength Indicator value for the bluetooth device."
                .to_owned(),
            unit: "RSSI".to_owned(),
        };
        (series, metadata)
    }

    fn get_tx_power(&self, device: Device) -> (TimeSeries, MetricMetadata) {
        let mut labels = Labels::from(device.clone());
        labels.0.push(Label {
            name: "__name__".to_owned(),
            value: "bluetooth_tx_power".to_owned(),
        });
        let series = TimeSeries {
            labels: labels.0,
            samples: vec![Sample {
                timestamp: device
                    .timestamp
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_millis() as i64,
                value: device.tx_power.unwrap_or(0) as f64,
            }],
            exemplars: vec![],
        };
        let metadata = MetricMetadata {
            r#type: MetricType::Gauge.into(),
            metric_family_name: "bluetooth_tx_power".to_owned(),
            help: "The TX Power in dBm for the bluetooth device.".to_owned(),
            unit: "dBm".to_owned(),
        };
        (series, metadata)
    }
}

#[async_trait]
impl<C> device_writer::DeviceWriter for RemoteWrite<C>
where
    C: Client + Send + Sync,
{
    async fn write(&mut self, device: Device) {
        let mut req = WriteRequest {
            timeseries: vec![],
            metadata: vec![],
        };

        if device.rssi.is_some() {
            let (ts, md) = self.get_rssi(device.clone());
            req.timeseries.push(ts);
            req.metadata.push(md);
        }

        if device.tx_power.is_some() {
            let (ts, md) = self.get_tx_power(device);
            req.timeseries.push(ts);
            req.metadata.push(md);
        }

        self.client.remote_write(req).await;
    }
}
