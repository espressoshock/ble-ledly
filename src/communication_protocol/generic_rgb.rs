use crate::{
    capability::{color::ColorOption, light::LightOption},
    communication_protocol::Protocol,
};

pub struct GenericRGB {}

impl Protocol for GenericRGB {
    // Light //
    fn light(&self, option: LightOption) -> Vec<u8> {
        // this doesn't work when
        // HWspecific effects are turn_on
        // use legacy mode instead
        // vec![0xcc, 0x24, 0x33]
        match option {
            LightOption::On => vec![0xcc, 0x23, 0x33],
            LightOption::Off => vec![0xcc, 0x24, 0x33],
        }
    }

    fn color(&self, option: ColorOption) -> Vec<u8> {
        match option {
            ColorOption::RGB(r, g, b) => vec![0x56, r, g, b, 0x00, 0xF0, 0xAA],
        }
    }
}
