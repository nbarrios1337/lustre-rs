//! Implementations of various textures
//!
//! Textures can be procedural color generation, image lookup, or a combination of both.

use std::fmt::Debug;

use glam::Vec3A;

pub use crate::color::Color as SolidColor;
use crate::color::Color;

pub use self::image::*;
pub use self::noise::*;
pub use checkered::*;
pub use perlin::*;

pub mod checkered;
pub mod image;
pub mod noise;
pub mod perlin;

/// Behavior of a texture
pub trait Texture: Send + Sync {
    /// Returns the color value at the uv coordinates or point for the texture
    fn color(&self, u: f32, v: f32, point: Vec3A) -> Color;
}

impl Debug for dyn Texture {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Texture with Color {:?}",
            self.color(0.0, 0.0, Vec3A::ZERO)
        )
    }
}

impl Texture for SolidColor {
    fn color(&self, _u: f32, _v: f32, _point: Vec3A) -> Color {
        // Solid Color is the same at all coordinates
        *self
    }
}
