use glam::Vec3;

use crate::ray::Ray;

#[derive(Debug)]
pub struct Camera {
    pub origin: Vec3,
    pub ll_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,

    pub spp: u16,
}

impl Camera {
    pub const ASPECT_RATIO: f32 = 16.0 / 9.0;

    pub fn new(
        look_from: Vec3,
        look_at: Vec3,
        view_up: Vec3,
        vert_fov: f32,
        aspect_ratio: f32,
        spp: u16,
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
        let horizontal = viewport_w * u;
        let vertical = viewport_h * v;
        let ll_corner = origin - horizontal / 2.0 - vertical / 2.0 - w;
        Self {
            origin,
            ll_corner,
            horizontal,
            vertical,
            spp,
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray {
            origin: self.origin,
            direction: self.ll_corner + u * self.horizontal + v * self.vertical - self.origin,
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
            16,
        )
    }
}
