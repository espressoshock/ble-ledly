use ble_ledly::capability::color::*;
use ble_ledly::capability::light::*;
use ble_ledly::communication_protocol::GenericRGB;
use ble_ledly::Controller;
use ble_ledly::device::LedDevice;
use ble_ledly::device::Device;
use ble_ledly::device::{CharKind, UuidKind};

use std::error::Error;
use std::time::Duration;
use tokio::time;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Create a new Light controller
    let mut controller = Controller::<LedDevice>::new().await?;

    // Discover devices (scan)
    let led_devices = controller.device_discovery().await?;

    // inspect all found devices
    for device in led_devices.iter() {
        println!("Found device: {}", device);
    }
    // filter devices
    let lights: Vec<LedDevice> = led_devices
        .into_iter()
        .filter(|device| device.name.contains("QHM-"))
        .collect();

    // Connect
    controller.connect_with_devices(lights).await?;

    // Choose your communication protocol
    let protocol = GenericRGB::default();

    // list all connected devices
    let connected_lights = controller.list();
    for light in connected_lights.iter_mut() {
        println!("-- Manipulating light {} --", light);

        // set the write_char for the current device
        // you can set different write characteristics for different
        // devices, as one controller support devices with different communication
        // protocols
        // Set it with an Uuid, an u32, or u16
        light.set_char(&CharKind::Write, &UuidKind::Uuid16(0xFFD9))?;

        /////////////////////////////////
        // Control the lights as usual //
        /////////////////////////////////

        // Control the lights
        println!("Turning light on...");
        light.turn_on(&protocol).await?;

        // Set color
        println!("Setting color...");
        light.color(&protocol, 255, 0, 0).await?;
        time::sleep(Duration::from_millis(800)).await;
        light.color(&protocol, 0, 255, 0).await?;
        time::sleep(Duration::from_millis(800)).await;
        light.color(&protocol, 0, 0, 255).await?;
        time::sleep(Duration::from_millis(800)).await;

        println!("Turning light off...");
        light.turn_off(&protocol).await?;
    }

    Ok(())
}
