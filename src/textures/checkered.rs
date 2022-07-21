//! A texture mapping alternating between two other Textures in a checkerboard fashion.

use std::sync::Arc;

use glam::Vec3A;

use crate::color::Color;

use super::Texture;

/// A checkered texture alternating between two enclosed textures.
pub struct Checkered {
    pub even: Arc<dyn Texture>,
    pub odd: Arc<dyn Texture>,
}

impl Checkered {
    /// Creates a new checkered texture
    pub fn new(o: &Arc<dyn Texture>, e: &Arc<dyn Texture>) -> Self {
        Self {
            even: Arc::clone(e),
            odd: Arc::clone(o),
        }
    }
}

impl Texture for Checkered {
    fn color(&self, u: f32, v: f32, point: Vec3A) -> Color {
        let sin_x = (point * 10.0).x.sin();
        let sin_y = (point * 10.0).y.sin();
        let sin_z = (point * 10.0).z.sin();

        if sin_x * sin_y * sin_z < 0.0 {
            self.odd.color(u, v, point)
        } else {
            self.even.color(u, v, point)
        }
    }
}
