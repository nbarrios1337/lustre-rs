//! Render an image given a [Camera] and a [Hittable].

use glam::Vec3A;
use indicatif::ParallelProgressIterator;
use rand::{rngs::SmallRng, Rng, SeedableRng};
use rayon::prelude::*;

use crate::{camera::Camera, color::Color, hittables::Hittable, utils::progress::get_progressbar};

/// Image Renderer storing scene context values such as image dimensions and samples per pixel
#[derive(Debug, Clone, Copy)]
pub struct Renderer {
    image_width: u32,
    image_height: u32,
    samples_per_pixel: u32,
    bounce_depth: u16,
}

impl Renderer {
    /// Creates a new [Renderer].
    pub fn new(
        image_width: u32,
        image_height: u32,
        samples_per_pixel: u32,
        bounce_depth: u16,
    ) -> Self {
        Self {
            image_width,
            image_height,
            samples_per_pixel,
            bounce_depth,
        }
    }

    /// Calculates the total color value of the pixel at image coordinates (`x`, `y`)
    ///
    /// Uses the provided [Camera] to translate the image coordinates
    /// to world space coordinates, then computes the color value
    #[inline]
    fn compute_pixel_v(
        &self,
        cam: &Camera,
        world: &impl Hittable,
        x: u32,
        y: u32,
        rng: &mut impl Rng,
    ) -> Vec3A {
        // convert buffer indices to viewport coordinates
        let offset_u: f32 = rng.gen();
        let offset_v: f32 = rng.gen();
        let u: f64 = (x as f32 + offset_u) as f64 / (self.image_width - 1) as f64;
        let v: f64 =
            ((self.image_height - y) as f32 + offset_v) as f64 / (self.image_height - 1) as f64;

        // trace ray
        let contrib =
            cam.get_ray(u as f32, v as f32, rng)
                .shade(world, self.bounce_depth, cam.bg_color, rng);
        Vec3A::from(contrib)
    }

    /// Generates an image from the given scene.
    ///
    /// A scene consists of a [Camera] and some [Hittable].
    /// This functions outputs its progress to the commandline.
    pub fn render_scene(&self, scene: (Camera, impl Hittable)) -> image::RgbImage {
        let progress_bar = get_progressbar((self.image_height * self.image_width) as u64)
            .with_prefix("Generating pixels");

        // Set up rendering properties
        let (cam, world) = scene;

        // Allocate image buffer
        let mut img_buf: image::RgbImage =
            image::ImageBuffer::new(self.image_width, self.image_height);

        // Generate image
        img_buf
            .enumerate_pixels_mut()
            .par_bridge()
            .progress_with(progress_bar)
            .for_each(|indexed_pixel| {
                // unpack the enumeration
                let (x, y, pixel) = indexed_pixel;

                // map reduce N samples into single Vec3A
                let mut color_v = (0..self.samples_per_pixel)
                    .into_par_iter()
                    .map_init(
                        || SmallRng::from_rng(rand::thread_rng()),
                        // current sample # doesn't matter, ignore
                        |rng, _| {
                            // from_rng(...) gives Result, unpack here
                            let rng = rng.as_mut().unwrap();
                            self.compute_pixel_v(&cam, &world, x, y, rng)
                        },
                    )
                    .reduce(|| Vec3A::ZERO, |a, b| a + b);

                // Account for number of samples
                color_v /= self.samples_per_pixel as f32;

                // "gamma" correction
                color_v = color_v.powf(0.5); // sqrt

                // modify pixel with generated color value
                *pixel = image::Rgb::<u8>::from(Color::new(color_v));
            });

        img_buf
    }
}
