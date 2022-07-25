use ble_ledly::capability::color::*;
use ble_ledly::capability::light::*;
use ble_ledly::capability::sw_animate::*;
use ble_ledly::communication_protocol::generic_rgb::GenericRGB;
use ble_ledly::controller::Controller;
use ble_ledly::device::led_device::LedDevice;

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
    controller.connect(Some(lights), None).await?;

    // Choose your communication protocol
    let protocol = GenericRGB::default();

    // list all connected devices
    let connected_lights = controller.list();
    for light in connected_lights.iter_mut() {
        println!("Connected to : {}", light.name);

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

        println!("SW Animation - Breathing effect...");
        light
            .breathing(
                &GenericRGB {},
                &ColorOption::RGB(255, 0, 0),
                &SWAnimationRepeat::FiniteCount(2),
                &SWAnimationSpeed::Fastest,
            )
            .await?;
        light
            .breathing(
                &GenericRGB {},
                &ColorOption::RGB(0, 255, 0),
                &SWAnimationRepeat::FiniteCount(2),
                &SWAnimationSpeed::Fastest,
            )
            .await?;
        light
            .breathing(
                &GenericRGB {},
                &ColorOption::RGB(0, 0, 255),
                &SWAnimationRepeat::FiniteCount(2),
                &SWAnimationSpeed::Fastest,
            )
            .await?;

        // Control the lights
        println!("Turning light off...");
        light.turn_off(&protocol).await?;
    }

    Ok(())
}
