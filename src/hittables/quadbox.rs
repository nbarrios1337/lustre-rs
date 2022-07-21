use std::sync::Arc;

use glam::Vec3A;

use crate::{bounds::BoundingBox, material::Material};

use super::{HitRecord, Hittable, Quad};

#[derive(Debug)]
pub struct QuadBox {
    min: Vec3A,
    max: Vec3A,
    sides: Vec<Quad>,
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
    pub fn new(min: Vec3A, max: Vec3A, m: &Arc<Material>) -> Self {
        let min = min.min(max);
        let max = min.max(max);

        let side0 = Quad::from_bounds_k(min.x, max.x, min.y, max.y, max.z, 2, m);
        let side1 = Quad::from_bounds_k(min.x, max.x, min.y, max.y, min.z, 2, m);

        let side2 = Quad::from_bounds_k(min.x, max.x, min.z, max.z, max.y, 1, m);
        let side3 = Quad::from_bounds_k(min.x, max.x, min.z, max.z, min.y, 1, m);

        let side4 = Quad::from_bounds_k(min.y, max.y, min.z, max.z, max.x, 0, m);
        let side5 = Quad::from_bounds_k(min.y, max.y, min.z, max.z, min.x, 0, m);

        let sides: Vec<Quad> = vec![side0, side1, side2, side3, side4, side5];

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
