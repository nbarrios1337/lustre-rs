//! Implementation of a camera
//!
//! # Features
//! * positionable and orientable - Using the `look_from`, `look_at`, and `view_up` triplet of vectors
//! * resizable film - Using `aspect_ratio`
//! * depth of field (aka defocus blur) - Using the `aperture` and `focus_dist` data

use glam::Vec3;

use crate::{
    utils::random::{rand_range_f32, rand_vec3_in_unit_disk},
    ray::Ray,
};

/// A Camera that generates rays
#[derive(Debug)]
pub struct Camera {
    /// Camera position in space
    origin: Vec3,
    /// Position of the viewport's lower left corner
    ll_corner: Vec3,
    /// Horizontal 'size' of the viewport
    horizontal: Vec3,
    /// Vertical 'size' of the viewport
    vertical: Vec3,
    /// Orthonormal base 1
    u: Vec3,
    /// Orthonormal base 2
    v: Vec3,
    /// Orthonormal base 3, works like focal length
    w: Vec3,
    /// Radius of the approximated camera lens
    lens_radius: f32,
    /// Shutter open time
    shutter_open: f32,
    /// Shutter close time
    shutter_close: f32,
}

impl Camera {
    /// Creates a new Camera
    ///
    /// # Arguments
    /// * look_from - A [Vec3] holding the position of the camera
    /// * look_at - A [Vec3] holding the eye direction of the camera
    /// * view_up - A [Vec3] holding the "up" direction of the camera
    /// * vert_fov - The vertical field of view
    /// * aspect_ratio - The aspect ratio of the viewport
    /// * aperture - How "big" the approximated lens is
    /// * focus_dist - The distance to the plane in space where objects are "in focus"
    pub fn new(
        look_from: Vec3,
        look_at: Vec3,
        view_up: Vec3,
        vert_fov: f32,
        aspect_ratio: f32,
        aperture: f32,
        focus_dist: f32,
        shutter_open: f32,
        shutter_close: f32,
    ) -> Self {
        // Set up viewport
        let theta = vert_fov.to_radians();
        let viewport_h = 2.0 * (theta / 2.0).tan();
        let viewport_w = aspect_ratio * viewport_h;

        // Set up position
        let w = (look_from - look_at).normalize();
        let u = view_up.cross(w).normalize();
        let v = w.cross(u);

        let origin = look_from;
        let horizontal = viewport_w * focus_dist * u;
        let vertical = viewport_h * focus_dist * v;
        let ll_corner = origin - horizontal / 2.0 - vertical / 2.0 - focus_dist * w;

        let lens_radius = aperture / 2.0;
        Self {
            origin,
            ll_corner,
            horizontal,
            vertical,
            u,
            v,
            w,
            lens_radius,
            shutter_open,
            shutter_close,
        }
    }

    /// Returns a ray from the camera for the normalized pixel (u,v)
    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        let rd = self.lens_radius * rand_vec3_in_unit_disk();
        let offest = self.u * rd.x + self.v * rd.y;
        Ray {
            origin: self.origin + offest,
            direction: self.ll_corner + u * self.horizontal + v * self.vertical
                - self.origin
                - offest,
            time: rand_range_f32(self.shutter_open, self.shutter_close),
        }
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self::new(
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, -1.0),
            Vec3::new(0.0, 1.0, 0.0),
            90.0,
            16.0 / 9.0,
            0.1,
            10.0,
            0.0,
            0.0,
        )
    }
}
