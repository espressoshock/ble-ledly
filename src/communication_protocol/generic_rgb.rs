
use crate::{communication_protocol::Protocol, capability::LightOption};

pub struct GenericRGB {}

impl Protocol for GenericRGB {
    fn light(&self, option: LightOption) -> &[u8]{
         // this doesn't work when
        // HWspecific effects are turn_on
        // use legacy mode instead
        // vec![0xcc, 0x24, 0x33]
        match option {
            LightOption::On => &[0xcc, 0x23, 0x33],
            LightOption::Off => &[0xcc, 0x24, 0x33],
        }
    }
}
