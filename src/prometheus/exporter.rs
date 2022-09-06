use std::net::TcpListener;

use ::prometheus::{opts, register_gauge_vec, Encoder, GaugeVec, TextEncoder};
use async_trait::async_trait;
use hyper::{
    header::CONTENT_TYPE,
    service::{make_service_fn, service_fn},
    Body, Request, Response, Server,
};
use lazy_static::lazy_static;

use crate::config;
use crate::device_writer;
use crate::{bluetooth::Device, config::HOSTNAME};

lazy_static! {
    static ref RSSI: GaugeVec = register_gauge_vec!(
        opts!(
            "bluetooth_rssi",
            "The Received Signal Strength Indicator value for the bluetooth device.",
        ),
        &["address", "host", "name"]
    )
    .unwrap();
    static ref TX_POWER: GaugeVec = register_gauge_vec!(
        opts!(
            "bluetooth_tx_power",
            "The transmit power of the bluetooth device.",
        ),
        &["address", "host", "name"]
    )
    .unwrap();
}

async fn handle(_req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let encoder = TextEncoder::new();
    let mut metrics = prometheus::gather();

    let mut buffer = vec![];
    encoder.encode(&metrics, &mut buffer).unwrap();

    let res = Response::builder()
        .status(200)
        .header(CONTENT_TYPE, encoder.format_type())
        .body(Body::from(buffer))
        .unwrap();

    metrics.clear();

    Ok(res)
}

#[derive(Clone, Debug)]
pub struct Exporter {
    config: config::PrometheusExporter,
    listening: bool,
}

impl Exporter {
    pub fn new(config: config::PrometheusExporter) -> Self {
        Self {
            config,
            listening: false,
        }
    }

    pub fn run(&self) {
        let listener = TcpListener::bind(self.config.host.clone()).unwrap();
        let server = Server::from_tcp(listener)
            .unwrap()
            .serve(make_service_fn(|_| async {
                Ok::<_, hyper::Error>(service_fn(handle))
            }));
        tokio::spawn(async move {
            if let Err(err) = server.await {
                log::error!("prometheus exporter server error: {}", err);
            }
        });
    }
}

#[async_trait]
impl device_writer::DeviceWriter for Exporter {
    async fn write(&mut self, device: Device) {
        if !self.listening {
            self.listening = true;
            self.run();
        }

        let host = HOSTNAME.to_string();
        if let Some(rssi) = device.rssi {
            RSSI.with_label_values(&[
                device.address.to_string().as_str(),
                host.as_str(),
                device
                    .name
                    .clone()
                    .unwrap_or_else(|| "".to_string())
                    .as_str(),
            ])
            .set(rssi.into());
        }
        if let Some(tx_power) = device.tx_power {
            TX_POWER
                .with_label_values(&[
                    device.address.to_string().as_str(),
                    host.as_str(),
                    device.name.unwrap_or_else(|| "".to_string()).as_str(),
                ])
                .set(tx_power.into());
        }
    }
}
