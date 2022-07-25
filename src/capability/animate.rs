// Define Device Capabilities

use crate::communication_protocol::Protocol;
use crate::device::Device;
use crate::device::Write;
use crate::error::{BluetoothError, CapabilityError};
use async_trait::async_trait;
use crate::capability::color::ColorOption;

//---------//
// animate //
//---------//
pub enum AnimateOption {
    Pulsating(StaticColorOption, AnimationSpeedSetting),
}

pub enum StaticColorOption {
    Red,
    Green,
    Blue,
}
// TODO: more meaningf&ul name
pub enum AnimationSpeedSetting {
    Speed1,
    Speed2,
    Speed3,
    Speed4,
    Speed5,
    Speed6,
    Speed7,
    Speed8,
    Speed9,
}


#[async_trait]
pub trait Animate {
    async fn set<P: Protocol + std::marker::Send + std::marker::Sync>(
        &self,
        protocol: P,
        option: AnimateOption,
    ) -> Result<(), BluetoothError>;
}

//-------------------------//
// Blanket implementations //
//-------------------------//
#[async_trait]
impl<D: Device + std::marker::Sync> Animate for D {
    // bound type to be transferred across threads
    async fn set<P: Protocol + std::marker::Send + std::marker::Sync>(
        &self,
        protocol: P,
        option: AnimateOption,
    ) -> Result<(), BluetoothError> {
        self.push(&protocol.animate(option)[..]).await?;
        Ok(())
    }
}
