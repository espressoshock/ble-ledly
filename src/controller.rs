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
    /// Creates a new `Device` controller
    ///
    /// # Examples
    ///
    /// ```
    /// let mut controller = Controller::<LedDevice>::new().await?;
    ///
    /// ```
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

    /// Creates a new `Device` controller with `Prefix`.
    /// The `prefix` is used to automatically filter the
    /// devices found during `device_discovery()`; The filter looks
    /// if the `prefix` __id contained__ in the device name.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut controller = Controller::<LedDevice>::new_with_prefix("QHM-").await?;
    ///
    /// ```
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

    /// Sets all the devices default _Characteristic_ (Write or Read)
    /// to the provided value. Global shortcut, instead of setting, per-device _Characteristic_
    /// configuration
    ///
    /// # Examples
    ///
    /// ```
    // controller.set_all_char(&CharKind::Write, &UuidKind::Uuid16(0xFFD9))?;
    ///
    /// ```
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

    /// Return a list (Vec<D>) of all the connected devices.
    /// The list is empty until devices are connected to the controller.
    ///
    /// # Examples
    ///
    /// ```
    /// let connected_lights = controller.list();
    ///
    /// ```
    pub fn list(&mut self) -> &mut Vec<D> {
        &mut self.led_devices
    }

    //------------------//
    // Device Discovery //
    //------------------//
    /// Discover _ble devices_ by running a scan op. on the default adapter
    /// and returns the found _devices__.
    ///
    /// # Examples
    ///
    /// ```
    /// let led_devices = controller.device_discovery().await?;
    ///
    /// ```
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
    /// Standalone `connect()` that can be used in conjunction with
    /// `Controller::new_with_prefix(prfix)`; it automatically runs
    /// a `device_discovery()` and connects to all devices that match `prefix`.
    /// If no `prefix` is provided it attemps to connect to all available devices.
    ///
    /// # Examples
    ///
    /// ```
    /// controller.connect().await?;
    ///
    /// ```
    pub async fn connect(&mut self) -> Result<(), BluetoothError> {
        // Discover devices //
        let led_devices = self.device_discovery().await?;
        self._connect(led_devices).await?;
        Ok(())
    }

    /// Connect to the devices passed as function's argument.
    ///
    /// # Examples
    ///
    /// ```
    /// let led_devices = controller.device_discovery().await?;
    /// filter devices
    /// let lights: Vec<LedDevice> = led_devices
    ///    .into_iter()
    ///   .filter(|device| device.name.contains("QHM-"))
    ///    .collect();
    /// controller.connect_with_devices(lights).await?;
    ///
    /// ```
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
