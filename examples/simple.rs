use ble_ledly::communication_protocol::generic_rgb_light::{
    AnimationSpeedSetting, PulsatingColor,
};
use ble_ledly::controller::Controller;
use ble_ledly::device::led_device::LedDevice;
use ble_ledly::device::traits::{Light, RGB};

use ble_ledly::animation::{AnimationRepeat, AnimationSpeed};
use ble_ledly::animation;

use std::time::Duration;
use tokio::time;

#[tokio::main]
async fn main() {
    // Create a new Light controller with prefix
    let mut controller = Controller::<LedDevice>::new("QHM-").await.unwrap();
    // Connect
    controller.connect(None, None).await.unwrap();
    // list all connected devices
    let connected_lights = controller.list();

    // control light

    for light in connected_lights.iter_mut() {
        // Test RGB
        light.set_color(255, 0, 0).await;
        time::sleep(Duration::from_millis(800)).await;
       
        light.set_color(0, 255, 0).await;
        time::sleep(Duration::from_millis(800)).await;
    
        light.set_color(0, 0, 255).await;
        time::sleep(Duration::from_millis(800)).await;
    
        // Client, non-transferrable animation
        animation::breathing(light, 255, 0, 0, &AnimationRepeat::FiniteCount(1), &AnimationSpeed::Fastest).await;
        animation::breathing(light, 0, 255, 0, &AnimationRepeat::FiniteCount(1), &AnimationSpeed::Fastest).await;
        animation::breathing(light, 0, 0, 255, &AnimationRepeat::FiniteCount(1), &AnimationSpeed::Fastest).await;
    
        // HWspecific animation
        light.pulsating(&PulsatingColor::Red, &AnimationSpeedSetting::Speed9).await;
        time::sleep(Duration::from_millis(2000)).await;
        light.pulsating(&PulsatingColor::Green, &AnimationSpeedSetting::Speed9).await;
        time::sleep(Duration::from_millis(2000)).await;
        light.pulsating(&PulsatingColor::Blue, &AnimationSpeedSetting::Speed9).await;
        time::sleep(Duration::from_millis(2000)).await;
    
        // turn-off
        light.turn_off().await;
    }
}
