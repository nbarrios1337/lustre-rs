use glam::Vec3;

pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    *v - *n * v.dot(*n) * 2.0
}
