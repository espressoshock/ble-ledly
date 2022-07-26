use crate::device::led_device::CharactericKind;
use crate::error::BluetoothError;

use btleplug::api::Characteristic;
use btleplug::api::{Peripheral as _, WriteType};
use btleplug::platform::Peripheral;

use uuid::Uuid;

use async_trait::async_trait;
use std::fmt;
use enumflags2::BitFlags;

pub mod led_device;

pub trait Device: fmt::Display {
    fn new(
        name: &str,
        alias: &str,
        peripheral: Option<Peripheral>,
        write_char: Option<Characteristic>,
        read_char: Option<Characteristic>,
        write_char_uuid: Option<Uuid>,
    ) -> Self;
    //--------//
    // Getter //
    //--------//
    fn alias(&self) -> &str;
    fn name(&self) -> &str;
    fn address(&self) -> Option<String>;
    fn peripheral(&self) -> Option<&Peripheral>;
    fn write_char_uuid(&self) -> &Uuid;
    fn write_char(&self) -> Option<&Characteristic>;
    fn read_char(&self) -> Option<&Characteristic>;
    fn default_write_characteristic_uuid(&self) -> Uuid;
    fn characteristics(&self) -> Option<Vec<Characteristic>>;
    fn characteristics_by_type(
        &self,
        kinds: BitFlags<CharactericKind>,
    ) -> Option<Vec<Characteristic>>;

    //--------//
    // Setter //
    //--------//
    fn set_alias(&mut self, alias: &str);
    fn set_name(&mut self, name: &str);
    fn set_peripheral(&mut self, peripheral: Peripheral);
    fn set_write_char(&mut self, characteristic: &Characteristic);
    fn set_write_char_uuid(&mut self, char_uuid: Uuid);
}

#[async_trait]
pub trait Disconnect {
    async fn leave(&self) -> Result<(), BluetoothError>;
}
pub trait Connect {}
#[async_trait]
pub trait Write {
    async fn push(&self, raw_bytes: &[u8]) -> Result<(), BluetoothError>;
}

//-------------------------//
// Blanket implementations //
//-------------------------//
#[async_trait]
impl<D: Device + std::marker::Sync> Disconnect for D {
    async fn leave(&self) -> Result<(), BluetoothError> {
        self.peripheral()
            .as_ref()
            .ok_or(BluetoothError::InvalidPeripheralReference)?
            .disconnect()
            .await?;
        Ok(())
    }
}

#[async_trait]
impl<D: Device + std::marker::Sync> Write for D {
    async fn push(&self, raw_bytes: &[u8]) -> Result<(), BluetoothError> {
        //TODO: Implement different WriteType(s)
        self.peripheral()
            .as_ref()
            .ok_or(BluetoothError::InvalidPeripheralReference)?
            .write(
                self.write_char()
                    .ok_or(BluetoothError::InvalidCharacteristic)?,
                raw_bytes,
                WriteType::WithoutResponse,
            )
            .await?;

        Ok(())
    }
}
