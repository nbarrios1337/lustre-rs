use glam::Vec3;

use crate::{rand_util::rand_vec3_in_unit_disk, ray::Ray};

#[derive(Debug)]
pub struct Camera {
    origin: Vec3,
    ll_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f32,
}

impl Camera {
    pub fn new(
        look_from: Vec3,
        look_at: Vec3,
        view_up: Vec3,
        vert_fov: f32,
        aspect_ratio: f32,
        apeture: f32,
        focus_dist: f32,
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

        let lens_radius = apeture / 2.0;
        Self {
            origin,
            ll_corner,
            horizontal,
            vertical,
            u,
            v,
            w,
            lens_radius,
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        let rd = self.lens_radius * rand_vec3_in_unit_disk();
        let offest = self.u * rd.x + self.v * rd.y;
        Ray {
            origin: self.origin + offest,
            direction: self.ll_corner + u * self.horizontal + v * self.vertical
                - self.origin
                - offest,
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
        )
    }
}
