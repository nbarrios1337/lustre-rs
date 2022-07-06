//! Implementations of various textures
//!
//! Textures can be procedural color generation, image lookup, or a combination of both.

use std::{fmt::Debug, rc::Rc};

use glam::Vec3;
use rand::Rng;

pub use crate::color::Color as SolidColor;
use crate::{color::Color, rand_util::rand_f32};

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

/// A checkered texture alternating between two enclosed textures.
pub struct Checkered {
    pub even: Rc<dyn Texture>,
    pub odd: Rc<dyn Texture>,
}

impl Checkered {
    /// Creates a new checkered texture
    pub fn new(o: &Rc<dyn Texture>, e: &Rc<dyn Texture>) -> Self {
        Self {
            even: e.clone(),
            odd: o.clone(),
        }
    }
}

impl Texture for Checkered {
    fn color(&self, u: f32, v: f32, point: Vec3) -> Color {
        let sin_x = (point * 10.0).x.sin();
        let sin_y = (point * 10.0).y.sin();
        let sin_z = (point * 10.0).z.sin();

        if sin_x * sin_y * sin_z < 0.0 {
            self.odd.color(u, v, point)
        } else {
            self.even.color(u, v, point)
        }
    }
}

#[derive(Debug)]
pub struct PerlinNoise {
    random_values: [f32; Self::POINT_COUNT],
    perm_x: [i32; Self::POINT_COUNT],
    perm_y: [i32; Self::POINT_COUNT],
    perm_z: [i32; Self::POINT_COUNT],
}

impl PerlinNoise {
    const POINT_COUNT: usize = 256;

    pub fn noise(&self, point: Vec3) -> f32 {
        // let ijk = (4 * point.as_uvec3()) & UVec3::splat((Self::POINT_COUNT - 1) as u32);
        let i = (4.0 * point.x) as isize & (Self::POINT_COUNT - 1) as isize;
        let j = (4.0 * point.y) as isize & (Self::POINT_COUNT - 1) as isize;
        let k = (4.0 * point.z) as isize & (Self::POINT_COUNT - 1) as isize;

        let perm_x_at_i = self.perm_x[i as usize];
        let perm_y_at_j = self.perm_y[j as usize];
        let perm_z_at_k = self.perm_z[k as usize];

        let index = perm_x_at_i ^ perm_y_at_j ^ perm_z_at_k;
        assert!(self.random_values[index as usize] < 1.0);
        self.random_values[index as usize]
    }

    fn gen_perm() -> [i32; Self::POINT_COUNT] {
        // Generate successive integers from 0 to 256
        let mut perm: [i32; Self::POINT_COUNT] = (0..(Self::POINT_COUNT as i32))
            .collect::<Vec<i32>>()
            .try_into()
            .unwrap();

        // Swap values in perm through an increasingly small window of possible swap targets
        for i in (0..Self::POINT_COUNT).rev() {
            let target = rand::thread_rng().gen_range(0..=i);
            perm.swap(i, target);
        }

        assert!(
            perm.iter()
                .all(|p| (0..Self::POINT_COUNT as i32).contains(p)),
            "result contains value >= 256"
        );

        perm
    }

    pub fn new() -> Self {
        Self {
            random_values: [0.0; Self::POINT_COUNT].map(|_| rand_f32()),
            perm_x: Self::gen_perm(),
            perm_y: Self::gen_perm(),
            perm_z: Self::gen_perm(),
        }
    }
}

impl Texture for PerlinNoise {
    fn color(&self, _u: f32, _v: f32, point: Vec3) -> Color {
        Color(Vec3::ONE * self.noise(point))
    }
}
