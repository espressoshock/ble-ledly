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
    write_char: Option<&'p Characteristic>,
    read_char: Option<&'p Characteristic>,
}

impl<'p> LedDevice<'p> {
    pub fn new(
        name: &str,
        alias: &str,
        peripheral: Option<Peripheral>,
        write_char: Option<&'p Characteristic>,
        read_char: Option<&'p Characteristic>,
    ) -> Self {
        Self {
            name: name.to_string(),
            alias: alias.to_string(),
            peripheral,
            write_char,
            read_char,
        }
    }
    //--------//
    // Getter //
    //--------//
    fn alias(&self) -> &str {
        &self.alias
    }
    fn name(&self) -> &str {
        &self.name
    }
    fn peripheral(&self) -> &Option<Peripheral> {
        &self.peripheral
    }
    fn write_char(&self) -> Option<&'p Characteristic> {
        self.write_char
    }
    fn read_char(&self) -> Option<&'p Characteristic> {
        self.read_char
    }
}
