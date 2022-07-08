use std::rc::Rc;

use glam::Vec3;

use crate::{bounds::Aabb, material::Material};

use super::{HitRecord, Hittable};

#[derive(Debug)]
pub struct Quad {
    p0: Vec3,
    p1: Vec3,
    normal: Vec3,
    pub material: Rc<Material>,
}

impl Hittable for Quad {
    fn bounding_box(&self, _time0: f32, _time1: f32) -> Option<crate::bounds::Aabb> {
        Some(Aabb::new(self.p0, self.p1))
    }

    fn hit(&self, ray: &crate::ray::Ray, t_min: f32, t_max: f32) -> Option<super::HitRecord> {
        // check if we intersect the plane containing the quad
        let center = (self.p0 + self.p1) / 2.0;
        // t = (center - ray.o) dot n over ray.dir dot n
        let dir_dot_normal = self.normal.dot(ray.direction);
        if dir_dot_normal < 1e-6 {
            return None;
        }

        let t = (center - ray.origin).dot(self.normal) / dir_dot_normal;

        // Check with t bounds
        if t > t_max || t < t_min {
            return None;
        }

        // if so, compute the coords
        let point = ray.at(t);

        // if the coords lie in the quad, then we've intersected the quad
        if point.cmplt(self.p0).any() || point.cmpgt(self.p1).any() {
            return None;
        }

        let local_coords = point - self.p0;

        Some(HitRecord {
            point,
            normal: self.normal,
            material: Rc::clone(&self.material),
            t,
            u: local_coords.x,
            v: local_coords.y,
            front_face: true,
        })
    }
}
