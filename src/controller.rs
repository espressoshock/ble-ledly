use crate::ledmodule::Ledmodule;

use std::error::Error;

use btleplug::api::Characteristic;
use btleplug::api::{
    bleuuid::uuid_from_u16, Central, Manager as _, Peripheral as _, ScanFilter, WriteType,
};
use btleplug::platform::{Adapter, Manager, Peripheral};
use uuid::Uuid;


pub const DEFAULT_WRITE_CHARACTERISTIC_UUID: Uuid = uuid_from_u16(0xFFD9); 
pub const DEFAULT_WRITE_SERVICE_UUID: Uuid = uuid_from_u16(0xFFD5);

pub struct Controller{
    prefix: String,

    ble_manager: Manager,
    ble_adapter: Adapter,

    ledmodules: Vec<Ledmodule>,
}

impl Controller {
    pub async fn new(prefix: &str) -> Result<Controller, Box<dyn Error>> {
        let ble_manager = Manager::new().await?;
        
        let ble_adapter = ble_manager.adapters().await?;
        let client = ble_adapter
            .into_iter()
            .nth(0) // take first
            .expect("Unable to find a working adapter"); // replace with safe implementation
        Ok (
            Controller{
                prefix: prefix.to_string(),
                ble_manager,
                ble_adapter: client,
                ledmodules: Vec::new()
            }
        )
    }
}
