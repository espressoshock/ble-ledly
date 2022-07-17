// use super::traits::Encode;

pub trait GenericRGBLight {
    fn turn_off(&self) -> Vec<u8> {
        vec![0xcc, 0x24, 0x33]
    }

    fn turn_on(&self) -> Vec<u8> {
        vec![0xcc, 0x23, 0x33]
    }

    fn encode_color(&self, red: u8, green: u8, blue: u8) -> Vec<u8> {
        vec![0x56, red, green, blue, 0x00, 0xF0, 0xAA]
    }
}
