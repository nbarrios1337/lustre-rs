use glam::Vec3;

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
        let dir_n = r.direction.normalize_or_zero();
        let t = 0.5 * (dir_n.y + 1.0);
        let color_vec = (1.0 - t) * Vec3::ONE + t * Vec3::new(0.5, 0.7, 1.0);
        let color_arr: [f32; 3] = color_vec.into();

        Self([color_arr[0] as u8, color_arr[1] as u8, color_arr[2] as u8])
    }
}
