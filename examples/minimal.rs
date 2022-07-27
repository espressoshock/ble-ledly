use ble_ledly::capability::color::*;
use ble_ledly::communication_protocol::GenericRGB;
use ble_ledly::device::LedDevice;
use ble_ledly::device::{CharKind, UuidKind};
use ble_ledly::Controller;

use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Create a new Light controller with prefix
    // Auto-filter devices that contain the prefix
    let mut controller = Controller::<LedDevice>::new_with_prefix("QHM-").await?;

    // Connect
    controller.connect().await?;

    // Choose your communication protocol
    let protocol = GenericRGB::default();

    // set default write characteristic for all connected
    // devices
    controller.set_all_char(&CharKind::Write, &UuidKind::Uuid16(0xFFD9))?;

    // Setting first found light color to red
    let first_light = controller.list().get(0).unwrap();
    first_light.color(&protocol, 255, 0, 0).await?;

    Ok(())
}
