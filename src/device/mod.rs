use crate::error::BluetoothError;

use btleplug::api::Characteristic;
use btleplug::api::{Peripheral as _, WriteType};
use btleplug::platform::Peripheral;

use uuid::Uuid;

use async_trait::async_trait;
use enumflags2::{bitflags, BitFlags};
use std::fmt;

pub mod led_device;

const BT_BASE_UUID: u128 = 0x00000000_0000_1000_8000_00805f9b34fb;

// wrapper for native ble charprops
#[bitflags]
#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum OpKind {
    Broadcast = 0x01,
    Read = 0x02,
    WriteWithoutResponse = 0x04,
    Write = 0x08,
    Notify = 0x10,
    Indicate = 0x20,
    AuthenticatedSignedWrites = 0x40,
    ExtendedProperties = 0x80,
}

pub enum UuidKind {
    Uuid(Uuid),
    Uuid16(u16),
    Uuid32(u32),
    Uuid128(u128),
}

pub enum CharKind {
    Read,
    Write,
}

pub trait Device: fmt::Display {
    fn new(
        name: &str,
        alias: &str,
        peripheral: Option<Peripheral>,
        write_char: Option<Characteristic>,
        read_char: Option<Characteristic>,
    ) -> Self;
    //--------//
    // Getter //
    //--------//
    fn alias(&self) -> &str;
    fn name(&self) -> &str;
    fn address(&self) -> Option<String>;
    fn peripheral(&self) -> Option<&Peripheral>;
    fn write_char(&self) -> Option<&Characteristic>;
    fn read_char(&self) -> Option<&Characteristic>;
    fn default_write_characteristic_uuid(&self) -> Uuid;
    fn characteristics(&self) -> Option<Vec<Characteristic>> {
        if let Some(peripheral) = self.peripheral().as_ref() {
            return Some(
                peripheral
                    .characteristics()
                    .into_iter()
                    .collect::<Vec<Characteristic>>(),
            );
        }
        None
    }
    fn characteristics_by_type(&self, kinds: BitFlags<OpKind>) -> Option<Vec<Characteristic>> {
        if let Some(chars) = self.characteristics() {
            return Some(
                chars
                    .into_iter()
                    .filter(|c| c.properties.bits() == kinds.bits())
                    .collect(),
            );
        }
        None
    }

    //--------//
    // Setter //
    //--------//
    fn set_alias(&mut self, alias: &str);
    fn set_name(&mut self, name: &str);
    fn set_peripheral(&mut self, peripheral: Peripheral);
    fn set_write_char(&mut self, characteristic: &Characteristic);
    fn set_char(
        &mut self,
        char_kind: &CharKind,
        uuid_kind: &UuidKind,
    ) -> Result<(), BluetoothError> {
        match char_kind {
            CharKind::Write => match uuid_kind {
                UuidKind::Uuid(uuid) => self.set_char_with_uuid(char_kind, &uuid),
                UuidKind::Uuid128(uuid) => {
                    self.set_char_with_uuid(char_kind, &Uuid::from_u128(*uuid))
                }
                UuidKind::Uuid32(uuid) => self.set_char_with_u32(char_kind, *uuid),
                UuidKind::Uuid16(uuid) => self.set_char_with_u16(char_kind, *uuid),
            },
            CharKind::Read => unimplemented!(),
        }
    }
    fn set_char_with_uuid(
        &mut self,
        char_kind: &CharKind,
        uuid: &Uuid,
    ) -> Result<(), BluetoothError> {
        let char = self
            .peripheral()
            .as_ref()
            .ok_or(BluetoothError::InvalidPeripheralReference)?
            .characteristics()
            .into_iter()
            .find(|c| c.uuid.as_u128() == uuid.as_u128())
            .ok_or(BluetoothError::NotFoundTargetCharacteristic)?;
        match char_kind {
            CharKind::Write => self.set_write_char(&char),
            CharKind::Read => unimplemented!(),
        }
        Ok(())
    }
    fn set_char_with_u16(&mut self, char_kind: &CharKind, u16: u16) -> Result<(), BluetoothError> {
        self.set_char_with_u32(char_kind, u16 as u32) // extend it to 32 bits
    }
    fn set_char_with_u32(&mut self, char_kind: &CharKind, u32: u32) -> Result<(), BluetoothError> {
        let uuid = Uuid::from_u128(BT_BASE_UUID | ((u32 as u128) << 96));
        self.set_char_with_uuid(char_kind, &uuid)
    }
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
