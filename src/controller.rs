use crate::ledmodule::LedModule;

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

pub struct Controller{
    prefix: String,

    ble_manager: Manager,
    ble_adapter: Adapter,

    ledmodules: Vec<LedModule>,
}

impl Controller {
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
                ledmodules: Vec::new()
            }
        )
    }
    async fn discovery(&self) -> Result<Vec<LedModule>, Box<dyn Error>> {
        println!("Starting scanning...");
        self.ble_adapter.start_scan(ScanFilter::default()).await;
        time::sleep(Duration::from_secs(2)).await;

        let mut ledmodules = Vec::new();

        for p in self.ble_adapter.peripherals().await.unwrap() {
            if p.properties()
                .await
                .unwrap()
                .unwrap()
                .local_name
                .iter()
                .any(|name| name.contains(&self.prefix))
                {
                    ledmodules.push(LedModule::new("Alias", p, None, None));
                }
        }
        println!("Scan Terminated.");
        Ok(ledmodules)
    }
    pub async fn connect(&mut self) {
        let ledmodules = self.discovery().await.expect("Error during Discovery");
        self.ledmodules = ledmodules;
        println!("Connecting...");
        for ledmodule in self.ledmodules.iter_mut() {
            ledmodule.peripheral().connect().await;
            println!("\n\nConnected to {:?}...", ledmodule.peripheral());

            ledmodule.peripheral().discover_services().await;
            let chars = ledmodule.peripheral().characteristics();
            let cmd_char = chars
                .into_iter()
                .find(|c| c.uuid == DEFAULT_WRITE_CHARACTERISTIC_UUID)
                .expect("Unable to find characterics");
            ledmodule.add_write_characteristic(cmd_char);

            for _ in 0..20 {
                println!("Light off");
                let color_cmd = vec![0xcc, 0x24, 0x33];
                ledmodule.peripheral().write(ledmodule.write_char().unwrap(), &color_cmd, WriteType::WithoutResponse).await;
                time::sleep(Duration::from_millis(200)).await;
                println!("Light on");
                let color_cmd = vec![0xcc, 0x23, 0x33];
                ledmodule.peripheral().write(ledmodule.write_char().unwrap(), &color_cmd, WriteType::WithoutResponse).await;
                time::sleep(Duration::from_millis(200)).await;
            }
        }

    }
}
