use crate::capability::{
    animate::{AnimateOption, AnimationSpeedSetting, StaticColorOption}, brightness::BrightnessOption, color::ColorOption, light::LightOption,
};

pub mod generic_rgb;

pub trait Protocol {
    fn light(&self, option: LightOption) -> Vec<u8>;
    fn color(&self, option: ColorOption) -> Vec<u8>;
    fn brightness(&self, option: BrightnessOption) -> Vec<u8>;
    fn animate(&self, option: AnimateOption) -> Vec<u8>;

    // animate hwspecific helpers //
    fn _animation_speed(setting: &AnimationSpeedSetting) -> u8;
    fn _static_color(color: &StaticColorOption) -> u8;
}
