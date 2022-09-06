# Bluetooth Monitor

## Configuration

Location: `~/.config/bluez-monitor/config.toml`

Example:
```toml
[prometheus.remote_write]
url = "http://127.0.0.1:9090/api/v1/write"

[loki]
url = "http://127.0.0.1:3100/loki/api/v1/push"
```


### Loki
```toml
[loki]
url = "http://127.0.0.1:3100/loki/api/v1/push"
```

### Prometheus

#### Remote Write
```toml
[prometheus.remote_write]
url = "http://127.0.0.1:9090/api/v1/write"
```

#### Exporter
```toml
[prometheus.exporter]
host = "127.0.0.1:9099"
```

## Running the monitor

```
make run
```

[Loki]: https://grafana.com/oss/loki/
[Prometheus]: https://prometheus.io/