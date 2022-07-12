//! Miscelleanous utilities related to random number generation and random sampling
//!
//! Relies on the [rand] crate

use glam::Vec3;
use rand::Rng;
use rand_distr::{Distribution, UnitDisc, UnitSphere};

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
/// wrapper function around [UnitSphere]'s sample method
pub fn rand_vec3_in_unit_sphere(rng: &mut impl Rng) -> Vec3 {
    Vec3::from_array(UnitSphere.sample(rng))
}

/// Generates a random [Vec3] within the unit disk (radius 1).
///
/// wrapper function around [UnitDisc]'s `sample` method.
pub fn rand_vec3_in_unit_disk(rng: &mut impl Rng) -> Vec3 {
    let [x,y] = UnitDisc.sample(rng);
    Vec3::new(x, y, 0.0)
}

/// Returns the output from [rand_vec3_in_unit_sphere], normalized.
pub fn rand_unit_vec3(rng: &mut impl Rng) -> Vec3 {
    // debug_assert_eq!(rand_vec3_in_unit_sphere(rng), rand_vec3_in_unit_sphere(rng).normalize());
    rand_vec3_in_unit_sphere(rng).normalize()
}
