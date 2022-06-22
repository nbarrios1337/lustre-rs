use glam::Vec3;

use crate::ray::Ray;

#[derive(Debug)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32) -> Self {
        Self { center, radius }
    }

    pub fn hit(self, r: &Ray) -> f32 {
        let oc = r.origin - self.center;
        // dot product of a vector with itself is the length squared
        let a = r.direction.length_squared();
        let half_b = oc.dot(r.direction);
        let c = oc.length_squared() - self.radius * self.radius;
        let discrim = half_b * half_b - a * c;
        if discrim > 0.0 {
            (-half_b - discrim.sqrt()) / a
        } else {
            -1.0
        }
    }
}
