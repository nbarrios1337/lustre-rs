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
    const VIEWPORT_HEIGHT: f32 = 2.0;
    const VIEWPORT_WIDTH: f32 = Self::ASPECT_RATIO * Self::VIEWPORT_HEIGHT;
    const FOCAL_LENGTH: f32 = 1.0;

    pub fn new(spp: u16) -> Self {
        let origin = Vec3::ZERO;
        let horizontal = Vec3::new(Self::VIEWPORT_WIDTH, 0.0, 0.0);
        let vertical = Vec3::new(0.0, Self::VIEWPORT_HEIGHT, 0.0);
        let ll_corner =
            origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, Self::FOCAL_LENGTH);
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
        Self::new(1)
    }
}
