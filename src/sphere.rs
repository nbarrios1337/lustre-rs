use glam::Vec3;

use crate::{
    hittable::{HitRecord, Hittable},
    ray::Ray,
};

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

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        // dot product of a vector with itself is the length squared
        let a = ray.direction.length_squared();
        let half_b = oc.dot(ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;

        let discrim = half_b * half_b - a * c;
        if discrim < 0.0 {
            return None;
        }

        let mut root = (-half_b - discrim.sqrt()) / a;
        if t_min > root || root > t_max {
            root = (-half_b + discrim.sqrt()) / a;
            if t_min > root || root > t_max {
                return None;
            }
        }

        let t = root;
        let point = ray.at(t);
        let outward_n = (point - self.center) / self.radius;
        let (front_face, normal) = if ray.direction.dot(outward_n) < 0.0 {
            (true, outward_n)
        } else {
            (false, -outward_n)
        };
        
        Some(HitRecord {
            t,
            point,
            normal,
            front_face,
        })
    }
}
