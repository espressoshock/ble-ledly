use crate::errors::LightControlError;

use btleplug::api::Characteristic;
use btleplug::platform::Peripheral;
use uuid::Uuid;

use async_trait::async_trait;

#[async_trait]
pub trait Device {
    fn new(
        alias: &str,
        peripheral: Peripheral,
        write_chars: Option<Vec<Characteristic>>,
        read_chars: Option<Vec<Characteristic>>,
    ) -> Self;
    fn alias(&self) -> &str;
    fn peripheral(&self) -> &Option<Peripheral>;
    fn write_char(&self, nth: Option<usize>) -> Option<&Characteristic>;
    fn default_write_characteristic_uuid(&self) -> &'static Uuid;
    async fn write_raw(&self, raw_bytes: &Vec<u8>);

    fn set_peripheral(&mut self, peripheral: Peripheral);
    fn add_write_characteristic(&mut self, characteristic: Characteristic);
    fn add_read_characteristic(&mut self, characteristic: Characteristic);
}

#[async_trait]
pub trait Light {
    async fn turn_on(&self);
    async fn turn_off(&self);
    async fn set_brightness(&self, value: f32) -> Result<(), LightControlError>;
}

#[async_trait]
pub trait RGB: Light {
    async fn set_color(&self, red: u8, green: u8, blue: u8);
}
