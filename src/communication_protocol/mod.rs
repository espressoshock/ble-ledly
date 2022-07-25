use crate::capability::LightOption;

pub mod generic_rgb;

pub trait Protocol {
    fn light(&self, option: LightOption) -> &[u8];
}
