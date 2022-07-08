//! Miscelleanous utilities related to random number generation and random sampling
//!
//! Relies on the [rand] crate

use glam::Vec3;
use rand::Rng;

/// Generates a random f32.
pub fn rand_f32() -> f32 {
    rand::thread_rng().gen::<f32>()
}

/// Generates a random usize.
pub fn rand_usize() -> usize {
    rand::thread_rng().gen::<usize>()
}

/// Generates a random f32 within the given range `[min, max)`.
pub fn rand_range_f32(min: f32, max: f32) -> f32 {
    rand::thread_rng().gen_range(min..max)
}

/// Generates a random usize within the given range `[min, max)`.
pub fn rand_range_usize(min: usize, max: usize) -> usize {
    rand::thread_rng().gen_range(min..max)
}

/// Generates a random [Vec3].
pub fn rand_vec3() -> Vec3 {
    rand::thread_rng().gen::<Vec3>()
}

/// Generates a random [Vec3] within the given range `[min, max)`.
pub fn rand_range_vec3(min: f32, max: f32) -> Vec3 {
    Vec3::new(
        rand_range_f32(min, max),
        rand_range_f32(min, max),
        rand_range_f32(min, max),
    )
}

/// Generates a random [Vec3] within the unit sphere (radius 1).
///
/// Uses [rand_range_vec3] to generate [Vec3]'s within [-1, 1),
/// rejecting those who's squared length is greater than 1.
pub fn rand_vec3_in_unit_sphere() -> Vec3 {
    loop {
        let v = rand_range_vec3(-1.0, 1.0);
        if v.length_squared() < 1.0 {
            return v;
        }
    }
}

/// Generates a random [Vec3] within the unit disk (radius 1).
///
/// Functionally similar to [rand_vec3_in_unit_sphere].
pub fn rand_vec3_in_unit_disk() -> Vec3 {
    loop {
        let v = Vec3::new(rand_range_f32(-1.0, 1.0), rand_range_f32(-1.0, 1.0), 0.0);
        if v.length_squared() < 1.0 {
            return v;
        }
    }
}

pub fn rand_unit_vec3() -> Vec3 {
    rand_vec3_in_unit_sphere().normalize()
}
