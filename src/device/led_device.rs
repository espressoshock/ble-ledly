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
    //--------//
    // Getter //
    //--------//
    /// Provides access to __user-settable__
    /// device alias
    fn alias(&self) -> &str {
        &self.alias
    }
    /// Provides access to the device local_name
    fn name(&self) -> &str {
        &self.name
    }
    /// Provides access to _BLE-specific_ MAC device address
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
        unimplemented!()
    }

    //--------//
    // Setter //
    //--------//
    /// Allows to set __user-settable__
    /// device alias
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
