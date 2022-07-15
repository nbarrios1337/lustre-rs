//! Contains description of what it means to intersect something,
//! as well as what's returned on intersection

use std::rc::Rc;

use enum_dispatch::enum_dispatch;
use glam::Vec3;

use crate::{bounds::BoundingBox, material::Material, ray::Ray, bvh::BvhNode};

pub mod list;
pub mod quad;
pub mod quadbox;
pub mod sphere;

pub use list::*;
pub use quad::*;
pub use quadbox::*;
pub use sphere::*;

#[enum_dispatch/* (Hittable) */]
pub enum HitObject {
    HittableList,
    Quad,
    QuadBox,
    Sphere,
    MovingSphere,
    BvhNode,
}

/// Defines a set of data returned upon a successful intersection
#[derive(Debug)]
pub struct HitRecord {
    /// Point of intersection in 3D space
    pub point: Vec3,
    /// Surface normal off the point of intersection
    pub normal: Vec3,
    /// Material of the intersected object
    pub material: Rc<Material>,
    /// distance from the origin to the point of intersection
    pub t: f32,
    /// u coordinate of surface of point of intersection
    pub u: f32,
    /// v coordinate of surface of point of intersection
    pub v: f32,
    /// Whether or not the ray hit the object's inside or outside face
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

impl PartialOrd for HitRecord {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.t.partial_cmp(&other.t)
    }
}

impl PartialEq for HitRecord {
    fn eq(&self, other: &Self) -> bool {
        self.t == other.t
    }
}

/// Describes the behavior of objects that support intersection
#[enum_dispatch(HitObject)]
pub trait Hittable
where
    HitObject: std::convert::From<Self>,
    Self: Sized,
{
    /// Intersects the given ray with the object
    ///
    /// Returns a `Some(HitRecord)` if successful, otherwise `None`
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;

    /// Returns the axis aligned bounding box for the object
    ///
    /// Returns a `Some(Aabb)` if the object has a bounding box (like spheres), otherwise `None` (like planes)
    fn bounding_box(&self, time0: f32, time1: f32) -> Option<BoundingBox>;

    fn wrap(self) -> Rc<HitObject>
    where
        Self: Sized,
    {
        Rc::new(self.into())
    }
}
