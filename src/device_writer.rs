use async_trait::async_trait;

use crate::bluetooth::Device;
use crate::config;
use crate::{loki, prometheus};

#[async_trait]
pub trait DeviceWriter {
    async fn write(&mut self, device: Device);
}

#[derive(Clone)]
pub enum DeviceWriters<PC, LC>
where
    PC: prometheus::Client + Send + Sync + Clone,
    LC: loki::Client + Send + Sync + Clone,
{
    PrometheusExporter(prometheus::Exporter),
    PrometheusRemoteWrite(prometheus::RemoteWrite<PC>),
    Loki(loki::Push<LC>),
}

impl<PC, LC> DeviceWriters<PC, LC>
where
    PC: prometheus::Client + Send + Sync + Clone,
    LC: loki::Client + Send + Sync + Clone,
{
    pub fn prometheus_exporter(config: config::PrometheusExporter) -> Self {
        DeviceWriters::PrometheusExporter(prometheus::Exporter::new(config))
    }

    pub fn prometheus_remote_write(client: PC) -> Self {
        Self::PrometheusRemoteWrite(prometheus::RemoteWrite::new(client))
    }

    pub fn loki(client: LC) -> Self {
        Self::Loki(loki::Push::new(client))
    }

    pub async fn write(&mut self, device: Device) {
        match self {
            Self::PrometheusRemoteWrite(writer) => writer.write(device).await,
            Self::Loki(writer) => writer.write(device).await,
            Self::PrometheusExporter(writer) => writer.write(device).await,
        }
    }
}
