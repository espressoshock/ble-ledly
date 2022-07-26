use btleplug::api::{CharPropFlags, Characteristic, Peripheral as _};
use btleplug::platform::Peripheral;

use btleplug::api::bleuuid::uuid_from_u16;
use enumflags2::_internal::RawBitFlags;
use uuid::Uuid;

use std::fmt;

use crate::device::Device;
use enumflags2::{bitflags, BitFlags};

// wrapper for native ble charprops
#[bitflags]
#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum CharactericKind {
    Broadcast = 0x01,
    Read = 0x02,
    WriteWithoutResponse = 0x04,
    Write = 0x08,
    Notify = 0x10,
    Indicate = 0x20,
    AuthenticatedSignedWrites = 0x40,
    ExtendedProperties = 0x80,
}

#[derive(Debug)]
pub struct LedDevice {
    // BLE localname mapping
    pub name: String,
    // user-settable alias
    pub alias: String,

    // underlying BLE Pheripheral
    peripheral: Option<Peripheral>,

    // default communication chars
    write_char: Option<Characteristic>,
    read_char: Option<Characteristic>,

    // default write char uuid
    write_char_uuid: Uuid,
}
// common to generic ble ic(s)
pub const DEFAULT_WRITE_CHARACTERISTIC_UUID: Uuid = uuid_from_u16(0xFFD9);

impl Device for LedDevice {
    fn new(
        name: &str,
        alias: &str,
        peripheral: Option<Peripheral>,
        write_char: Option<Characteristic>,
        read_char: Option<Characteristic>,
        write_char_uuid: Option<Uuid>,
    ) -> Self {
        Self {
            name: name.to_string(),
            alias: alias.to_string(),
            peripheral,
            write_char: write_char.clone(),
            read_char: read_char.clone(),
            write_char_uuid: write_char_uuid.unwrap_or(DEFAULT_WRITE_CHARACTERISTIC_UUID),
        }
    }
    // TODO: remove getters and setters
    //--------//
    // Getter //
    //--------//
    fn alias(&self) -> &str {
        &self.alias
    }
    fn name(&self) -> &str {
        &self.name
    }
    fn address(&self) -> Option<String> {
        if let Some(peripheral) = self.peripheral.as_ref() {
            return Some(peripheral.address().to_string());
        }
        None
    }
    fn peripheral(&self) -> Option<&Peripheral> {
        self.peripheral.as_ref()
    }
    fn write_char_uuid(&self) -> &Uuid {
        &self.write_char_uuid
    }
    fn write_char(&self) -> Option<&Characteristic> {
        self.write_char.as_ref()
    }
    fn read_char(&self) -> Option<&Characteristic> {
        self.read_char.as_ref()
    }
    fn default_write_characteristic_uuid(&self) -> Uuid {
        DEFAULT_WRITE_CHARACTERISTIC_UUID.clone()
    }
    fn characteristics(&self) -> Option<Vec<Characteristic>> {
        if let Some(peripheral) = self.peripheral.as_ref() {
            return Some(
                peripheral
                    .characteristics()
                    .into_iter()
                    .collect::<Vec<Characteristic>>(),
            );
        }
        None
    }
    fn characteristics_by_type(&self, kinds: BitFlags<CharactericKind>) -> Option<Vec<Characteristic>> {
        if let Some(chars) = self.characteristics() {
            return Some(
                chars
                    .into_iter()
                    .filter(|c| c.properties.bits() == kinds.bits())
                    .collect(),
            );
        }
        None
    }

    //--------//
    // Setter //
    //--------//
    fn set_alias(&mut self, alias: &str) {
        self.alias = alias.to_string();
    }
    fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }
    fn set_peripheral(&mut self, peripheral: Peripheral) {
        self.peripheral = Some(peripheral);
    }
    fn set_write_char_uuid(&mut self, char_uuid: Uuid) {
        self.write_char_uuid = char_uuid;
    }
    fn set_write_char(&mut self, characteristic: &Characteristic) {
        self.write_char = Some(characteristic.clone());
    }
}
//--------------//
// Display impl //
//--------------//
impl fmt::Display for LedDevice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} ({})",
            self.name(),
            self.address().unwrap_or(String::from("-"))
        )
    }
}
