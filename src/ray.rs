//! The Ray module provies an implementation of a 3-dimensional Ray.

use std::f32::INFINITY;

use glam::Vec3;

use crate::{color::Color, hittable::Hittable};

/// A 3-dimensional Ray
///
/// The crucial parts of the Ray are its origin and direction;
/// these two members are the primary way to determine an intersection with a [`Hittable`]
#[derive(Debug, Clone, Copy, Default)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    /// Creates a new Ray.
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    /// Returns a position in 3D space along the ray.
    /// 
    /// Performs the following calculation: `position = origin + t * direction`
    pub fn at(&self, t: f32) -> Vec3 {
        self.origin + t * self.direction
    }

    /// Returns a [`Color`] value based on the accumulated light and color at the initial intersection point.
    /// 
    /// Uses `bounce_depth` to limit the amount of recursion when gathering contributions.
    pub fn shade(&self, hittable: &impl Hittable, bounce_depth: u16) -> Color {
        // Limit recursion depth
        if bounce_depth == 0 {
            return Color::from(Vec3::ZERO);
        }

        // Check for a hit against the `hittable` parameter
        let v = match hittable.hit(self, 0.0001, INFINITY) {
            // immediately match against the HitRecord's material member
            Some(rec) => match rec.material.scatter(self, &rec) {
                // A successful ray scatter leads to more contributions.
                Some((scattered, attenuation)) => {
                    attenuation * Vec3::from(scattered.shade(hittable, bounce_depth - 1))
                }
                None => Vec3::ZERO,
            },
            // without a hit, functions like a miss shader
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
