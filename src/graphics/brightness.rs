use macroquad::color::Color;

/// A newtype for the brightness level, max is 255.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Brightness(u8);

impl Default for Brightness {
    fn default() -> Self {
        Self(255)
    }
}

impl From<u8> for Brightness {
    fn from(val: u8) -> Self {
        Self(val)
    }
}

impl From<Brightness> for u8 {
    fn from(val: Brightness) -> Self {
        val.0
    }
}

impl From<Brightness> for Color {
    fn from(val: Brightness) -> Self {
        Self::from_rgba(255, 255, 255, val.into())
    }
}
