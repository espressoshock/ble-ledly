use crate::capability::{light::LightOption, color::ColorOption, brightness::BrightnessOption};

pub mod generic_rgb;

pub trait Protocol {
    fn light(&self, option: LightOption) -> Vec<u8>;
    fn color(&self, option: ColorOption) -> Vec<u8>;
    fn brightness(&self, option: BrightnessOption) -> Vec<u8>;

}
