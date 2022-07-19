use btleplug::api::Characteristic;
use btleplug::platform::Peripheral;
use uuid::Uuid;

use async_trait::async_trait;

use crate::communication_protocol::generic_rgb_light::{AnimationSpeedSetting, PulsatingColor};
use crate::errors::BluetoothError;

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
    // async fn write_raw(&self, raw_bytes: &Vec<u8>);
    async fn write_raw(&mut self, raw_bytes: &Vec<u8>) -> Result<(), BluetoothError>;

    fn set_peripheral(&mut self, peripheral: Peripheral);
    fn add_write_characteristic(&mut self, characteristic: Characteristic);
    fn add_read_characteristic(&mut self, characteristic: Characteristic);
}

#[async_trait]
pub trait Light {
    async fn turn_on(&mut self);
    async fn turn_off(&mut self);
    async fn set_brightness(&mut self, red: u8, green: u8, blue: u8, value: f32);
    async fn pulsating(&mut self, color: &PulsatingColor, speed_setting: &AnimationSpeedSetting);
}

#[async_trait]
pub trait RGB: Light {
    async fn set_color(&mut self, red: u8, green: u8, blue: u8);
}
