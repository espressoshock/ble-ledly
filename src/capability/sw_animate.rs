// Define Device Capabilities

use super::brightness::BrightnessOption;
use crate::capability::color::ColorOption;
use crate::communication_protocol::Protocol;
use crate::device::Device;
use crate::device::Write;
use crate::error::{BluetoothError, CapabilityError};
use async_trait::async_trait;

use std::time::Duration;
use tokio::time;

//---------//
// animate //
//---------//
pub enum SWAnimateOption {
    Breathing(ColorOption, SWAnimationRepeat, SWAnimationSpeed),
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
    async fn set<P: Protocol + std::marker::Send + std::marker::Sync>(
        &self,
        protocol: P,
        option: SWAnimateOption,
    ) -> Result<(), BluetoothError>;
}

//-------------------------//
// Blanket implementations //
//-------------------------//
#[async_trait]
impl<D: Device + std::marker::Sync> SWAnimate for D {
    // bound type to be transferred across threads
    async fn set<P: Protocol + std::marker::Send + std::marker::Sync>(
        &self,
        protocol: P,
        option: &SWAnimateOption,
    ) -> Result<(), BluetoothError> {
        self.push(&protocol.sw_animate(option)[..]).await?;
        Ok(())
    }
}

//------------//
// Animations //
//------------//
// TODO: Implement exponential fading
async fn _breathing<P: Protocol + std::marker::Send + std::marker::Sync>(
    protocol: &P,
    color: &ColorOption,
    interval: u64,
) {
    // TODO: replace loops with sine impl.
    for i in 0..=100 {
        protocol
            .brightness(&BrightnessOption::LevelWithColor(i as f32 / 100 as f32, *color));
        time::sleep(Duration::from_millis(interval)).await;
    }
    for i in (0..=100).rev() {
        protocol
            .brightness(&BrightnessOption::LevelWithColor(i as f32 / 100 as f32, *color));
        time::sleep(Duration::from_millis(interval)).await;
    }
}

pub async fn breathing<P: Protocol + std::marker::Send + std::marker::Sync>(
    protocol: P,
    color: &ColorOption,
    repeat: &SWAnimationRepeat,
    speed: &SWAnimationSpeed,
) {
    match repeat {
        SWAnimationRepeat::FiniteCount(count) => {
            let mut i = 0;
            while i < *count {
                _breathing(&protocol, &color, sw_animation_speed(speed)).await;
                i += 1;
            }
        }
        SWAnimationRepeat::InfiniteCount => loop {
            _breathing(&protocol, color, sw_animation_speed(speed)).await;
        },
    }
}
