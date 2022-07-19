pub trait Encode {
    fn encode_color(&self, red: u8, green: u8, blue: u8) -> Vec<u8>;
}
