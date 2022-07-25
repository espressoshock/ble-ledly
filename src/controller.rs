use crate::device::Device;

use crate::error::BluetoothError;

use btleplug::api::{Central, Manager as _, Peripheral as _, ScanFilter};
use btleplug::platform::{Adapter, Manager};
use uuid::Uuid;

use std::time::Duration;
use tokio::time;

pub struct Controller<D: Device> {
    prefix: Option<String>,

    ble_manager: Manager,
    ble_adapter: Adapter,

    //TODO: provide key-like access - hashmap
    led_devices: Vec<D>,
}

impl<D: Device> Controller<D> {
    // Constructor //
    pub async fn new() -> Result<Controller<D>, BluetoothError> {
        let ble_manager = Manager::new().await?;

        let ble_adapter = ble_manager.adapters().await?;
        let client = ble_adapter
            .into_iter()
            .nth(0) // take first
            .ok_or(BluetoothError::InvalidBluetoothAdapter)?;

        Ok(Self {
            prefix: None,
            ble_manager,
            ble_adapter: client,
            led_devices: Vec::<D>::new(),
        })
    }
    pub async fn new_with_prefix(prefix: &str) -> Result<Controller<D>, BluetoothError> {
        let ble_manager = Manager::new().await?;

        let ble_adapter = ble_manager.adapters().await?;
        let client = ble_adapter
            .into_iter()
            .nth(0) // take first
            .ok_or(BluetoothError::InvalidBluetoothAdapter)?;

        Ok(Self {
            prefix: Some(prefix.to_string()),
            ble_manager,
            ble_adapter: client,
            led_devices: Vec::<D>::new(),
        })
    }
    //---------//
    // Getters //
    //---------//
    pub fn ble_manager(&self) -> &Manager {
        &self.ble_manager
    }
    pub fn list(&mut self) -> &mut Vec<D> {
        &mut self.led_devices
    }
    //------------------//
    // Device Discovery //
    //------------------//
    pub async fn device_discovery(&self) -> Result<Vec<D>, BluetoothError> {
        self.ble_adapter.start_scan(ScanFilter::default()).await?;
        time::sleep(Duration::from_secs(2)).await;

        let mut led_devices: Vec<D> = Vec::new();

        for p in self.ble_adapter.peripherals().await? {
            let name = &p
                .properties()
                .await?
                .ok_or(BluetoothError::InvalidPeriperipheralProperty)?
                .local_name
                .unwrap_or(String::from("Unknown"));

            if name.contains(self.prefix.as_ref().unwrap_or(&"".to_string())) {
                led_devices.push(D::new(&name, &name, Some(p), None, None, None));
            }
        }
        Ok(led_devices)
    }
    //---------//
    // Connect //
    //---------//j
    pub async fn connect(
        &mut self,
        led_devices: Option<Vec<D>>,
        characteristics_uuid: Option<Uuid>,
    ) -> Result<(), BluetoothError> {
        // Discover devices //
        if let Some(l_devices) = led_devices {
            self.led_devices = l_devices;
        } else {
            self.led_devices = self.device_discovery().await?;
        }

        // Connect devices //
        for led_device in self.led_devices.iter_mut() {
            // Connect //
            led_device
                .peripheral()
                .as_ref()
                .ok_or(BluetoothError::InvalidPeripheralReference)?
                .connect()
                .await?;

            // Service discovry //
            led_device
                .peripheral()
                .as_ref()
                .ok_or(BluetoothError::InvalidPeripheralReference)?
                .discover_services()
                .await?;

            let characteric = led_device
                .peripheral()
                .as_ref()
                .ok_or(BluetoothError::InvalidPeripheralReference)?
                .characteristics()
                .into_iter()
                .find(|c| {
                    c.uuid
                        == characteristics_uuid
                            .unwrap_or(led_device.default_write_characteristic_uuid())
                })
                .ok_or(BluetoothError::NotFoundTargetCharacteristic)?;

            // Add write characteric //
            // only one supported for now
            led_device.set_write_char(&characteric);
        }
        Ok(())
    }
}
