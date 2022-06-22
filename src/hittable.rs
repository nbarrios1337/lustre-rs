use std::rc::Rc;

use glam::Vec3;

use crate::ray::Ray;

#[derive(Debug)]
pub struct HitRecord {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, ray: &Ray, outward_n: Vec3) {
        if ray.direction.dot(outward_n) < 0.0 {
            self.front_face = true;
            self.normal = outward_n;
        } else {
            self.front_face = false;
            self.normal = -outward_n;
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

pub struct HittableList {
    pub hittables: Vec<Rc<dyn Hittable>>,
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut rec: Option<HitRecord> = None;
        let mut t_closest = t_max;

        for hittable in self.hittables.iter() {
            rec = hittable.hit(ray, t_min, t_closest);
            if let Some(HitRecord { t, .. }) = rec {
                t_closest = t;
            }
        }

        rec
    }
}
