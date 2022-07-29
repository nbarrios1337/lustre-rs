//! A texture mapping backed by procedural Perlin noise

use ::noise::{NoiseFn, Perlin};
use glam::Vec3A;

use crate::color::Color;

use super::Texture;

#[derive(Debug)]
pub struct PerlinNoise {
    noise: Perlin,
    scale: f32,
}

impl PerlinNoise {
    pub fn new(scale: f32) -> Self {
        Self {
            noise: Perlin::new(),
            scale,
        }
    }
}

impl Texture for PerlinNoise {
    fn color(&self, _u: f32, _v: f32, point: Vec3A) -> Color {
        let noise_val = self.noise.get((self.scale * point).as_dvec3().to_array());
        Color::new(Vec3A::ONE * 0.5 * (1.0 + noise_val) as f32)
    }
}
