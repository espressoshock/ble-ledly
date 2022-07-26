use crate::device::{CharKind, Device, UuidKind};
use crate::error::BluetoothError;
use btleplug::api::{Central, Manager as _, Peripheral as _, ScanFilter};
use btleplug::platform::{Adapter, Manager};
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
    pub fn set_all_char(
        &mut self,
        char_kind: &CharKind,
        uuid_kind: &UuidKind,
    ) -> Result<(), BluetoothError> {
        self.led_devices
            .iter_mut()
            .map(|device| device.set_char(char_kind, uuid_kind))
            .collect::<Result<(), BluetoothError>>()?;
        Ok(())
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
                led_devices.push(D::new(&name, &name, Some(p), None, None));
            }
        }
        Ok(led_devices)
    }
    //---------//
    // Connect //
    //---------//
    pub async fn connect(&mut self) -> Result<(), BluetoothError> {
        // Discover devices //
        let led_devices = self.device_discovery().await?;
        self._connect(led_devices).await?;
        Ok(())
    }
    pub async fn connect_with_devices(
        &mut self,
        led_devices: Vec<D>,
    ) -> Result<(), BluetoothError> {
        self._connect(led_devices).await?;
        Ok(())
    }
    async fn _connect(&mut self, led_devices: Vec<D>) -> Result<(), BluetoothError> {
        self.led_devices = led_devices;

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
        }
        Ok(())
    }
}
