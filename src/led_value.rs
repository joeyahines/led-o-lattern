
/// Stores RGB output values
#[derive(Debug)]
pub struct LEDValue {
    /// Red brightness
    pub red: u8,
    /// Green brightness
    pub green: u8,
    /// Blue brightness
    pub blue: u8,
    /// Count state
    pub count_state: u8,
}

impl From<u32> for LEDValue {
    fn from(v: u32) -> Self {
        Self {
            red: (v & 0x00000000FF) as u8,
            green: ((v & 0x0000FF00) >> 8) as u8,
            blue: ((v & 0x00FF0000) >> 16) as u8,
            count_state: ((v & 0xFF000000) >> 24) as u8,
        }
    }
}

impl Into<u32> for LEDValue {
    fn into(self) -> u32 {
        (self.red as u32) | ((self.green as u32) << 8) | ((self.blue as u32) << 16) | ((self.count_state as u32) << 24)
    }
}
