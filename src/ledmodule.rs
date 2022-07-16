use btleplug::{platform::Peripheral, api::Characteristic};

#[derive(Debug)]
pub struct Ledmodule{
    pub peripheral: Peripheral,

    pub write_char: Option<Characteristic>,
    pub read_char: Option<Characteristic>,
}
