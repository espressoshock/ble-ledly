use crate::device::traits::{Light, RGB, Device};

use std::error::Error;

use btleplug::api::Characteristic;
use btleplug::api::{
   Peripheral as _, WriteType,
};
use btleplug::platform::{Peripheral};


#[derive(Debug)]
pub struct LedDevice{
    alias: String,

    peripheral: Option<Peripheral>,

    // .0: Characteristics
    // .1: Default Characteristic
    write_chars: (Vec<Characteristic>, usize),
    read_chars: (Vec<Characteristic>, usize),
}

impl Device for LedDevice {
    // Constructor //
    fn new (alias: &str, peripheral: Peripheral, write_chars: Option<Vec<Characteristic>>, read_chars: Option<Vec<Characteristic>>) -> Self {
        Self {
            alias: alias.to_owned(),
            peripheral: Some(peripheral),
            write_chars: (write_chars.unwrap_or(Vec::new()), 0usize),
            read_chars: (read_chars.unwrap_or(Vec::new()), 0usize),
        }
    }
    //--------//
    // Getter //
    //--------//
    fn alias(&self) -> &str {
        &self.alias
    }
    fn peripheral(&self) -> &Option<Peripheral> {
        &self.peripheral
    }
    fn write_char(&self, nth: Option<usize>) -> Option<&Characteristic> {
        (&self).write_chars.0.get(nth.unwrap_or(self.write_chars.1))
    }

    fn write_raw(&self, raw_bytes: &Vec<u8>) {
        self.peripheral.as_ref().expect("Error unpacking peripheral").write(self.write_char(None).as_ref().unwrap(), raw_bytes, WriteType::WithoutResponse);
    }

    //--------//
    // Setter //
    //--------//
    fn set_peripheral(&mut self, peripheral: Peripheral) {
        self.peripheral = Some(peripheral);
    }
    fn add_write_characteristic(&mut self, characteristic: Characteristic) {
        self.write_chars.0.push(characteristic);
    }
    fn add_read_characteristic(&mut self, characteristic: Characteristic) {
        self.read_chars.0.push(characteristic);
    }
}


//-------//
// Light //
//-------//
impl Light for LedDevice {
    fn turn_on(&self) {todo!();}
    fn turn_off(&self){todo!();}
    fn set_brightness(&self, value: u8) -> Result<(), Box<dyn Error>>{todo!();}
}
//-----//
// RGB //
//-----//
impl RGB for LedDevice {
    fn set_color(&self, red: u8, green: u8, blue: u8) -> Result<(), Box<dyn Error>>{todo!();}
}
