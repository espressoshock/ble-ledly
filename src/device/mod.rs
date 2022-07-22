use crate::error::BluetoothError;

use btleplug::api::Characteristic;
use btleplug::api::{Peripheral as _, WriteType};
use btleplug::platform::Peripheral;

use async_trait::async_trait;

pub mod led_device;

pub trait Device<'p> {
    fn peripheral(&self) -> &Option<Peripheral>;
    fn write_char(&self) -> Option<&'p Characteristic>;
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
impl<'a, D: Device<'a> + std::marker::Sync> Disconnect for D {
    async fn leave(&self) -> Result<(), BluetoothError> {
        self.peripheral()
            .as_ref()
            .ok_or(BluetoothError::InvalidPeripheralReference)?
            .disconnect()
            .await?;
        Ok(())
    }
}

