use crate::capability::{
    hw_animate::{HWAnimateOption, HWAnimationSpeedSetting, HWStaticColorOption}, brightness::BrightnessOption, color::ColorOption, light::LightOption,
};

pub mod generic_rgb;

pub trait Protocol {
    fn light(&self, option: LightOption) -> Vec<u8>;
    fn color(&self, option: ColorOption) -> Vec<u8>;
    fn brightness(&self, option: BrightnessOption) -> Vec<u8>;
    fn hw_animate(&self, option: HWAnimateOption) -> Vec<u8>;

    // animate hwspecific helpers //
    fn _animation_speed(setting: &HWAnimationSpeedSetting) -> u8;
    fn _static_color(color: &HWStaticColorOption) -> u8;
}
