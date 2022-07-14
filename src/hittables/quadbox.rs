use std::rc::Rc;

use glam::Vec3;

use crate::{bounds::BoundingBox, material::Material};

use super::{HitRecord, Hittable, Quad};

#[derive(Debug)]
pub struct QuadBox {
    min: Vec3,
    max: Vec3,
    sides: Vec<Rc<Quad>>,
}

impl QuadBox {
    // min = c1
    // max = c8

    //

    // c1-------c2
    // |         |
    // |   bot   |
    // |         |
    // c3-------c4

    // c5-------c6
    // |         |
    // |   top   |
    // |         |
    // c7-------c8

    /// Creates a new Box comprised of 6 sides stored as [Quad]s
    ///
    /// uses the rather expensive Quad::from_two_points_z fn, determining
    pub fn new(min: Vec3, max: Vec3, m: &Rc<Material>) -> Self {
        let min = min.min(max);
        let max = min.max(max);

        let mut sides: Vec<Rc<Quad>> = Vec::with_capacity(6);

        let s0s1min = Vec3::new(min.x, min.y, 0.0);
        let s0s1max = Vec3::new(max.x, max.y, 0.0);

        let s2s3min = Vec3::new(min.x, 0.0, min.z);
        let s2s3max = Vec3::new(max.x, 0.0, max.z);

        let s4s5min = Vec3::new(0.0, min.y, min.z);
        let s4s5max = Vec3::new(0.0, max.y, max.z);

        let s0 = Quad::from_two_points_z(s0s1min, s0s1max, min.z, m);
        let s1 = Quad::from_two_points_z(s0s1min, s0s1max, max.z, m);

        let s2 = Quad::from_two_points_z(s2s3min, s2s3max, min.y, m);
        let s3 = Quad::from_two_points_z(s2s3min, s2s3max, max.y, m);

        let s4 = Quad::from_two_points_z(s4s5min, s4s5max, min.x, m);
        let s5 = Quad::from_two_points_z(s4s5min, s4s5max, max.x, m);

        sides.push(s0.wrap());
        sides.push(s1.wrap());
        sides.push(s2.wrap());
        sides.push(s3.wrap());
        sides.push(s4.wrap());
        sides.push(s5.wrap());

        Self { min, max, sides }
    }
}

impl Hittable for QuadBox {
    fn bounding_box(&self, _time0: f32, _time1: f32) -> Option<BoundingBox> {
        Some(BoundingBox::new(self.min, self.max))
    }

    fn hit(&self, ray: &crate::ray::Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        // self.sides.hit(ray, t_min, t_max)
        //  copy over HittableList hit impl because the above doesn't work
        let mut rec = None;
        let mut t_closest = t_max;

        for hittable in self.sides.iter() {
            let hit_result = hittable.hit(ray, t_min, t_closest);
            if let Some(HitRecord { t, .. }) = hit_result {
                t_closest = t;
                rec = hit_result;
            }
        }
        rec
    }
}
