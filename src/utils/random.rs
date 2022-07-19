//! Miscelleanous utilities related to random number generation and random sampling
//!
//! Relies on the [rand] and [rand_distr] crates

use glam::Vec3;
use rand::Rng;
use rand_distr::{Distribution, UnitDisc, UnitSphere};

/// Generates a random [Vec3] within the unit sphere (radius 1).
///
/// wrapper function around [UnitSphere]'s `sample` method
pub fn rand_vec3_in_unit_sphere(rng: &mut impl Rng) -> Vec3 {
    let arr = UnitSphere.sample(rng);
    Vec3::from_array(arr)
}

/// Generates a random [Vec3] within the same unit hemisphere as the given normal.
pub fn rand_vec3_in_unit_hemisphere(rng: &mut impl Rng, normal: Vec3) -> Vec3 {
    let arr = UnitSphere.sample(rng);
    let mut unit_v = Vec3::from_array(arr);
    if unit_v.dot(normal) < 0.0 {
        unit_v = -unit_v;
    }

    unit_v
}

/// Generates a random [Vec3] within the unit disk (radius 1).
///
/// wrapper function around [UnitDisc]'s `sample` method.
pub fn rand_vec3_in_unit_disk(rng: &mut impl Rng) -> Vec3 {
    let [x, y] = UnitDisc.sample(rng);
    Vec3::new(x, y, 0.0)
}
