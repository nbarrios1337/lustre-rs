use std::f32::{EPSILON, INFINITY};

use glam::Vec3;

use crate::{color::Color, hittable::Hittable, rand_util::rand_vec3_in_unit_sphere};

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

        let v = match hittable.hit(self, EPSILON, INFINITY) {
            Some(rec) => {
                let new_target = rec.point + rec.normal + rand_vec3_in_unit_sphere();
                let bounce = Ray::new(rec.point, new_target - rec.point);
                Vec3::from(bounce.shade(hittable, bounce_depth - 1)) * 0.5
            }
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
