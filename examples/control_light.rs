use ble_ledly::capability::color::*;
use ble_ledly::capability::hw_animate::*;
use ble_ledly::capability::light::*;
use ble_ledly::capability::sw_animate::*;
use ble_ledly::communication_protocol::generic_rgb::GenericRGB;
use ble_ledly::controller::Controller;
use ble_ledly::device::led_device::LedDevice;
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

    // set the default write Characteristic
    // for all devices. Optionally you can also
    // set it per-device. Look the examples folder for more
    controller.set_all_char(&CharKind::Write, &UuidKind::Uuid16(0xFFD9))?;

    // list all connected devices
    let connected_lights = controller.list();
    for light in connected_lights.iter_mut() {
        println!("Connected to : {}", light.name);

        // Control the lights
        println!("Turning light on...");
        Light::set(light, &protocol, &LightOption::On).await?;

        // Set color
        println!("Setting color...");
        Color::set(light, &protocol, &ColorOption::RGB(255, 0, 0)).await?;
        time::sleep(Duration::from_millis(800)).await;
        Color::set(light, &protocol, &ColorOption::RGB(0, 255, 0)).await?;
        time::sleep(Duration::from_millis(800)).await;
        Color::set(light, &protocol, &ColorOption::RGB(0, 0, 255)).await?;
        time::sleep(Duration::from_millis(800)).await;

        // HW-specific animation
        println!("HW Animation - Pulsating effect...");
        HWAnimate::set(
            light,
            &protocol,
            &HWAnimateOption::Pulsating(
                &HWStaticColorOption::Red,
                &HWAnimationSpeedSetting::Speed9,
            ),
        )
        .await?;
        time::sleep(Duration::from_millis(2000)).await;
        HWAnimate::set(
            light,
            &protocol,
            &HWAnimateOption::Pulsating(
                &HWStaticColorOption::Green,
                &HWAnimationSpeedSetting::Speed9,
            ),
        )
        .await?;
        time::sleep(Duration::from_millis(2000)).await;
        HWAnimate::set(
            light,
            &protocol,
            &HWAnimateOption::Pulsating(
                &HWStaticColorOption::Blue,
                &HWAnimationSpeedSetting::Speed9,
            ),
        )
        .await?;

        // SW animations
        println!("SW Animation - Breathing effect...");
        light
            .breathing(
                &protocol,
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
