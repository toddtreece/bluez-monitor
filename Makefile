all: src/prometheus/proto.rs pi
	cargo build --release

pi: src/prometheus/proto.rs 
	cross build --target aarch64-unknown-linux-gnu --release

src/prometheus/proto.rs:
	cd src/prometheus && buf generate buf.build/prometheus/prometheus && mv prometheus.rs proto.rs

clean:
	rm src/prometheus/proto.rs
	cargo clean

run:
	docker-compose up -d
	cargo run

scp: pi
	scp target/aarch64-unknown-linux-gnu/release/bluez-monitor cam-1.local:~/bluez-monitor
	scp target/aarch64-unknown-linux-gnu/release/bluez-monitor cam-2.local:~/bluez-monitor

.PHONY: all clean run pi scp