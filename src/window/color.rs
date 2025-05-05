/// Holds RGB color values
pub struct RGBColor(pub u8, pub u8, pub u8);

impl RGBColor {
    /// Converts an RGB color into a single decimal number
    pub fn as_decimal(&self) -> u32 {
        rgb_to_decimal(self.0 as u32, self.1 as u32, self.2 as u32)
    }
}

#[inline]
fn rgb_to_decimal(r: u32, g: u32, b: u32) -> u32 {
    ((r << 16) + (g << 8) + b) as u32
}
