//! Implementation of bounding volumes

use glam::Vec3;

use crate::ray::Ray;

/// An axis aligned bounding box
#[derive(Debug, Clone, Copy)]
pub struct Aabb {
    pub min: Vec3,
    pub max: Vec3,
}

impl Aabb {
    /// Creates a new Axis aligned bounding box
    pub fn new(min: Vec3, max: Vec3) -> Self {
        Self { min, max }
    }

    pub fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> bool {
        for axis_idx in 0..3 {
            let inverse_dir = ray.direction.recip()[axis_idx];
            let t0 = (self.min[axis_idx] - ray.origin[axis_idx]) * inverse_dir;
            let t1 = (self.max[axis_idx] - ray.origin[axis_idx]) * inverse_dir;

            // swap
            let (t0, t1) = if inverse_dir < 0.0 {
                (t1, t0)
            } else {
                (t0, t1)
            };

            let t_near = t0.max(t_min);
            let t_far = t1.min(t_max);
            if t_far <= t_near {
                return false;
            }
        }

        true
    }

    pub fn union(&self, other: &Aabb) -> Aabb {
        let min = self.min.min(other.min);
        let max = self.max.max(other.max);
        Self { min, max }
    }
}

impl Default for Aabb {
    fn default() -> Self {
        Self {
            min: Vec3::ZERO,
            max: Vec3::ZERO,
        }
    }
}
