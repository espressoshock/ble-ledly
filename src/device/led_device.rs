use btleplug::api::Characteristic;
use btleplug::api::{Peripheral as _, WriteType};
use btleplug::platform::Peripheral;

#[derive(Debug)]
pub struct LedDevice<'p> {
    // BLE localname mapping
    name: String,
    // user-settable alias
    alias: String,
    // underlying BLE Pheripheral
    peripheral: Option<Peripheral>,

    // default communication chars
    write_chars: Option<&'p Characteristic>,
    read_chars: Option<&'p Characteristic>,
}

impl<'p> LedDevice<'p> {
    pub fn new(
        name: &str,
        alias: &str,
        peripheral: Option<Peripheral>,
        write_chars: Option<&'p Characteristic>,
        read_chars: Option<&'p Characteristic>,
    ) -> Self {
        Self {
            name: name.to_string(),
            alias: alias.to_string(),
            peripheral,
            write_chars,
            read_chars,
        }
    }
}
