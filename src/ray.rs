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

    pub fn shade(&self, hittable: &impl Hittable, bounce_depth: u16) -> Color {
        // Limit recursion depth
        if bounce_depth == 0 {
            return Color::from(Vec3::ZERO);
        }

        let v = match hittable.hit(self, 0.0001, INFINITY) {
            Some(rec) => match rec.material.scatter(self, &rec) {
                Some((scattered, attenuation)) => {
                    attenuation * Vec3::from(scattered.shade(hittable, bounce_depth - 1))
                }
                None => Vec3::ZERO,
            },
            None => {
                // linearly interpolate from white to blue-ish
                let dir_n = self.direction.normalize_or_zero();
                let t = 0.5 * (dir_n.y + 1.0);
                Vec3::ONE.lerp(Vec3::new(0.5, 0.7, 1.0), t)
            }
        };

        Color::from(v)
    }
}
