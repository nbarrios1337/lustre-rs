use std::f32::INFINITY;

use glam::Vec3;

use crate::{color::Color, hittable::Hittable};

#[derive(Debug, Clone, Copy, Default)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    pub fn at(&self, t: f32) -> Vec3 {
        self.origin + t * self.direction
    }

    pub fn shade(&self, hittable: &impl Hittable) -> Color {
        let v = match hittable.hit(self, 0.0, INFINITY) {
            Some(rec) => {
                // shade with surface normal
                (rec.normal + 1.0) * 0.5
            }
            None => {
                // linearly interpolate from white to blue-ish
                let dir_n = self.direction.normalize_or_zero();
                let t = 0.5 * (dir_n.y + 1.0);
                Vec3::ONE.lerp(Vec3::new(0.5, 0.7, 1.0), t)
            }
        };

        v.into()
    }
}
