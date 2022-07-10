//! Ray scattering utilities
use glam::Vec3;

/// Returns a reflected ray direction based on the given normal
///
/// Performs the following computation: `v - 2 * v.dot(n) * n`
pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - n * v.dot(n) * 2.0
}

/// Returns a refracted ray direction using the given normal
/// and the ratio between two refractive indices.
///
/// See [Shirley's RTiOW's section on Snell's Law](https://raytracing.github.io/books/RayTracingInOneWeekend.html#dielectrics/snell'slaw) for more information
pub fn refract(uv: Vec3, n: Vec3, eta_ratio: f32) -> Vec3 {
    let cos_theta = (-uv).dot(n).min(1.0);
    let r_perp = eta_ratio * (uv + cos_theta * n);
    let r_para = (1.0 - r_perp.length_squared()).abs().sqrt() * -1.0 * n;
    r_perp + r_para
}
