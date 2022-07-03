//! Bounding Volume Hierarchy

use std::{fmt::Debug, rc::Rc};

use crate::{
    bounds::Aabb,
    hittable::{HitRecord, Hittable},
    ray::Ray,
};

pub struct BvhNode {
    left: Rc<dyn Hittable>,
    right: Rc<dyn Hittable>,
    bbox: Aabb,
}

impl Debug for BvhNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BvhNode {{{:?}}}", self.bbox)
    }
}

impl Hittable for BvhNode {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        if self.bbox.hit(ray, t_min, t_max) {
            None
        } else {
            let left_hit = self.left.hit(ray, t_min, t_max);

            let t_max = match &left_hit {
                Some(rec) => rec.t,
                None => t_max,
            };

            let right_hit = self.right.hit(ray, t_min, t_max);
            if left_hit < right_hit {
                left_hit
            } else {
                right_hit
            }
        }
    }

    fn bounding_box(&self, _time0: f32, _time1: f32) -> Option<Aabb> {
        Some(self.bbox)
    }
}
