use crate::capability::color::ColorOption;
use crate::communication_protocol::Protocol;
use crate::device::Device;
use crate::device::Write;
use crate::error::{BluetoothError};
use async_trait::async_trait;

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
        device: &Self,
        protocol: &'e P,
        option: &'e BrightnessOption,
    ) -> Result<(), BluetoothError>;

    // -------------------------------//
    // Syntactic sugar /////////////////
    // more idiomatic syntactic sugar //
    // -------------------------------//
    async fn brightness<'e, P: Protocol + std::marker::Send + std::marker::Sync>(
        &self,
        protocol: &'e P,
        r: u8,
        g: u8,
        b: u8,
        level: f32,
    ) -> Result<(), BluetoothError>;
}

//-------------------------//
// Blanket implementations //
//-------------------------//
#[async_trait]
impl<D: Device + std::marker::Sync> Brightness for D {
    // bound type to be transferred across threads
    async fn set<'e, P: Protocol + std::marker::Send + std::marker::Sync>(
        device: &Self,
        protocol: &'e P,
        option: &'e BrightnessOption,
    ) -> Result<(), BluetoothError> {
        device.push(&protocol.brightness(option)[..]).await?;
        Ok(())
    }

    // -------------------------------//
    // Syntactic sugar /////////////////
    // more idiomatic syntactic sugar //
    // -------------------------------//
    async fn brightness<'e, P: Protocol + std::marker::Send + std::marker::Sync>(
        &self,
        protocol: &'e P,
        r: u8,
        g: u8,
        b: u8,
        level: f32,
    ) -> Result<(), BluetoothError> {
        self
            .push(
                &protocol.brightness(&BrightnessOption::LevelWithColor(
                    level,
                    &ColorOption::RGB(r, g, b),
                ))[..],
            )
            .await?;
        Ok(())
    }
}
