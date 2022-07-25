use crate::capability::{light::LightOption, color::ColorOption};

pub mod generic_rgb;

pub trait Protocol {
    fn light(&self, option: LightOption) -> Vec<u8>;
    fn color(&self, option: ColorOption) -> Vec<u8>;
}
