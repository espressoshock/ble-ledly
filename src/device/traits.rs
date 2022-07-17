use std::error::Error;
use btleplug::api::Characteristic;
use btleplug::platform::Peripheral;


pub trait Light {
    fn turn_on(&self);
    fn turn_off(&self);
    fn set_brightness(&self, value: u8) -> Result<(), Box<dyn Error>>;
}

pub trait RGB : Light {
    fn set_color(&self, red: u8, green: u8, blue: u8) -> Result<(), Box<dyn Error>>;
}
