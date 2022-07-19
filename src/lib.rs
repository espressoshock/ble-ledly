//! ble-ledly is a _Customizable_ and _extensible_ cross-platform high-level _Bluetooth Low Energy_ light controller. Built on top of btleplug, it is the designed to control _ble led lights and _led strips_. Provides out-of-the-box _hardware-specific_ controls and animation as well as non-transferable (requires continuous client connection).
//!
//!
//! ## Usage
//!
//! An example of how to use the library
//!
//! ```rust, no_run
//!   use ble_ledly::communication_protocol::generic_rgb_light::{
//!       AnimationSpeedSetting, PulsatingColor,
//!   };
//!   use ble_ledly::controller::Controller;
//!   use ble_ledly::device::led_device::LedDevice;
//!   use ble_ledly::device::traits::{Device, Light, RGB};
//!
//!   use ble_ledly::animation::{AnimationRepeat, AnimationSpeed};
//!   use ble_ledly::animation;
//!
//!   use std::time::Duration;
//!   use tokio::time;
//!
//!   #[tokio::main]
//!   async fn main() {
//!       // Create a new Light controller with prefix
//!       let mut controller = Controller::<LedDevice>::new("QHM-").await.unwrap();
//!       // Discover devices (scan)
//!       let led_devices = controller.device_discovery().await.unwrap();
//!       // inspect all found devices
//!       for device in led_devices.iter() {
//!           println!("Found device: {}", device.name());
//!       }
//!       // filter devices
//!       let lights: Vec<LedDevice> = led_devices
//!           .into_iter()
//!           .filter(|device| device.name().contains("1249"))
//!           .collect();
//!       // print filtered using debug trait
//!       println!("Filtered Lights: {:?}", lights);
//!
//!       // Connect
//!       controller.connect(Some(lights), None).await.unwrap();
//!       // list all connected devices
//!       let connected_lights = controller.list();
//!       for light in connected_lights.iter_mut() {
//!           println!("Connected to : {}", light.name());
//!
//!           // Control the lights
//!           light.set_color(255, 255, 255).await;
//!           time::sleep(Duration::from_millis(800)).await;
//!
//!           // Test RGB
//!           light.set_color(255, 0, 0).await;
//!           time::sleep(Duration::from_millis(800)).await;
//!           
//!           light.set_color(0, 255, 0).await;
//!           time::sleep(Duration::from_millis(800)).await;
//!
//!           light.set_color(0, 0, 255).await;
//!           time::sleep(Duration::from_millis(800)).await;
//!
//!           // Client, non-transferrable animation
//!           animation::breathing(light, 255, 0, 0, &AnimationRepeat::FiniteCount(1), &AnimationSpeed::Fastest).await;
//!           animation::breathing(light, 0, 255, 0, &AnimationRepeat::FiniteCount(1), &AnimationSpeed::Fastest).await;
//!           animation::breathing(light, 0, 0, 255, &AnimationRepeat::FiniteCount(1), &AnimationSpeed::Fastest).await;
//!
//!           // HWspecific animation
//!           light.pulsating(&PulsatingColor::Red, &AnimationSpeedSetting::Speed9).await;
//!           time::sleep(Duration::from_millis(2000)).await;
//!           light.pulsating(&PulsatingColor::Green, &AnimationSpeedSetting::Speed9).await;
//!           time::sleep(Duration::from_millis(2000)).await;
//!           light.pulsating(&PulsatingColor::Blue, &AnimationSpeedSetting::Speed9).await;
//!           time::sleep(Duration::from_millis(2000)).await;
//!
//!           // turn-off
//!           light.turn_off().await;
//!       }
//!   }
//! ```

pub mod animation;
pub mod communication_protocol;
pub mod controller;
pub mod device;
pub mod errors;
