use std::io::Read;
use std::path::Path;

use lazy_static::lazy_static;
use serde::Deserialize;

lazy_static! {
    #[derive(Copy, Clone, Debug)]
    pub static ref HOSTNAME: String = {
        match hostname::get() {
            Ok(host) => host.into_string().unwrap_or_else(|_| "".to_owned()),
            Err(_) => "".to_string(),
        }
    };

    #[derive(Clone, Debug)]
    pub static ref CONFIG: Config = {
        let path = match dirs::home_dir() {
            Some(path) => path.join(".config/bluez-monitor/config.toml"),
            None => Path::new("config.toml").to_path_buf(),
        };
        let config_file = std::fs::File::open(path);

        if config_file.is_err() {
            log::error!("failed to open config.toml, using default config");
            return Config::default();
        }

        let mut config_str = String::new();
        config_file.unwrap().read_to_string(&mut config_str).unwrap_or_else(|_| {
            0
        });
        toml::from_str(&config_str).unwrap_or_else(|_| Config::default())
    };
}

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub prometheus: Option<Prometheus>,
    pub loki: Option<Loki>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            prometheus: Some(Prometheus::default()),
            loki: None,
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct Prometheus {
    pub exporter: Option<PrometheusExporter>,
    pub remote_write: Option<PrometheusRemoteWrite>,
}

impl Default for Prometheus {
    fn default() -> Self {
        Self {
            exporter: Some(PrometheusExporter::default()),
            remote_write: None,
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct PrometheusExporter {
    pub host: String,
}

impl Default for PrometheusExporter {
    fn default() -> Self {
        Self {
            host: "0.0.0.0:9099".to_string(),
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct PrometheusRemoteWrite {
    pub url: String,
    pub username: Option<String>,
    pub password: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Loki {
    pub url: String,
    pub username: Option<String>,
    pub password: Option<String>,
}
