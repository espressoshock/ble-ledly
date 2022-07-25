// Define Device Capabilities

use crate::communication_protocol::Protocol;
use crate::device::Device;
use crate::device::Write;
use crate::error::{BluetoothError, CapabilityError};
use async_trait::async_trait;
use crate::capability::color::ColorOption;

//------------//
// Brightness //
//------------//
pub enum BrightnessOption<'e> {
    Level(f32),
    LevelWithColor(f32, &'e ColorOption),
}
#[async_trait]
pub trait Brightness {
    async fn set<'e, P: Protocol + std::marker::Send + std::marker::Sync>(
        &self,
        protocol: &'e P,
        option: &'e BrightnessOption,
    ) -> Result<(), BluetoothError>;
}

//-------------------------//
// Blanket implementations //
//-------------------------//
#[async_trait]
impl<D: Device + std::marker::Sync> Brightness for D {
    // bound type to be transferred across threads
    async fn set<'e, P: Protocol + std::marker::Send + std::marker::Sync>(
        &self,
        protocol: &'e P,
        option: &'e BrightnessOption,
    ) -> Result<(), BluetoothError> {
        self.push(&protocol.brightness(option)[..]).await?;
        Ok(())
    }
}