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
        let b = 2.0 * oc.dot(r.direction);
        let c = oc.length_squared() - self.radius * self.radius;
        let discrim = b * b - 4.0 * a * c;
        if discrim > 0.0 {
            (-b - discrim.sqrt()) / (2.0 * a)
        } else {
            -1.0
        }
    }
}
