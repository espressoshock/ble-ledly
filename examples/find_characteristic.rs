use ble_ledly::capability::color::*;
use ble_ledly::capability::light::*;
use ble_ledly::communication_protocol::GenericRGB;
use ble_ledly::Controller;
use ble_ledly::device::{LedDevice, OpKind};
use ble_ledly::device::Device;

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
        println!("--- Found characteristics for device {}: ---", light);

        // inspect all characteristic for every device
        for characteristic in light.characteristics().unwrap().iter() {
            println!(
                "\tUuid: {:?}, Type: {:?}",
                characteristic.uuid, characteristic.properties
            );
        }

        println!("--- Filtered characteristics for device {}: ---", light);

        // otherwise inspect all characteristic by supported operation kind
        let char_kind_filter = OpKind::Write | OpKind::WriteWithoutResponse;

        for characteristic in light
            .characteristics_by_type(char_kind_filter)
            .unwrap()
            .iter()
        {
            println!(
                "\tUuid: {:?}, Type: {:?}",
                characteristic.uuid, characteristic.properties
            );
        }

        // choose the characteristic to use to write to the device
        let chosen = light.characteristics_by_type(char_kind_filter).unwrap();
        println!("\nChosen {:?}\n", chosen.get(0));

        // set it as a write_char for the current device
        // you can set different write characteristics for different
        // devices, as one controller support devices with different communication
        // protocols
        light.set_write_char(&chosen.get(0).unwrap());

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
