//! A texture mapping backed by procedural Perlin noise

use ::noise::{NoiseFn, Perlin};
use glam::Vec3;

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
    fn color(&self, _u: f32, _v: f32, point: Vec3) -> Color {
        let noise = &self.noise as &dyn NoiseFn<[f64; 3]>;
        let noise_val = noise.get((self.scale * point).as_dvec3().to_array());
        Color::new(Vec3::ONE * 0.5 * (1.0 + noise_val) as f32)
    }
}
