use btleplug::api::{Characteristic, Peripheral as _, Service};
use btleplug::platform::Peripheral;

use btleplug::api::bleuuid::uuid_from_u16;
use uuid::Uuid;

use crate::device::Device;
use std::collections::BTreeSet;

#[derive(Debug)]
pub struct LedDevice<'p> {
    // BLE localname mapping
    name: String,
    // user-settable alias
    alias: String,

    // underlying BLE Pheripheral
    peripheral: Option<Peripheral>,
    // underlying services
    services: Option<BTreeSet<Service>>,

    // default communication chars
    write_char: Option<&'p Characteristic>,
    read_char: Option<&'p Characteristic>,

    // default write char uuid
    write_char_uuid: Uuid,
}
// common to generic ble ic(s)
pub const DEFAULT_WRITE_CHARACTERISTIC_UUID: Uuid = uuid_from_u16(0xFFD9);

impl<'p> Device<'p> for LedDevice<'p> {
    fn new(
        name: &str,
        alias: &str,
        peripheral: Option<Peripheral>,
        write_char: Option<&'p Characteristic>,
        read_char: Option<&'p Characteristic>,
        write_char_uuid: Option<Uuid>,
        services: Option<BTreeSet<Service>>,
    ) -> Self {
        Self {
            name: name.to_string(),
            alias: alias.to_string(),
            peripheral,
            write_char,
            read_char,
            write_char_uuid: write_char_uuid.unwrap_or(DEFAULT_WRITE_CHARACTERISTIC_UUID),
            services,
        }
    }
    // TODO: remove getters and setters
    //--------//
    // Getter //
    //--------//
    fn alias(&self) -> &str {
        &self.alias
    }
    fn name(&self) -> &str {
        &self.name
    }
    fn peripheral(&self) -> Option<&Peripheral> {
        self.peripheral.as_ref()
    }
    fn write_char_uuid(&self) -> &Uuid {
        &self.write_char_uuid
    }
    fn write_char(&self) -> Option<&'p Characteristic> {
        self.write_char
    }
    fn read_char(&self) -> Option<&'p Characteristic> {
        self.read_char
    }
    fn services(&self) -> &BTreeSet<Service> {
        self.services.as_ref().unwrap()
    }
    fn characteristic<'f>(&'f self) -> BTreeSet<&'f Characteristic> {
        self.services()
            .iter()
            .flat_map(|service| service.characteristics.iter())
            .collect()
    }

    //--------//
    // Setter //
    //--------//
    fn set_alias(&mut self, alias: &str) {
        self.alias = alias.to_string();
    }
    fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }
    fn set_peripheral(&mut self, peripheral: Peripheral) {
        self.peripheral = Some(peripheral);
    }
    fn set_write_char_uuid(&mut self, char_uuid: Uuid) {
        self.write_char_uuid = char_uuid;
    }
    fn set_write_char(&mut self, characteristic: &'p Characteristic) {
        self.write_char = Some(characteristic);
    }
    fn set_services(&mut self, services: BTreeSet<Service>) {
        self.services = Some(services);
    }
}
