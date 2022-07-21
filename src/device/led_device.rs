use crate::communication_protocol::generic_rgb_light::{
    AnimationSpeedSetting, GenericRGBLight, PulsatingColor,
};
use crate::device::traits::{Device, Light, RGB};
use crate::errors::BluetoothError;
use core::fmt;

use btleplug::api::bleuuid::uuid_from_u16;
use btleplug::api::Characteristic;
use btleplug::api::{Peripheral as _, WriteType};
use btleplug::platform::Peripheral;

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
    name: String,
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
        name: &str,
        alias: &str,
        peripheral: Peripheral,
        write_chars: Option<Vec<Characteristic>>,
        read_chars: Option<Vec<Characteristic>>,
    ) -> Self {
        Self {
            name: name.to_owned(),
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
    fn name(&self) -> &str {
        &self.name
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
        // TODO: implement error handling
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

    //------------//
    // Disconnect //
    //------------//
    async fn disconnect(&self) -> Result<(), BluetoothError>{
        self.peripheral
            .as_ref()
            .ok_or(BluetoothError::InvalidPeripheralReference)?
            .disconnect()
            .await?;
        Ok(())
    }

    //--------//
    // Setter //
    //--------//
    fn set_peripheral(&mut self, peripheral: Peripheral) {
        self.peripheral = Some(peripheral);
    }
    fn set_alias(&mut self, alias: &str) {
        self.alias = alias.to_owned();
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
    // TODO: Implement better error handling
    async fn turn_on(&mut self) {
        self.write_raw(&GenericRGBLight::turn_on(self))
            .await
            .unwrap_or_else(|err| println!("Error turning light on: {:?}", err));
    }
    async fn turn_off(&mut self) {
        self.write_raw(&GenericRGBLight::turn_off(self))
            .await
            .unwrap_or_else(|err| println!("Error turning light off: {:?}", err));
    }
    async fn set_brightness(&mut self, red: u8, green: u8, blue: u8, value: f32) {
        // TODO: update with proper implementation
        self.write_raw(&GenericRGBLight::encode_color(
            self,
            (red as f32 * value) as u8,
            (green as f32 * value) as u8,
            (blue as f32 * value) as u8,
        ))
        .await
        .unwrap_or_else(|err| println!("Error setting light brightness: {:?}", err));
    }
    async fn pulsating(&mut self, color: &PulsatingColor, speed_setting: &AnimationSpeedSetting) {
        self.write_raw(&GenericRGBLight::pulsating(self, color, speed_setting))
            .await
            .unwrap_or_else(|err| println!("Error setting light brightness: {:?}", err));
    }
}

//-----//
// RGB //
//-----//
#[async_trait]
impl RGB for LedDevice {
    async fn set_color(&mut self, red: u8, green: u8, blue: u8) {
        self.write_raw(&GenericRGBLight::encode_color(self, red, green, blue))
            .await
            .unwrap_or_else(|err| println!("Error setting light brightness: {:?}", err));
    }
}

//---------//
// Display //
//---------//
impl fmt::Display for LedDevice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(peripheral) = self.peripheral.as_ref() {
            return write!(f, "{} ({})", self.name, peripheral.address());
        }
        write!(f, "{} (Unknown)", self.name)
    }
}
