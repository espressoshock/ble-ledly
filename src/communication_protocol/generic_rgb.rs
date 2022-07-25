use crate::{
    capability::{
        animate::{AnimateOption, AnimationSpeedSetting, StaticColorOption},
        brightness::BrightnessOption,
        color::ColorOption,
        light::LightOption,
    },
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

    fn brightness(&self, option: BrightnessOption) -> Vec<u8> {
        match option {
            BrightnessOption::Level(level) => {
                unimplemented!("Brightness without ColorOption, not supported yet")
            }
            BrightnessOption::LevelWithColor(level, color) => match color {
                ColorOption::RGB(r, g, b) => {
                    self.color(ColorOption::RGB(r * level, g * level, b * level))
                }
            },
        }
    }
    //---------//
    // Animate //
    //---------//
    fn animate(&self, option: AnimateOption) -> Vec<u8> {
        match option {
            AnimateOption::Pulsating(color, speed) => {
                vec![
                    0xBB,
                    GenericRGB::_static_color(&color),
                    GenericRGB::_animation_speed(&speed),
                    0x44,
                ]
            }
        }
    }

    fn _animation_speed(setting: &AnimationSpeedSetting) -> u8 {
        match setting {
            AnimationSpeedSetting::Speed1 => 0x1F,
            AnimationSpeedSetting::Speed2 => 0x1B,
            AnimationSpeedSetting::Speed3 => 0x1A,
            AnimationSpeedSetting::Speed4 => 0x17,
            AnimationSpeedSetting::Speed5 => 0x13,
            AnimationSpeedSetting::Speed6 => 0x10,
            AnimationSpeedSetting::Speed7 => 0x0C,
            AnimationSpeedSetting::Speed8 => 0x05,
            AnimationSpeedSetting::Speed9 => 0x01,
        }
    }

    fn _static_color(color: &StaticColorOption) -> u8 {
        match color {
            StaticColorOption::Red => 0x26,
            StaticColorOption::Green => 0x27,
            StaticColorOption::Blue => 0x28,
        }
    }
}
