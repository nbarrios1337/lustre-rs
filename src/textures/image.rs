use std::path::PathBuf;

use glam::Vec3;

use crate::color::Color;

use super::Texture;

/// An image-based texture
#[derive(Debug)]
pub struct ImageMap {
    image: Option<image::RgbImage>,
}

impl ImageMap {
    /// Creates a new [ImageMap]
    ///
    /// Loads the image located at `file_path`:
    /// * if successful, holds the decoded [image::RgbImage] in an Option
    /// * on error, holds `None`
    pub fn new(file_path: PathBuf) -> Self {
        let img_result = image::open(file_path);

        let img = match img_result {
            Ok(dyn_img) => Some(dyn_img.to_rgb8()),
            Err(e) => {
                eprintln!("Failed to load image: {}", e);
                None
            }
        };

        Self { image: img }
    }
}

impl Texture for ImageMap {
    fn color(&self, u: f32, v: f32, _point: glam::Vec3) -> Color {
        match &self.image {
            None => Color::new(Vec3::new(0.0, 1.0, 1.0)),
            Some(img) => {
                let u = u.clamp(0.0, 1.0);
                let v = 1.0 - v.clamp(0.0, 1.0);

                let i = u * img.width() as f32;
                let j = v * img.height() as f32;

                let i = (i as u32).clamp(0, img.width());
                let j = (j as u32).clamp(0, img.height());

                // let color_scale = 1.0 / 255.0;
                let pixel = img[(i, j)];
                Color::from(pixel)
            }
        }
    }
}
