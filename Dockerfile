FROM rust:slim

RUN apt-get update && apt-get install -y \
    bluez \
    dbus \
    sudo \
    libdbus-1-dev \
    libdbus-1-3 \
    pkg-config && \
    rustup component add rustfmt

COPY ./config/bluezuser.conf /etc/dbus-1/system.d/

RUN useradd -m bluez && \ 
    adduser bluez sudo && \
    passwd -d bluez && \
    mkdir -p /home/bluez/monitor/target && \
    chown -R bluez /home/bluez/monitor
USER bluez

WORKDIR /home/bluez/monitor

ENV CARGO_HOME=/home/bluez/.cargo

RUN cargo install cargo-watch

COPY ./config/bluezuser.conf /etc/dbus-1/system.d/
COPY Cargo.toml ./Cargo.toml
COPY Cargo.lock ./Cargo.lock
COPY src ./src
COPY entrypoint.sh ./entrypoint.sh

VOLUME [ "/home/bluez/monitor/target" ]
VOLUME [ "/home/bluez/cargo" ]

CMD ./entrypoint.sh