// Define Device Capabilities

use crate::communication_protocol::Protocol;
use crate::device::Device;
use crate::device::Write;
use crate::error::{BluetoothError, CapabilityError};
use async_trait::async_trait;

//-------//
// Light //
//-------//
pub enum LightOption {
    On,
    Off,
}
#[async_trait]
pub trait Light {
    async fn set<'e, P: Protocol + std::marker::Send + std::marker::Sync>(
        &self,
        protocol: &'e P,
        option: &'e LightOption,
    ) -> Result<(), BluetoothError>;
}

//-------------------------//
// Blanket implementations //
//-------------------------//
#[async_trait]
impl<D: Device + std::marker::Sync> Light for D {
    // bound type to be transferred across threads
    async fn set<'e, P: Protocol + std::marker::Send + std::marker::Sync>(
        &self,
        protocol: &'e P,
        option: &'e LightOption,
    ) -> Result<(), BluetoothError> {
        self.push(&protocol.light(option)[..]).await?;
        Ok(())
    }
}
