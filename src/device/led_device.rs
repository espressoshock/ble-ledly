use btleplug::api::{Characteristic, Peripheral as _};
use btleplug::platform::Peripheral;

use uuid::Uuid;

use std::fmt;

use crate::device::Device;

#[derive(Debug)]
pub struct LedDevice {
    // BLE localname mapping
    pub name: String,
    // user-settable alias
    pub alias: String,

    // underlying BLE Pheripheral
    peripheral: Option<Peripheral>,

    // default communication chars
    write_char: Option<Characteristic>,
    read_char: Option<Characteristic>,
}

impl Device for LedDevice {
    fn new(
        name: &str,
        alias: &str,
        peripheral: Option<Peripheral>,
        write_char: Option<Characteristic>,
        read_char: Option<Characteristic>,
    ) -> Self {
        Self {
            name: name.to_string(),
            alias: alias.to_string(),
            peripheral,
            write_char: write_char.clone(),
            read_char: read_char.clone(),
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
    fn address(&self) -> Option<String> {
        if let Some(peripheral) = self.peripheral.as_ref() {
            return Some(peripheral.address().to_string());
        }
        None
    }
    fn peripheral(&self) -> Option<&Peripheral> {
        self.peripheral.as_ref()
    }
    fn write_char(&self) -> Option<&Characteristic> {
        self.write_char.as_ref()
    }
    fn read_char(&self) -> Option<&Characteristic> {
        self.read_char.as_ref()
    }
    fn default_write_characteristic_uuid(&self) -> Uuid {
        // DEFAULT_WRITE_CHARACTERISTIC_UUID.clone()
        let my_uuid = Uuid::new_v4();
        my_uuid
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
    fn set_write_char(&mut self, characteristic: &Characteristic) {
        self.write_char = Some(characteristic.clone());
    }
}
//--------------//
// Display impl //
//--------------//
impl fmt::Display for LedDevice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} ({})",
            self.name(),
            self.address().unwrap_or(String::from("-"))
        )
    }
}
