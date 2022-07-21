//! Implementation of a 3-dimensional Ray.

use std::f32::INFINITY;

use glam::Vec3A;
use rand::Rng;

use crate::{color::Color, hittables::Hittable};

/// A 3-dimensional Ray
///
/// The crucial parts of the Ray are its origin and direction;
/// these two members are the primary way to determine an intersection with a [`Hittable`]
#[derive(Debug, Clone, Copy, Default)]
pub struct Ray {
    pub origin: Vec3A,
    pub direction: Vec3A,
    pub time: f32,
}

impl Ray {
    /// Creates a new Ray.
    pub fn new(origin: Vec3A, direction: Vec3A, time: f32) -> Self {
        Self {
            origin,
            direction,
            time,
        }
    }

    /// Returns a position in 3D space along the ray.
    ///
    /// Performs the following calculation: `position = origin + t * direction`
    pub fn at(&self, t: f32) -> Vec3A {
        self.origin + t * self.direction
    }

    /// Returns a [`Color`] value based on the accumulated light and color at the initial intersection point.
    ///
    /// Uses `bounce_depth` to limit the amount of recursion when gathering contributions.
    pub fn shade(
        &self,
        hittable: &impl Hittable,
        bounce_depth: u16,
        bg_color: Color,
        rng: &mut impl Rng,
    ) -> Color {
        // Limit recursion depth
        if bounce_depth == 0 {
            return Color::new(Vec3A::ZERO);
        }

        // Check for a hit against the `hittable` parameter
        match hittable.hit(self, 0.001, INFINITY) {
            // successful hit, let's do some light gathering
            Some(rec) => {
                // need a ref since scatter takes a ref to rec later
                let mat = &rec.material;
                // gather any emitted light contribution
                let emit_contrib = match mat.emit(rec.u, rec.v, rec.point) {
                    Some(color) => Vec3A::from(color),
                    None => Vec3A::ZERO,
                };

                // gather any scattered light contribution
                let scatter_contrib = match mat.scatter(self, &rec, rng) {
                    // A successful ray scatter leads to more contributions.
                    Some((scattered, attenuation)) => {
                        attenuation
                            * Vec3A::from(scattered.shade(
                                hittable,
                                bounce_depth - 1,
                                bg_color,
                                rng,
                            ))
                    }
                    // Otherwise, we're done
                    None => Vec3A::ZERO,
                };

                // both emissives and scattered light contribute, unless they're zeroed
                // with current materials, one of these will always be zero
                Color::new(emit_contrib + scatter_contrib)
            }
            // without a hit, functions like a miss shader
            None => bg_color,
        }
    }
}
