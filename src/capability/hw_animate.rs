use crate::communication_protocol::Protocol;
use crate::device::Device;
use crate::device::Write;
use crate::error::{BluetoothError};
use async_trait::async_trait;

//---------//
// animate //
//---------//
pub enum HWAnimateOption<'e> {
    Pulsating(&'e HWStaticColorOption, &'e HWAnimationSpeedSetting),
}

pub enum HWStaticColorOption {
    Red,
    Green,
    Blue,
}
// TODO: more meaningf&ul name
pub enum HWAnimationSpeedSetting {
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
pub trait HWAnimate {
    async fn set<'e, P: Protocol + std::marker::Send + std::marker::Sync>(
        device: &Self,
        protocol: &'e P,
        option: &'e HWAnimateOption,
    ) -> Result<(), BluetoothError>;

    // -------------------------------//
    // Syntactic sugar /////////////////
    // more idiomatic syntactic sugar //
    // -------------------------------//
    async fn hw_anim_pulsating<'e, P: Protocol + std::marker::Send + std::marker::Sync>(
        &self,
        protocol: &'e P,
        color: &'e HWStaticColorOption,
        speed: &'e HWAnimationSpeedSetting
    ) -> Result<(), BluetoothError>;
}

//-------------------------//
// Blanket implementations //
//-------------------------//
#[async_trait]
impl<D: Device + std::marker::Sync> HWAnimate for D {
    // bound type to be transferred across threads
    async fn set<'e, P: Protocol + std::marker::Send + std::marker::Sync>(
        device: &Self,
        protocol: &'e P,
        option: &'e HWAnimateOption,
    ) -> Result<(), BluetoothError> {
        device.push(&protocol.hw_animate(option)[..]).await?;
        Ok(())
    }

    // -------------------------------//
    // Syntactic sugar /////////////////
    // more idiomatic syntactic sugar //
    // -------------------------------//
    async fn hw_anim_pulsating<'e, P: Protocol + std::marker::Send + std::marker::Sync>(
        &self,
        protocol: &'e P,
        color: &'e HWStaticColorOption,
        speed: &'e HWAnimationSpeedSetting
    ) -> Result<(), BluetoothError> {
        self.push(&protocol.hw_animate(&HWAnimateOption::Pulsating(color, speed))[..]).await?;
        Ok(())
    }
}
