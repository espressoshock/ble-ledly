use crate::device::traits::{Device, Light};

use crate::errors::BluetoothError;
use std::error::Error;

use btleplug::api::{Central, Manager as _, Peripheral as _, ScanFilter};
use btleplug::platform::{Adapter, Manager};
use uuid::Uuid;

use std::time::Duration;
use tokio::time;

pub struct Controller<T: Light> {
    prefix: String,

    ble_manager: Manager,
    ble_adapter: Adapter,

    led_devices: Vec<T>,
}

impl<T> Controller<T>
where
    T: Light + Device,
{
    // Constructor //
    pub async fn new(prefix: &str) -> Result<Self, BluetoothError> {
        let ble_manager = Manager::new().await?;

        let ble_adapter = ble_manager.adapters().await?;
        let client = ble_adapter
            .into_iter()
            .nth(0) // take first
            .ok_or(BluetoothError::InvalidBluetoothAdapter)?;

        Ok(Self {
            prefix: prefix.to_string(),
            ble_manager,
            ble_adapter: client,
            led_devices: Vec::<T>::new(),
        })
    }
    //---------//
    // Getters //
    //---------//
    pub fn ble_manager(&self) -> &Manager {
        &self.ble_manager
    }

    //------------------//
    // Device Discovery //
    //------------------//
    pub async fn device_discovery(&self) -> Result<Vec<T>, Box<dyn Error>> {
        self.ble_adapter.start_scan(ScanFilter::default()).await?;
        time::sleep(Duration::from_secs(2)).await;

        let mut led_devices: Vec<T> = Vec::new();

        for p in self.ble_adapter.peripherals().await? {
            if p.properties()
                .await?
                .ok_or(BluetoothError::InvalidPeriperipheralProperty)?
                .local_name
                .iter()
                .any(|name| name.contains(&self.prefix))
            {
                println!("d{:?}", &p);
                led_devices.push(T::new("Alias", p, None, None));
            }
        }
        println!("Scan Terminated.");
        Ok(led_devices)
    }

    //---------//
    // Connect //
    //---------//j
    pub async fn connect(
        &mut self,
        characteristics_uuid: Option<Uuid>,
    ) -> Result<(), Box<dyn Error>> {
        // Discover devices //
        self.led_devices = self.device_discovery().await?;
        println!("Connecting...");

        // Connect devices //
        for led_device in self.led_devices.iter_mut() {
            // Connect //
            led_device
                .peripheral()
                .as_ref()
                .ok_or(BluetoothError::InvalidPeripheralReference)?
                .connect()
                .await?;

            println!("\n\nConnected to {:?}...", led_device.peripheral());

            // Service discovery //
            led_device
                .peripheral()
                .as_ref()
                .ok_or(BluetoothError::InvalidPeripheralReference)?
                .discover_services()
                .await?;

            // TODO: implement support for multiple write/notify services
            let characteric = led_device
                .peripheral()
                .as_ref()
                .ok_or(BluetoothError::InvalidPeripheralReference)?
                .characteristics()
                .into_iter()
                .find(|c| {
                    c.uuid
                        == characteristics_uuid
                            .unwrap_or(*led_device.default_write_characteristic_uuid())
                })
                .ok_or(BluetoothError::NotFoundTargetCharacteristic)?;

            // Add write characteric //
            // only one supported for now
            led_device.add_write_characteristic(characteric);
        }
        Ok(())
    }
}
