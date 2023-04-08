/// A simple struct for representing color in an RGB format. Each channel has a value between 0-255
/// as is common in most image displaying systems
#[derive(Copy, Clone, Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    /// Returns a new color with the given r, g, and b values
    pub fn new(r: u8, g: u8, b: u8) -> Color {
        Color { r, g, b }
    }

    /// Returns the `r`, `g`, and `b` values as a single array.
    ///
    /// Useful for pushing color data into a pixel buffer
    pub fn data(self) -> [u8; 3] {
        [self.r, self.g, self.b]
    }
}
