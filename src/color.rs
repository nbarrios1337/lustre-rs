//! Convenience newtype for color pixel output

use glam::Vec3;

/// Wrapper around [Vec3] to enable [Vec3] -> [image::Rgb] conversion
///
/// See "[The Newtype Pattern In Rust](https://www.worthe-it.co.za/blog/2020-10-31-newtype-pattern-in-rust.html)" article for more info
#[derive(Debug, Clone, Copy)]
pub struct Color {
    value: Vec3,
}

impl Color {
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
