use std::ops::{Deref, DerefMut};

use glam::Vec3;

#[derive(Debug)]
pub struct Color(pub Vec3);

impl From<Vec3> for Color {
    fn from(v: Vec3) -> Self {
        Self(v)
    }
}

impl Deref for Color {
    type Target = Vec3;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Color {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<Color> for image::Rgb<u8> {
    fn from(color: Color) -> Self {
        Self(
            (color.clamp(Vec3::ZERO, Vec3::ONE) * 256.0)
                .to_array()
                .iter()
                .map(|&x| x as u8)
                .collect::<Vec<u8>>()
                .try_into()
                .unwrap_or([0, 0, 0]),
        )
    }
}

pub const RED: Color = Color(glam::const_vec3!([1.0, 0.0, 0.0]));
pub const GREEN: Color = Color(glam::const_vec3!([0.0, 1.0, 0.0]));
pub const BLUE: Color = Color(glam::const_vec3!([0.0, 0.0, 1.0]));
