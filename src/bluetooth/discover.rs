use bluer::DeviceProperty;
use std::collections::HashMap;
use std::collections::HashSet;
use std::time::SystemTime;

use async_stream::try_stream;
use bluer::{AdapterEvent, Address, AddressType, Modalias, Uuid};
use futures_util::stream::Stream;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DiscoveryError {
    #[error("bluetooth error: {0}")]
    Bluetooth(#[from] bluer::Error),
    #[error("system time error: {0}")]
    SystemTime(#[from] std::time::SystemTimeError),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Device {
    pub timestamp: SystemTime,
    pub address: Address,
    pub address_type: Option<AddressType>,
    pub name: Option<String>,
    pub uuids: HashSet<Uuid>,
    pub class: Option<u32>,
    pub appearance: Option<u16>,
    pub paired: bool,
    pub connected: bool,
    pub trusted: bool,
    pub blocked: bool,
    pub wake_allowed: bool,
    pub legacy_pairing: bool,
    pub alias: Option<String>,
    pub modalias: Option<Modalias>,
    pub icon: Option<String>,
    pub manufacturer_data: HashMap<u16, Vec<u8>>,
    pub advertising_data: HashMap<u8, Vec<u8>>,
    pub advertising_flags: Vec<u8>,
    pub tx_power: Option<i16>,
    pub rssi: Option<i16>,
    pub service_data: HashMap<Uuid, Vec<u8>>,
    pub services_resolved: bool,
}

impl Default for Device {
    fn default() -> Self {
        Device {
            timestamp: SystemTime::now(),
            address: Address::default(),
            address_type: None,
            name: None,
            uuids: HashSet::new(),
            class: None,
            appearance: None,
            paired: false,
            connected: false,
            trusted: false,
            blocked: false,
            wake_allowed: false,
            legacy_pairing: false,
            alias: None,
            modalias: None,
            icon: None,
            manufacturer_data: HashMap::new(),
            advertising_data: HashMap::new(),
            advertising_flags: vec![],
            tx_power: None,
            rssi: None,
            service_data: HashMap::new(),
            services_resolved: false,
        }
    }
}

impl From<Vec<DeviceProperty>> for Device {
    fn from(properties: Vec<DeviceProperty>) -> Self {
        let mut device = Device::default();
        for property in properties {
            match property {
                DeviceProperty::Name(name) => device.name = Some(name),
                DeviceProperty::AddressType(address_type) => {
                    device.address_type = Some(address_type)
                }
                DeviceProperty::Class(class) => device.class = Some(class),
                DeviceProperty::Appearance(appearance) => device.appearance = Some(appearance),
                DeviceProperty::Blocked(blocked) => device.blocked = blocked,
                DeviceProperty::Paired(paired) => device.paired = paired,
                DeviceProperty::Connected(connected) => device.connected = connected,
                DeviceProperty::Trusted(trusted) => device.trusted = trusted,
                DeviceProperty::Alias(alias) => device.alias = Some(alias),
                DeviceProperty::Modalias(modalias) => device.modalias = Some(modalias),
                DeviceProperty::Icon(icon) => device.icon = Some(icon),
                DeviceProperty::Uuids(uuids) => device.uuids = uuids,
                DeviceProperty::ManufacturerData(manufacturer_data) => {
                    device.manufacturer_data = manufacturer_data;
                }
                DeviceProperty::AdvertisingData(advertising_data) => {
                    device.advertising_data = advertising_data;
                }
                DeviceProperty::AdvertisingFlags(advertising_flags) => {
                    device.advertising_flags = advertising_flags
                }
                DeviceProperty::Rssi(rssi) => device.rssi = Some(rssi),
                DeviceProperty::TxPower(tx_power) => device.tx_power = Some(tx_power),
                DeviceProperty::WakeAllowed(wake_allowed) => device.wake_allowed = wake_allowed,
                DeviceProperty::LegacyPairing(legacy_pairing) => {
                    device.legacy_pairing = legacy_pairing
                }
                DeviceProperty::ServicesResolved(services_resolved) => {
                    device.services_resolved = services_resolved
                }
                DeviceProperty::ServiceData(service_data) => {
                    device.service_data = service_data;
                }
                _ => {}
            }
        }
        device
    }
}

pub fn discover() -> impl Stream<Item = Result<Device, DiscoveryError>> {
    try_stream! {
        let session = bluer::Session::new().await?;
        let adapter = session.default_adapter().await?;
        adapter.set_powered(true).await?;
        let device_events = adapter.discover_devices_with_changes().await?;
        for await device_event in device_events {
            if let AdapterEvent::DeviceAdded(addr) = device_event {
                let device = adapter.device(addr)?;
                let properties = device.all_properties().await?;
                let mut result = Device::from(properties);
                result.address = device.address();
                yield result;
            }
        }
    }
}
