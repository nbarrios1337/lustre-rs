use glam::Vec3;
use noise::NoiseFn;

use crate::{color::Color, textures::Texture};

impl Texture for dyn NoiseFn<[f64; 3]> {
    fn color(&self, _u: f32, _v: f32, point: Vec3) -> Color {
        let noise_val = self.get((4.0 * point).as_dvec3().to_array());
        Color::new(Vec3::ONE * 0.5 * (1.0 + noise_val) as f32)
    }
}
