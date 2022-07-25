use crate::{
    capability::{
        brightness::BrightnessOption,
        color::ColorOption,
        hw_animate::{
            HWAnimateOption, HWAnimationSpeedSetting, HWStaticColorOption,
        },
        light::LightOption,
    },
    communication_protocol::Protocol,
};

pub struct GenericRGB {}

impl Default for GenericRGB {
    fn default() -> Self {
        Self {}
    }
}


impl Protocol for GenericRGB {
    // Light //
    fn light(&self, option: &LightOption) -> Vec<u8> {
        // this doesn't work when
        // HWspecific effects are turn_on
        // use legacy mode instead
        match option {
            LightOption::On => vec![0xcc, 0x23, 0x33],
            LightOption::Off => vec![0xcc, 0x24, 0x33],
        }
    }

    fn color(&self, option: &ColorOption) -> Vec<u8> {
        match option {
            ColorOption::RGB(r, g, b) => vec![0x56, *r, *g, *b, 0x00, 0xF0, 0xAA],
        }
    }

    fn brightness(&self, option: &BrightnessOption) -> Vec<u8> {
        match option {
            BrightnessOption::Level(_level) => {
                unimplemented!("Brightness without ColorOption, not supported yet")
            }
            BrightnessOption::LevelWithColor(level, color) => match color {
                ColorOption::RGB(r, g, b) => {
                    self.color(&ColorOption::RGB((*r as f32 * level) as u8, (*g as f32 * level) as u8, (*b as f32 * level) as u8))
                }
            },
        }
    }
    //-----------//
    // HWAnimate //
    //-----------//
    fn hw_animate(&self, option: &HWAnimateOption) -> Vec<u8> {
        match option {
            HWAnimateOption::Pulsating(color, speed) => {
                vec![
                    0xBB,
                    GenericRGB::_static_color(&color),
                    GenericRGB::_animation_speed(&speed),
                    0x44,
                ]
            }
        }
    }

    fn _animation_speed(setting: &HWAnimationSpeedSetting) -> u8 {
        match setting {
            HWAnimationSpeedSetting::Speed1 => 0x1F,
            HWAnimationSpeedSetting::Speed2 => 0x1B,
            HWAnimationSpeedSetting::Speed3 => 0x1A,
            HWAnimationSpeedSetting::Speed4 => 0x17,
            HWAnimationSpeedSetting::Speed5 => 0x13,
            HWAnimationSpeedSetting::Speed6 => 0x10,
            HWAnimationSpeedSetting::Speed7 => 0x0C,
            HWAnimationSpeedSetting::Speed8 => 0x05,
            HWAnimationSpeedSetting::Speed9 => 0x01,
        }
    }

    fn _static_color(color: &HWStaticColorOption) -> u8 {
        match color {
            HWStaticColorOption::Red => 0x26,
            HWStaticColorOption::Green => 0x27,
            HWStaticColorOption::Blue => 0x28,
        }
    }
}
