use crate::communication_protocol::generic_rgb_light::GenericRGBLight;
use crate::device::traits::{Device, Light, RGB};
use crate::errors::BluetoothError;

use btleplug::api::bleuuid::uuid_from_u16;
use btleplug::api::Characteristic;
use btleplug::api::{Peripheral as _, WriteType};
use btleplug::platform::Peripheral;

use std::error::Error;

use async_trait::async_trait;
use uuid::Uuid;

pub const DEFAULT_WRITE_CHARACTERISTIC_UUID: Uuid = uuid_from_u16(0xFFD9);
pub const DEFAULT_WRITE_SERVICE_UUID: Uuid = uuid_from_u16(0xFFD5);

#[derive(Debug)]
pub enum CommandKind {
    HWSpecific,
    Transferrable,
}

#[derive(Debug)]
pub struct Command {
    pub kind: Option<CommandKind>,
    pub value: Option<Vec<u8>>,
}

impl Command {
    pub fn new(value: Option<Vec<u8>>, kind: Option<CommandKind>) -> Self {
        Self { kind, value }
    }
}
#[derive(Debug)]
pub struct LedDevice {
    alias: String,
    peripheral: Option<Peripheral>,

    _last_cmd: Command,

    // .0: Characteristics
    // .1: Default Characteristic
    write_chars: (Vec<Characteristic>, usize),
    read_chars: (Vec<Characteristic>, usize),
}

#[async_trait]
impl Device for LedDevice {
    // Constructor //
    fn new(
        alias: &str,
        peripheral: Peripheral,
        write_chars: Option<Vec<Characteristic>>,
        read_chars: Option<Vec<Characteristic>>,
    ) -> Self {
        Self {
            alias: alias.to_owned(),
            peripheral: Some(peripheral),
            write_chars: (write_chars.unwrap_or(Vec::new()), 0usize),
            read_chars: (read_chars.unwrap_or(Vec::new()), 0usize),
            _last_cmd: Command::new(None, None),
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
    fn default_write_characteristic_uuid(&self) -> &'static Uuid {
        &DEFAULT_WRITE_CHARACTERISTIC_UUID
    }

    //-----------------//
    // Write Raw Bytes //
    //-----------------//
    async fn write_raw(&mut self, raw_bytes: &Vec<u8>) -> Result<(), BluetoothError> {
        self.peripheral
            .as_ref()
            .ok_or(BluetoothError::InvalidPeripheralReference)?
            .write(
                self.write_char(None)
                    .as_ref()
                    .ok_or(BluetoothError::InvalidCharacteristic)?,
                &raw_bytes,
                WriteType::WithoutResponse,
            )
            .await?;

        Ok(())
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

//------------------------//
// Communication Protocol //
//------------------------//
impl GenericRGBLight for LedDevice {}

//-------//
// Light //
//-------//
#[async_trait]
impl Light for LedDevice {
    async fn turn_on(&mut self) {
        self.write_raw(&GenericRGBLight::turn_on(self)).await;
    }
    async fn turn_off(&mut self) {
        self.write_raw(&GenericRGBLight::turn_off(self)).await;
    }
    async fn set_brightness(&mut self, red: u8, green: u8, blue: u8, value: f32) {
        // TODO: update with proper implementation
        self.write_raw(&GenericRGBLight::encode_color(
            self,
            (red as f32 * value) as u8,
            (green as f32 * value) as u8,
            (blue as f32 * value) as u8,
        ))
        .await;
        // -> Result<(), LightControlError> {
        // if value < 0f32 && value > 1f32 {
        //     return Err(LightControlError::InvalidRange(String::from("0.0-1.0")));
        // }
    }
}

//-----//
// RGB //
//-----//
#[async_trait]
impl RGB for LedDevice {
    async fn set_color(&mut self, red: u8, green: u8, blue: u8) {
        self.write_raw(&GenericRGBLight::encode_color(self, red, green, blue))
            .await;
    }
}
