use futures_util::{pin_mut, stream::StreamExt};

mod bluetooth;
mod config;
mod device_writer;
mod loki;
mod prometheus;

use crate::bluetooth::discover;
use crate::device_writer::DeviceWriters;

#[tokio::main]
async fn main() {
    env_logger::init();

    let mut writers: Vec<DeviceWriters<prometheus::DefaultClient, loki::DefaultClient>> = vec![];

    if let Some(prom_config) = config::CONFIG.prometheus.clone() {
        if let Some(exporter) = prom_config.exporter {
            log::info!("Enabling Prometheus exporter: http://{}", exporter.host);
            let exporter = device_writer::DeviceWriters::prometheus_exporter(exporter);
            writers.push(exporter);
        }

        if let Some(remote_write) = prom_config.remote_write {
            log::info!("Enabling Prometheus remote write");
            let client = prometheus::DefaultClient::new(remote_write);
            writers.push(device_writer::DeviceWriters::prometheus_remote_write(
                client,
            ));
        }
    }

    if let Some(loki_config) = config::CONFIG.loki.clone() {
        log::info!("Enabling Loki push");
        let client = loki::DefaultClient::new(loki_config);
        writers.push(device_writer::DeviceWriters::loki(client));
    }

    if writers.is_empty() {
        log::error!("No writers enabled");
        return;
    }

    let devices = discover();
    pin_mut!(devices);

    loop {
        let device = devices.next().await;
        match device {
            Some(Ok(device)) => {
                writers.iter().for_each(|writer| {
                    let device = device.clone();
                    let mut writer = writer.clone();
                    tokio::spawn(async move {
                        writer.write(device.clone()).await;
                        log::trace!("Wrote device: {:?}", device);
                    });
                });
            }
            Some(Err(e)) => log::error!("Discovery error {:?}", e),
            None => log::trace!("no devices found"),
        }
    }
}
