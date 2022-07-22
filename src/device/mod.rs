use crate::error::BluetoothError;

use btleplug::api::Characteristic;
use btleplug::api::{Peripheral as _, WriteType};
use btleplug::platform::Peripheral;

use uuid::Uuid;

use async_trait::async_trait;

pub mod led_device;

pub trait Device<'p> {
    fn new(
        name: &str,
        alias: &str,
        peripheral: Option<Peripheral>,
        write_char: Option<&'p Characteristic>,
        read_char: Option<&'p Characteristic>,
        write_char_uuid: Option<Uuid>,
    ) -> Self;
    //--------//
    // Getter //
    //--------//
    fn alias(&self) -> &str;
    fn name(&self) -> &str;
    fn peripheral(&self) -> &Option<Peripheral>;
    fn write_char_uuid(&self) -> &Uuid;
    fn write_char(&self) -> Option<&'p Characteristic>;
    fn read_char(&self) -> Option<&'p Characteristic>;

    //--------//
    // Setter //
    //--------//
    fn set_alias(&mut self, alias: &str);
    fn set_name(&mut self, name: &str);
    fn set_peripheral(&mut self, peripheral: Peripheral);
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

#[async_trait]
impl<'p, D: Device<'p> + std::marker::Sync> Write for D {
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
