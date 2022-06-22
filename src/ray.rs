use glam::Vec3;

use crate::sphere::Sphere;

#[derive(Debug, Clone, Copy, Default)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    pub fn at(self, t: f32) -> Vec3 {
        self.origin + t * self.direction
    }
}

impl From<Ray> for image::Rgb<u8> {
    fn from(r: Ray) -> Self {
        let center = Vec3::new(0.0, 0.0, -1.0);
        let t = Sphere::new(center, 0.5).hit(&r);
        let color_vec: Vec3 = if t > 0.0 {
            // shade with surface normal
            let normed = (r.at(t) - center).normalize_or_zero();
            (normed + 1.0) * 0.5
        } else {
            // linearly interpolate from white to blue-ish
            let dir_n = r.direction.normalize_or_zero();
            let t = 0.5 * (dir_n.y + 1.0);
            Vec3::ONE.lerp(Vec3::new(0.5, 0.7, 1.0), t)
        };

        Self(
            (color_vec.clamp(Vec3::ZERO, Vec3::ONE) * 256.0)
                .to_array()
                .iter()
                .map(|&x| x as u8)
                .collect::<Vec<u8>>()
                .try_into()
                .unwrap_or([0, 0, 0]),
        )
    }
}
