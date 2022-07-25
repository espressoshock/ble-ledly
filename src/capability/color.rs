// Define Device Capabilities

use crate::communication_protocol::Protocol;
use crate::device::Device;
use crate::device::Write;
use crate::error::{BluetoothError, CapabilityError};
use async_trait::async_trait;

//-------//
// Color //
//-------//
pub enum ColorOption {
    RGB(u8, u8, u8),
}
#[async_trait]
pub trait Color {
    async fn set<P: Protocol + std::marker::Send + std::marker::Sync>(
        &self,
        protocol: P,
        option: ColorOption,
    ) -> Result<(), BluetoothError>;
}

//-------------------------//
// Blanket implementations //
//-------------------------//
#[async_trait]
impl<D: Device + std::marker::Sync> Color for D {
    // bound type to be transferred across threads
    async fn set<P: Protocol + std::marker::Send + std::marker::Sync>(
        &self,
        protocol: P,
        option: ColorOption,
    ) -> Result<(), BluetoothError> {
        self.push(&protocol.color(option)[..]).await?;
        Ok(())
    }
}
