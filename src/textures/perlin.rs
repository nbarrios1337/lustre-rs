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
        noise.color(_u, _v, self.scale * point)
    }
}
