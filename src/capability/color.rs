use crate::communication_protocol::Protocol;
use crate::device::Device;
use crate::device::Write;
use crate::error::{BluetoothError};
use async_trait::async_trait;

//-------//
// Color //
//-------//
pub enum ColorOption {
    RGB(u8, u8, u8),
}
#[async_trait]
pub trait Color {
    async fn set<'e, P: Protocol + std::marker::Send + std::marker::Sync>(
        device: &Self,
        protocol: &'e P,
        option: &'e ColorOption,
    ) -> Result<(), BluetoothError>;

    // -------------------------------//
    // Syntactic sugar /////////////////
    // more idiomatic syntactic sugar //
    // -------------------------------//
    async fn color<'e, P: Protocol + std::marker::Send + std::marker::Sync>(
        &self,
        protocol: &'e P,
        r: u8,
        g: u8,
        b: u8,
    ) -> Result<(), BluetoothError>;
}

//-------------------------//
// Blanket implementations //
//-------------------------//
#[async_trait]
impl<D: Device + std::marker::Sync> Color for D {
    // bound type to be transferred across threads
    async fn set<'e, P: Protocol + std::marker::Send + std::marker::Sync>(
        device: &Self,
        protocol: &'e P,
        option: &'e ColorOption,
    ) -> Result<(), BluetoothError> {
        device.push(&protocol.color(option)[..]).await?;
        Ok(())
    }

    // -------------------------------//
    // Syntactic sugar /////////////////
    // more idiomatic syntactic sugar //
    // -------------------------------//
    async fn color<'e, P: Protocol + std::marker::Send + std::marker::Sync>(
        &self,
        protocol: &'e P,
        r: u8,
        g: u8,
        b: u8,
    ) -> Result<(), BluetoothError> {
        self
            .push(&protocol.color(&ColorOption::RGB(r, g, b))[..])
            .await?;
        Ok(())
    }
}
