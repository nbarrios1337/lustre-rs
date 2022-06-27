use std::{
    ops::{Deref, DerefMut},
    rc::Rc,
};

use glam::Vec3;

use crate::{material::Material, ray::Ray};

#[derive(Debug)]
pub struct HitRecord {
    pub point: Vec3,
    pub normal: Vec3,
    pub material: Rc<Material>,
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

pub struct HittableList(pub Vec<Box<dyn Hittable>>);

impl Deref for HittableList {
    type Target = Vec<Box<dyn Hittable>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for HittableList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut rec: Option<HitRecord> = None;
        let mut t_closest = t_max;

        for hittable in self.iter() {
            let hit_result = hittable.hit(ray, t_min, t_closest);
            if let Some(HitRecord { t, .. }) = hit_result {
                t_closest = t;
                rec = hit_result;
            }
        }
        rec
    }
}
