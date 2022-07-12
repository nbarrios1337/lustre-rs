//! Render an image given a [Camera] and a [Hittable].

use glam::Vec3;
use rand::Rng;

use crate::{
    camera::Camera,
    color::Color,
    hittables::Hittable,
    utils::{progress::get_progressbar, random::rand_f32},
};

/// Image Renderer
#[derive(Debug, Clone, Copy)]
pub struct Renderer {
    image_width: u32,
    image_height: u32,
    samples_per_pixel: u32,
}

impl Renderer {
    /// Creates a new [Renderer].
    pub fn new(image_width: u32, image_height: u32, samples_per_pixel: u32) -> Self {
        Self {
            image_width,
            image_height,
            samples_per_pixel,
        }
    }

    /// Generates an image from the given scene.
    ///
    /// A scene consists of a [Camera] and some [Hittable].
    /// This functions outputs its progress to the commandline.
    pub fn render_scene(&self, scene: (Camera, impl Hittable), rng: &mut impl Rng) -> image::RgbImage {
        let progress_bar = get_progressbar((self.image_height * self.image_width) as u64)
            .with_prefix("Generating pixels");

        let (cam, world) = scene;

        // Generate image
        let depth = 50;
        let img_buf: image::RgbImage = image::ImageBuffer::from_fn(
            self.image_width,
            self.image_height,
            |x: u32, y: u32| -> image::Rgb<u8> {
                let mut color_v = Vec3::ZERO;
                for _ in 0..self.samples_per_pixel {
                    let offset_u: f32 = rng.gen();
                    let offset_v: f32 = rng.gen();
                    let u: f64 = (x as f32 + offset_u) as f64 / (self.image_width - 1) as f64;
                    let v: f64 = ((self.image_height - y) as f32 + offset_v) as f64
                        / (self.image_height - 1) as f64;
                    let contrib =
                        cam.get_ray(u as f32, v as f32, rng)
                            .shade(&world, depth, cam.bg_color, rng);
                    color_v += Vec3::from(contrib);
                }
                color_v /= self.samples_per_pixel as f32;
                color_v = color_v.powf(0.5); // sqrt
                progress_bar.inc(1);
                Color::new(color_v).into()
            },
        );

        progress_bar.finish_with_message("Done generating pixels");

        img_buf
    }
}
