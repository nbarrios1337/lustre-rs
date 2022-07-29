//! A texture mapping back by any noise function defined in the [noise] crate

use glam::Vec3A;
use noise::NoiseFn;

use super::Texture;
use crate::color::Color;

#[derive(Debug)]
pub struct NoiseTexture<N>
where
    N: NoiseFn<[f64; 3]> + Send + Sync,
{
    noise: N,
    scale: f32,
}

impl<N> NoiseTexture<N>
where
    N: NoiseFn<[f64; 3]> + Send + Sync,
{
    pub fn new(noise: N, scale: f32) -> Self {
        Self { noise, scale }
    }
}

impl<N> Texture for NoiseTexture<N>
where
    N: NoiseFn<[f64; 3]> + Send + Sync,
{
    fn color(&self, _u: f32, _v: f32, point: glam::Vec3A) -> super::SolidColor {
        let noise_val = self.noise.get((self.scale * point).as_dvec3().to_array());
        Color::new(Vec3A::ONE * 0.5 * (1.0 + noise_val) as f32)
    }
}
