//! Bounding Volume Hierarchy

use std::{cmp::Ordering, fmt::{Debug, Pointer}, rc::Rc};

use glam::Vec3;

use crate::{
    bounds::Aabb,
    hittable::{HitRecord, Hittable, HittableList},
    rand_util::rand_range_usize,
    ray::Ray,
};

pub struct BvhNode {
    left: Rc<dyn Hittable>,
    right: Rc<dyn Hittable>,
    bbox: Aabb,
}

fn box_cmp(a: &Option<Aabb>, b: &Option<Aabb>, axis_idx: usize) -> Option<Ordering> {
    match (a, b) {
        (None, None) => {
            eprintln!("box_cmp encountered two unbounded objects");
            None
        }
        (None, Some(_)) => Some(Ordering::Less),
        (Some(_), None) => Some(Ordering::Greater),
        (Some(a_box), Some(b_box)) => a_box.min[axis_idx].partial_cmp(&b_box.min[axis_idx]),
    }
}

impl BvhNode {
    pub fn new(mut hitlist: HittableList, time0: f32, time1: f32) -> Self {
        BvhNode::new_node(&mut hitlist[..], time0, time1)
    }

    fn new_node(
        hitlist: &mut [Rc<dyn Hittable>],
        time0: f32,
        time1: f32,
    ) -> Self {
        if hitlist.is_empty() {
            panic!("Given empty scene!");
        }

        let span = hitlist.len();
        let start = 0;

        let (left, right) = match span {
            1 => (hitlist[start].clone(), hitlist[start].clone()),
            2 => (hitlist[start].clone(), hitlist[start + 1].clone()),
            _ => {
                // TODO implement better axis decision-making
                let axis_idx = rand_range_usize(0, 3);

                hitlist.sort_by(|a, b| {
                    box_cmp(
                        &a.bounding_box(time0, time1),
                        &b.bounding_box(time0, time1),
                        axis_idx,
                    )
                    .unwrap()
                });

                let mid = start + span / 2;

                let (half0, half1) = hitlist.split_at_mut(mid);

                let left = BvhNode::new_node(half0, time0, time1);
                let right = BvhNode::new_node(half1, time0, time1);
                let wrapped_left: Rc<dyn Hittable> = left.wrap();
                let wrapped_right: Rc<dyn Hittable> = right.wrap();
                (wrapped_left, wrapped_right)
            }
        };

        let bbox = match (
            &left.bounding_box(time0, time1),
            &right.bounding_box(time0, time1),
        ) {
            (None, None) => {
                eprintln!("new_node encountered two unbounded objects");
                Aabb::new(Vec3::ZERO, Vec3::ZERO)
            }
            (None, Some(b)) => *b,
            (Some(a), None) => *a,
            (Some(a), Some(b)) => a.union(b),
        };

        Self { left, right, bbox }
    }
}

impl Debug for BvhNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BvhNode {{{:?}}}", self.bbox)
    }
}

impl Hittable for BvhNode {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        if self.bbox.hit(ray, t_min, t_max) {
            let left_hit = self.left.hit(ray, t_min, t_max);
            let right_hit = self.right.hit(ray, t_min, t_max);
            match (left_hit, right_hit) {
                (None, None) => None,
                (None, Some(r_rec)) => Some(r_rec),
                (Some(l_rec), None) => Some(l_rec),
                (Some(l_rec), Some(r_rec)) => {
                    if l_rec.t < r_rec.t {
                        Some(l_rec)
                    } else {
                        Some(r_rec)
                    }
                }
            }
        } else {
            None
        }
    }

    fn bounding_box(&self, _time0: f32, _time1: f32) -> Option<Aabb> {
        Some(self.bbox)
    }
}
