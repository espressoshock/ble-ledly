use std::error::Error;
use btleplug::api::Characteristic;
use btleplug::platform::Peripheral;

pub trait Device {
    fn new (alias: &str, peripheral: Peripheral, write_chars: Option<Vec<Characteristic>>, read_chars: Option<Vec<Characteristic>>) -> Self;
    fn alias(&self) -> &str;
    fn peripheral(&self) -> &Option<Peripheral>;
    fn write_char(&self, nth: Option<usize>) -> Option<&Characteristic>;
    fn write_raw(&self, raw_bytes: &Vec<u8>);

    fn set_peripheral(&mut self, peripheral: Peripheral);
    fn add_write_characteristic(&mut self, characteristic: Characteristic);
    fn add_read_characteristic(&mut self, characteristic: Characteristic);
}

pub trait Light {
    fn turn_on(&self);
    fn turn_off(&self);
    fn set_brightness(&self, value: u8) -> Result<(), Box<dyn Error>>;
}

pub trait RGB : Light {
    fn set_color(&self, red: u8, green: u8, blue: u8) -> Result<(), Box<dyn Error>>;
}
