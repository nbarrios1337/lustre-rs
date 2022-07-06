//! Color and pixel output

use glam::Vec3;

/// A RGB color.
///
/// Holds its value as a [Vec3]
#[derive(Debug, Clone, Copy)]
pub struct Color {
    value: Vec3,
}

impl Color {
    /// Creates a new Color
    pub fn new(value: Vec3) -> Self {
        Self { value }
    }
}

impl From<Color> for Vec3 {
    fn from(c: Color) -> Self {
        c.value
    }
}

// The important stuff
impl From<Color> for image::Rgb<u8> {
    fn from(color: Color) -> Self {
        Self(
            (color.value.clamp(Vec3::ZERO, Vec3::ONE) * 256.0)
                .to_array()
                .iter()
                .map(|&x| x as u8)
                .collect::<Vec<u8>>()
                .try_into()
                .unwrap_or([0, 0, 0]),
        )
    }
}

impl From<image::Rgb<u8>> for Color {
    fn from(rgb: image::Rgb<u8>) -> Self {
        let scale = 1.0 / 256.0;
        let scaled = [
            rgb[0] as f32 * scale,
            rgb[1] as f32 * scale,
            rgb[2] as f32 * scale,
        ];
        Self {
            value: Vec3::from(scaled),
        }
    }
}
