use std::fmt::Debug;

use glam::Vec3;

use crate::color::Color;

pub trait Texture {
    fn color(&self, u: f32, v: f32, point: Vec3) -> Color;
}

impl Texture for Color {
    fn color(&self, _u: f32, _v: f32, _point: Vec3) -> Color {
        *self
    }
}

impl Debug for dyn Texture {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Texture with Color {:?}",
            self.color(0.0, 0.0, Vec3::ZERO)
        )
    }
}
