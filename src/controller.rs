use crate::device::led_device::LedDevice;
use crate::device::traits::{Light, Device};

use std::error::Error;

use btleplug::api::Characteristic;
use btleplug::api::{
    bleuuid::uuid_from_u16, Central, Manager as _, Peripheral as _, ScanFilter, WriteType,
};
use btleplug::platform::{Adapter, Manager, Peripheral};
use uuid::Uuid;

use std::time::Duration;
use tokio::time;


pub const DEFAULT_WRITE_CHARACTERISTIC_UUID: Uuid = uuid_from_u16(0xFFD9); 
pub const DEFAULT_WRITE_SERVICE_UUID: Uuid = uuid_from_u16(0xFFD5);

pub struct Controller<T: Light>{
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
    pub async fn new(prefix: &str) -> Result<Self, Box<dyn Error>> {
        let ble_manager = Manager::new().await?;
        
        let ble_adapter = ble_manager.adapters().await?;
        let client = ble_adapter
            .into_iter()
            .nth(0) // take first
            .expect("Unable to find a working adapter"); // replace with safe implementation
        Ok (
            Self{
                prefix: prefix.to_string(),
                ble_manager,
                ble_adapter: client,
                led_devices: Vec::<T>::new()
            }
        )
    }

    //----------//
    // Discover //
    //----------//
    pub async fn discovery(&self) -> Result<Vec<T>, Box<dyn Error>> {
        println!("Starting scanning...");
        self.ble_adapter.start_scan(ScanFilter::default()).await;
        time::sleep(Duration::from_secs(2)).await;
    
        let mut led_devices: Vec<T> = Vec::new();
    
        for p in self.ble_adapter.peripherals().await.unwrap() {
            if p.properties()
                .await
                .unwrap()
                .unwrap()
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
    //---------//
    pub async fn connect(&mut self) {
        let led_devices = self.discovery().await.expect("Error during Discovery");
        self.led_devices = led_devices;
        println!("Connecting...");
        for ledmodule in self.led_devices.iter_mut() {
            ledmodule.peripheral().as_ref().unwrap().connect().await;
            println!("\n\nConnected to {:?}...", ledmodule.peripheral());
           
            ledmodule.peripheral().as_ref().unwrap().discover_services().await;
            let chars = ledmodule.peripheral().as_ref().unwrap().characteristics();
            let cmd_char = chars
                .into_iter()
                .find(|c| c.uuid == DEFAULT_WRITE_CHARACTERISTIC_UUID)
                .expect("Unable to find characterics");
            ledmodule.add_write_characteristic(cmd_char);
    
        }
    
    }
}
