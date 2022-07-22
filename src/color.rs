//! Color and pixel output

use glam::Vec3A;

/// A RGB color.
///
/// Holds its value as a [Vec3A]
#[derive(Debug, Clone, Copy)]
pub struct Color {
    value: Vec3A,
}

impl Color {
    /// Creates a new Color
    pub fn new(value: Vec3A) -> Self {
        Self { value }
    }
}

impl From<Color> for Vec3A {
    fn from(c: Color) -> Self {
        c.value
    }
}

// The important stuff
impl From<Color> for image::Rgb<u8> {
    fn from(color: Color) -> Self {
        let converted = color.value.clamp(Vec3A::ZERO, Vec3A::ONE) * 255.0;
        Self([converted.x as u8, converted.y as u8, converted.z as u8])
    }
}

impl From<image::Rgb<u8>> for Color {
    fn from(rgb: image::Rgb<u8>) -> Self {
        let scale = 1.0 / 255.0;
        Self {
            value: Vec3A::from_array([
                rgb[0] as f32 * scale,
                rgb[1] as f32 * scale,
                rgb[2] as f32 * scale,
            ]),
        }
    }
}
