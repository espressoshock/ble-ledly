use crate::communication_protocol::Protocol;
use crate::device::Device;
use crate::device::Write;
use crate::error::{BluetoothError};
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
        device: &Self,
        protocol: &'e P,
        option: &'e LightOption,
    ) -> Result<(), BluetoothError>;


    // -------------------------------//
    // Syntactic sugar /////////////////
    // more idiomatic syntactic sugar //
    // -------------------------------//
    async fn turn_on<'e, P: Protocol + std::marker::Send + std::marker::Sync>(
        &self,
        protocol: &'e P,
    ) -> Result<(), BluetoothError>;
    async fn turn_off<'e, P: Protocol + std::marker::Send + std::marker::Sync>(
        &self,
        protocol: &'e P,
    ) -> Result<(), BluetoothError>;
}

//-------------------------//
// Blanket implementations //
//-------------------------//
#[async_trait]
impl<D: Device + std::marker::Sync> Light for D {
    // bound type to be transferred across threads
    async fn set<'e, P: Protocol + std::marker::Send + std::marker::Sync>(
        device: &Self,
        protocol: &'e P,
        option: &'e LightOption,
    ) -> Result<(), BluetoothError> {
        device.push(&protocol.light(option)[..]).await?;
        Ok(())
    }


    // -------------------------------//
    // Syntactic sugar /////////////////
    // more idiomatic syntactic sugar //
    // -------------------------------//
    async fn turn_on<'e, P: Protocol + std::marker::Send + std::marker::Sync>(
        &self,
        protocol: &'e P,
    ) -> Result<(), BluetoothError>{
        self.push(&protocol.light(&LightOption::On)[..]).await?;
        Ok(())
    }
    async fn turn_off<'e, P: Protocol + std::marker::Send + std::marker::Sync>(
        &self,
        protocol: &'e P,
    ) -> Result<(), BluetoothError>{
        self.push(&protocol.light(&LightOption::Off)[..]).await?;
        Ok(())
    }
}
