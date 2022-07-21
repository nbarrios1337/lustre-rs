//! Implementation of bounding volumes

use glam::Vec3A;

use crate::ray::Ray;

/// An axis aligned bounding box
#[derive(Debug, Clone, Copy)]
pub struct BoundingBox {
    pub min: Vec3A,
    pub max: Vec3A,
}

impl BoundingBox {
    /// Creates a new Axis aligned bounding box
    pub fn new(min: Vec3A, max: Vec3A) -> Self {
        Self { min, max }
    }

    /// Returns whether or not the ray hits this bounding box.
    ///
    /// Checks for slab intersection in each of the 3 dimensions.
    pub fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> bool {
        let inverse_dir = ray.direction.recip();
        let diff0 = self.min - ray.origin;
        let diff1 = self.max - ray.origin;

        // Check for slab intersection in each dimension
        for axis_idx in 0..3 {
            let inverse_dir = inverse_dir[axis_idx];
            let t0 = diff0[axis_idx] * inverse_dir;
            let t1 = diff1[axis_idx] * inverse_dir;

            // swap if inverted
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

    /// Returns a bounding box enclosing this and the other box.
    ///
    /// In other words, combines the two boxes by taking:
    /// * the minimums of the two boxes' min members
    /// * the maximums of the two boxes' max members
    pub fn union(&self, other: &BoundingBox) -> BoundingBox {
        let min = self.min.min(other.min);
        let max = self.max.max(other.max);
        Self { min, max }
    }
}

impl Default for BoundingBox {
    fn default() -> Self {
        Self {
            min: Vec3A::ZERO,
            max: Vec3A::ZERO,
        }
    }
}
