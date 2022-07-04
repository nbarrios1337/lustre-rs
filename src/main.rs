use glam::Vec3;
use indicatif::{ProgressBar, ProgressStyle};
use scenes::{get_scene, SceneType};

use crate::{
    bvh::BvhNode,
    cli::{Arguments, Parser},
    color::Color,
    rand_util::rand_f32,
};

mod bounds;
mod bvh;
mod camera;
mod cli;
mod color;
mod hittable;
mod material;
mod rand_util;
mod ray;
mod scatter;
mod scenes;
mod sphere;
mod texture;

fn main() {
    // Parsing cli args
    let cli_args = Arguments::parse();
    let output_file = cli_args.output;

    // Set up image properties
    let samples_per_pixel = 100;
    let aspect_ratio = 3.0 / 2.0;
    let img_w = 1200 / 5;
    let img_h = (img_w as f32 / aspect_ratio) as u32;

    // Get scene
    let (cam, world) = get_scene(aspect_ratio, SceneType::CoverPhoto);
    let world = BvhNode::new(world, 0.0, 1.0);

    let progbar = ProgressBar::new((img_h * img_w) as u64)
        .with_style(
            ProgressStyle::default_bar()
                .template("[{elapsed_precise}] {prefix} {wide_bar} {pos:>7}/{len:7} ({percent}%)"),
        )
        .with_prefix("Generating pixels");

    // Generate image
    let depth = 50;
    let img_buf: image::RgbImage =
        image::ImageBuffer::from_fn(img_w, img_h, |x: u32, y: u32| -> image::Rgb<u8> {
            let mut color_v = Vec3::ZERO;
            for _ in 0..samples_per_pixel {
                let u: f64 = (x as f32 + rand_f32()) as f64 / (img_w - 1) as f64;
                let v: f64 = ((img_h - y) as f32 + rand_f32()) as f64 / (img_h - 1) as f64;
                let contrib = cam.get_ray(u as f32, v as f32).shade(&world, depth);
                color_v += Vec3::from(contrib);
            }
            color_v /= samples_per_pixel as f32;
            color_v = color_v.powf(0.5); // sqrt
            progbar.inc(1);
            Color(color_v).into()
        });

    progbar.finish_with_message("Done generating pixels");

    // write image to file
    match img_buf.save(output_file.clone()) {
        Ok(()) => println!("Done! Image written to {:?}", output_file),
        Err(why) => {
            eprintln!("Failed to write: {}", why);
        }
    }
}
