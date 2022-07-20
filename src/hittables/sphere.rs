//! Implementations of Sphere-like objects

use std::{
    f32::consts::{PI, TAU},
    sync::Arc,
};

use glam::Vec3;

use crate::{
    bounds::BoundingBox,
    hittables::{HitRecord, Hittable},
    material::Material,
    ray::Ray,
};

/// A Sphere object
#[derive(Debug)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub material: Arc<Material>,
}

impl Sphere {
    // Creates a new Sphere.
    pub fn new(c: Vec3, r: f32, m: &Arc<Material>) -> Self {
        Self {
            center: c,
            radius: r,
            material: Arc::clone(m),
        }
    }

    /// Returns the uv surface coordinates for a point on the sphere
    fn surface_coords(&self, point: Vec3) -> (f32, f32) {
        let theta = (-point.y).acos();
        let phi = (-point.z).atan2(point.x) + PI;
        let u = phi / TAU;
        let v = theta / PI;
        (u, v)
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
            // ray is outside sphere
            (true, outward_n)
        } else {
            // ray is inside sphere
            (false, -outward_n)
        };

        let material = self.material.clone();
        let (u, v) = self.surface_coords(outward_n);

        Some(HitRecord {
            point,
            normal,
            material,
            t,
            u,
            v,
            front_face,
        })
    }

    fn bounding_box(&self, _time0: f32, _time1: f32) -> Option<crate::bounds::BoundingBox> {
        Some(BoundingBox::new(
            self.center - Vec3::splat(self.radius),
            self.center + Vec3::splat(self.radius),
        ))
    }
}

/// Like [Sphere], but it moves.
#[derive(Debug)]
pub struct MovingSphere {
    center0: Vec3,
    center1: Vec3,
    time0: f32,
    time1: f32,
    radius: f32,
    pub material: Arc<Material>,
}

impl MovingSphere {
    /// Creates a new MovingSphere.
    pub fn new(
        center0: Vec3,
        center1: Vec3,
        time0: f32,
        time1: f32,
        radius: f32,
        m: &Arc<Material>,
    ) -> Self {
        Self {
            center0,
            center1,
            time0,
            time1,
            radius,
            material: Arc::clone(m),
        }
    }

    /// determines the point in space of the center of the sphere
    fn center(&self, time: f32) -> Vec3 {
        let ratio = (time - self.time0) / (self.time1 - self.time0);
        let offset = ratio * (self.center1 - self.center0);
        self.center0 + offset
    }

    /// Returns the uv surface coordinates for a point on the sphere
    fn surface_coords(&self, point: Vec3) -> (f32, f32) {
        let theta = (-point.y).acos();
        let phi = (-point.z).atan2(point.x) + PI;
        let u = phi / TAU;
        let v = theta / PI;
        (u, v)
    }
}

impl Hittable for MovingSphere {
    // a copy/paste of Sphere::hit, but uses the center() fn instead of a center field.
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = ray.origin - self.center(ray.time);
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
        let outward_n = (point - self.center(ray.time)) / self.radius;
        let (front_face, normal) = if ray.direction.dot(outward_n) < 0.0 {
            // ray is outside sphere
            (true, outward_n)
        } else {
            // ray is inside sphere
            (false, -outward_n)
        };

        let material = self.material.clone();
        let (u, v) = self.surface_coords(outward_n);

        Some(HitRecord {
            point,
            normal,
            material,
            t,
            u,
            v,
            front_face,
        })
    }

    fn bounding_box(&self, time0: f32, time1: f32) -> Option<BoundingBox> {
        let box0 = BoundingBox::new(
            self.center(time0) - Vec3::splat(self.radius),
            self.center(time0) + Vec3::splat(self.radius),
        );
        let box1 = BoundingBox::new(
            self.center(time1) - Vec3::splat(self.radius),
            self.center(time1) + Vec3::splat(self.radius),
        );

        Some(box0.union(&box1))
    }
}
