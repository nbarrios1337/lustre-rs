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

    fn slab_interval(&self, ray: &Ray, t_min: f32, t_max: f32, axis_idx: usize) -> (f32, f32) {
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
        (t_near, t_far)
    }

    pub fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> bool {
        for axis in 0..3 {
            let (t_near, t_far) = self.slab_interval(ray, t_min, t_max, axis);
            if t_near <= t_far {
                return false;
            }
        }

        true
    }

    pub fn intersection(&self, _other: &Aabb) -> Aabb {
        todo!()
    }

    pub fn union(&self, other: &Aabb) -> Aabb {
        let min = self.min.min(other.min);
        let max = self.max.max(other.max);
        Self { min, max }
    }
}
