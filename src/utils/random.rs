//! Miscelleanous utilities related to random number generation and random sampling
//!
//! Relies on the [rand] and [rand_distr] crates

use glam::Vec3A;
use rand::Rng;
use rand_distr::{Distribution, UnitDisc, UnitSphere};

/// Generates a random [Vec3A] within the unit sphere (radius 1).
///
/// wrapper function around [UnitSphere]'s `sample` method
pub fn rand_vec3_in_unit_sphere(rng: &mut impl Rng) -> Vec3A {
    let arr = UnitSphere.sample(rng);
    Vec3A::from_array(arr)
}

/// Generates a random [Vec3A] within the same unit hemisphere as the given normal.
pub fn rand_vec3_in_unit_hemisphere(rng: &mut impl Rng, normal: Vec3A) -> Vec3A {
    let mut unit_v = rand_vec3_in_unit_sphere(rng);
    if unit_v.dot(normal) < 0.0 {
        unit_v = -unit_v;
    }

    unit_v
}

/// Generates a random [Vec3A] within the unit disk (radius 1).
///
/// wrapper function around [UnitDisc]'s `sample` method.
pub fn rand_vec3_in_unit_disk(rng: &mut impl Rng) -> Vec3A {
    let [x, y] = UnitDisc.sample(rng);
    Vec3A::new(x, y, 0.0)
}
