use crate::device::traits::{Light, RGB};

use std::time::Duration;
use tokio::time;

pub enum AnimationRepeat {
    FiniteCount(i32),
    InfiniteCount,
}

pub enum AnimationSpeed {
    Slowest,
    Slower,
    Slow,
    Normal,
    Fast,
    Faster,
    Fastest,
}

fn animation_speed(speed: &AnimationSpeed) -> u64 {
    match speed {
        AnimationSpeed::Fastest => 5,
        AnimationSpeed::Faster => 20,
        AnimationSpeed::Fast => 50,
        AnimationSpeed::Normal => 200,
        AnimationSpeed::Slow => 300,
        AnimationSpeed::Slower => 400,
        AnimationSpeed::Slowest => 600,
    }
}
// TODO: Implement exponential fading
async fn _breathing<T: Light + RGB>(
    led_device: &mut T,
    red: u8,
    green: u8,
    blue: u8,
    interval: u64,
) {
    // TODO: replace loops with sine impl.
    for i in 0..=100 {
        led_device
            .set_brightness(red, green, blue, i as f32 / 100 as f32)
            .await;
        time::sleep(Duration::from_millis(interval)).await;
    }
    for i in (0..=100).rev() {
        led_device
            .set_brightness(red, green, blue, i as f32 / 100 as f32)
            .await;
        time::sleep(Duration::from_millis(interval)).await;
    }
}

pub async fn breathing<T: Light + RGB>(
    led_device: &mut T,
    red: u8,
    green: u8,
    blue: u8,
    repeat: &AnimationRepeat,
    speed: &AnimationSpeed,
) {
    match repeat {
        AnimationRepeat::FiniteCount(count) => {
            let mut i = 0;
            while i < *count {
                _breathing(led_device, red, green, blue, animation_speed(speed)).await;
                i += 1;
            }
        }
        AnimationRepeat::InfiniteCount => loop {
            _breathing(led_device, red, green, blue, animation_speed(speed)).await;
        },
    }
}
