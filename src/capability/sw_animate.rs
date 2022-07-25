use super::brightness::BrightnessOption;
use crate::capability::color::ColorOption;
use crate::communication_protocol::Protocol;
use crate::device::Device;
use crate::device::Write;
use crate::error::{BluetoothError};
use async_trait::async_trait;

use std::time::Duration;
use tokio::time;

//---------//
// animate //
//---------//
pub enum SWAnimateOption<'e> {
    Breathing(&'e ColorOption, &'e SWAnimationRepeat, &'e SWAnimationSpeed),
}

pub enum SWAnimationRepeat {
    FiniteCount(i32),
    InfiniteCount,
}

pub enum SWAnimationSpeed {
    Slowest,
    Slower,
    Slow,
    Normal,
    Fast,
    Faster,
    Fastest,
}
//--------------------//
// Enum value mapping //
//--------------------//
fn sw_animation_speed(speed: &SWAnimationSpeed) -> u64 {
    match speed {
        SWAnimationSpeed::Fastest => 5,
        SWAnimationSpeed::Faster => 20,
        SWAnimationSpeed::Fast => 50,
        SWAnimationSpeed::Normal => 200,
        SWAnimationSpeed::Slow => 300,
        SWAnimationSpeed::Slower => 400,
        SWAnimationSpeed::Slowest => 600,
    }
}

#[async_trait]
pub trait SWAnimate {
    async fn set<'e, P: Protocol + std::marker::Send + std::marker::Sync>(
        &self,
        protocol: &'e P,
        option: &'e SWAnimateOption,
    ) -> Result<(), BluetoothError>;
    async fn _breathing<'e, P: Protocol + std::marker::Send + std::marker::Sync>(
        &self,
        protocol: &'e P,
        color: &'e ColorOption,
        interval: u64,
    ) -> Result<(), BluetoothError>;
    async fn breathing<'e, P: Protocol + std::marker::Send + std::marker::Sync>(
        &self,
        protocol: &'e P,
        color: &'e ColorOption,
        repeat: &'e SWAnimationRepeat,
        speed: &'e SWAnimationSpeed,
    ) -> Result<(), BluetoothError>;
}

//-------------------------//
// Blanket implementations //
//-------------------------//
#[async_trait]
impl<D: Device + std::marker::Sync> SWAnimate for D {
    // bound type to be transferred across threads
    async fn set<'e, P: Protocol + std::marker::Send + std::marker::Sync>(
        &self,
        protocol: &'e P,
        option: &'e SWAnimateOption,
    ) -> Result<(), BluetoothError> {
        match option {
            SWAnimateOption::Breathing(color, repeat, speed) => {
                self.breathing(protocol, color, repeat, speed).await?;
            }
        }
        Ok(())
    }

    //------------//
    // Animations //
    //------------//
    // TODO: Implement exponential fading
    async fn _breathing<'e, P: Protocol + std::marker::Send + std::marker::Sync>(
        &self,
        protocol: &'e P,
        color: &'e ColorOption,
        interval: u64,
    ) -> Result<(), BluetoothError> {
        // TODO: replace loops with sine impl.
        for i in 0..=100 {
            let e_bytes = protocol.brightness(&BrightnessOption::LevelWithColor(
                i as f32 / 100 as f32,
                color,
            ));
            self.push(&(e_bytes)[..]).await?;
            time::sleep(Duration::from_millis(interval)).await;
        }
        for i in (0..=100).rev() {
            let e_bytes = protocol.brightness(&BrightnessOption::LevelWithColor(
                i as f32 / 100 as f32,
                color,
            ));
            self.push(&(e_bytes)[..]).await?;
            time::sleep(Duration::from_millis(interval)).await;
        }
        Ok(())
    }

    async fn breathing<'e, P: Protocol + std::marker::Send + std::marker::Sync>(
        &self,
        protocol: &'e P,
        color: &'e ColorOption,
        repeat: &'e SWAnimationRepeat,
        speed: &'e SWAnimationSpeed,
    ) -> Result<(), BluetoothError> {
        match repeat {
            SWAnimationRepeat::FiniteCount(count) => {
                let mut i = 0;
                while i < *count {
                    self._breathing(protocol, &color, sw_animation_speed(speed))
                        .await?;
                    i += 1;
                }
            }
            SWAnimationRepeat::InfiniteCount => loop {
                self._breathing(protocol, &color, sw_animation_speed(speed))
                    .await?;
            },
        }
        Ok(())
    }
}
