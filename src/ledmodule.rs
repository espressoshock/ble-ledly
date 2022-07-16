use btleplug::{platform::Peripheral, api::Characteristic};

// pub const DEFAULT_WRITE_CHARACTERISTIC_UUID: Uuid = uuid_from_u16(0xFFD9); 

#[derive(Debug)]
pub struct LedModule{
    alias: String,

    peripheral: Peripheral,

    write_chars: (Vec<Characteristic>, usize),
    read_chars: (Vec<Characteristic>, usize),
}

impl LedModule {
    // Constructor //
    pub fn new (alias: &str, peripheral: Peripheral, write_chars: Option<Vec<Characteristic>>, read_chars: Option<Vec<Characteristic>>) -> Self {
        Self {
            alias: alias.to_owned(),
            peripheral,
            write_chars: (write_chars.unwrap_or(Vec::new()), 0usize),
            read_chars: (read_chars.unwrap_or(Vec::new()), 0usize),
        }
    }

    // Getter, setter //
    pub fn alias(&self) -> &str {
        &self.alias
    }
    pub fn peripheral(&self) -> &Peripheral {
        &self.peripheral
    }
    pub fn set_peripheral(&mut self, peripheral: Peripheral) {
        self.peripheral = peripheral;
    }
    pub fn add_write_characteristic(&mut self, characteristic: Characteristic) {
        self.write_chars.0.push(characteristic);
    }
    pub fn add_read_characteristic(&mut self, characteristic: Characteristic) {
        self.read_chars.0.push(characteristic);
    }
    pub fn write_char(&self) -> Option<&Characteristic> {
        (&self).write_chars.0.get(self.write_chars.1)
    }
}
