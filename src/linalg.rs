use glam::Vec3;

pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    *v - *n * v.dot(*n) * 2.0
}

pub fn refract(uv: &Vec3, n: &Vec3, eta_ratio: f32) -> Vec3 {
    let cos_theta = (-*uv).dot(*n).min(1.0);
    let r_perp = eta_ratio * (*uv + cos_theta * *n);
    let r_para = (1.0 - r_perp.length_squared()).abs().sqrt() * -1.0 * *n;
    r_perp + r_para
}
