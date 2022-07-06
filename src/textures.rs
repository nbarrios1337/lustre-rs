//! Implementations of various textures
//!
//! Textures can be procedural color generation, image lookup, or a combination of both.

use std::{fmt::Debug, rc::Rc};

use glam::Vec3;
use rand::Rng;

pub use crate::color::Color as SolidColor;
use crate::{color::Color, utils::random::rand_f32};

pub use checkered::*;
pub use perlin::*;

pub mod checkered;
pub mod perlin;

/// Behavior of a texture
pub trait Texture {
    /// Returns the color value at the uv coordinates or point for the texture
    fn color(&self, u: f32, v: f32, point: Vec3) -> Color;
}

impl Debug for dyn Texture {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Texture with Color {:?}",
            self.color(0.0, 0.0, Vec3::ZERO)
        )
    }
}

impl Texture for SolidColor {
    fn color(&self, _u: f32, _v: f32, _point: Vec3) -> Color {
        // Solid Color is the same at all coordinates
        *self
    }
}
