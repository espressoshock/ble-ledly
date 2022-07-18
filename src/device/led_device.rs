use std::fmt::Result;

use crate::communication_protocol::generic_rgb_light::GenericRGBLight;
use crate::device::traits::{Device, Light, RGB};
use crate::errors::{BluetoothError, LightControlError};

use btleplug::api::bleuuid::uuid_from_u16;
use btleplug::api::Characteristic;
use btleplug::api::{Peripheral as _, WriteType};
use btleplug::platform::Peripheral;

use async_trait::async_trait;
use uuid::Uuid;

pub const DEFAULT_WRITE_CHARACTERISTIC_UUID: Uuid = uuid_from_u16(0xFFD9);
pub const DEFAULT_WRITE_SERVICE_UUID: Uuid = uuid_from_u16(0xFFD5);

#[derive(Debug)]
enum CommandKind {
    HWSpecific,
    Transferrable,
}

#[derive(Debug)]
struct Command {
    pub kind: Option<CommandKind>,
    pub value: Option<Vec<u8>>,
}

impl Command {
    pub fn new() -> Self {
        Self {
            kind: None,
            value: None,
        }
    }
}
#[derive(Debug)]
pub struct LedDevice {
    alias: String,
    peripheral: Option<Peripheral>,

    last_cmd: Command,

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
            last_cmd: Command::new(),
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
    async fn write_raw(&self, raw_bytes: &Vec<u8>) {
        // TODO: implement error handling
        let write = self
            .peripheral
            .as_ref()
            .unwrap()
            .write(
                self.write_char(None).as_ref().unwrap(),
                &raw_bytes,
                WriteType::WithoutResponse,
            )
            .await;
        match write {
            Ok(_) => println!("Raw write successfull"),
            Err(_) => println!("Error during raw write"),
        }
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
    async fn turn_on(&self) {
        self.write_raw(&GenericRGBLight::turn_on(self)).await;
    }
    async fn turn_off(&self) {
        self.write_raw(&GenericRGBLight::turn_off(self)).await;
    }
    async fn set_brightness(&self, value: f32) {
        // -> Result<(), LightControlError> {
        // if value < 0f32 && value > 1f32 {
        //     return Err(LightControlError::InvalidRange(String::from("0.0-1.0")));
        // }
        todo!();
    }
}

//-----//
// RGB //
//-----//
#[async_trait]
impl RGB for LedDevice {
    async fn set_color(&self, red: u8, green: u8, blue: u8) {
        self.write_raw(&GenericRGBLight::encode_color(self, red, green, blue))
            .await;
    }
}
