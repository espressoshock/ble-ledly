// use super::traits::Encode;

// TODO: more meaningf&ul name
pub enum AnimationSpeedSetting {
    Speed1,
    Speed2,
    Speed3,
    Speed4,
    Speed5,
    Speed6,
    Speed7,
    Speed8,
    Speed9,
}

pub enum PulsatingColor {
    Red,
    Green,
    Blue,
}

fn animation_speed(setting: &AnimationSpeedSetting) -> u8 {
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

fn pulsating_color(color: &PulsatingColor) -> u8 {
    match color {
        PulsatingColor::Red => 0x26,
        PulsatingColor::Green => 0x27,
        PulsatingColor::Blue => 0x28,
    }
}

pub trait GenericRGBLight {
    fn turn_off(&self) -> Vec<u8> {
        // this doesn't work when
        // HWspecific effects are turn_on
        // use legacy mode instead
        // vec![0xcc, 0x24, 0x33]
        self.encode_color(0, 0, 0)
    }

    fn turn_on(&self) -> Vec<u8> {
        vec![0xcc, 0x23, 0x33]
    }

    fn pulsating(&self, color: &PulsatingColor, speed_setting: &AnimationSpeedSetting) -> Vec<u8> {
        vec![
            0xBB,
            pulsating_color(color),
            animation_speed(speed_setting),
            0x44,
        ]
    }

    // TODO:
    // remove and implement encode trait and error handling
    fn encode_color(&self, red: u8, green: u8, blue: u8) -> Vec<u8> {
        vec![0x56, red, green, blue, 0x00, 0xF0, 0xAA]
    }
}
