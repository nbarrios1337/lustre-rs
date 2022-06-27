use std::rc::Rc;

use camera::Camera;
use cli::{Arguments, Parser};
use color::Color;
use glam::Vec3;
use hittable::HittableList;
use indicatif::{ProgressBar, ProgressStyle};
use rand_util::rand_f32;
use sphere::Sphere;

mod camera;
mod cli;
mod color;
mod hittable;
mod linalg;
mod material;
mod rand_util;
mod ray;
mod sphere;

fn main() {
    // Parsing cli args
    let cli_args = Arguments::parse();
    let output_file = cli_args.output;

    // Setup camera properties
    let samples_per_pixel = 100;
    let cam = Camera::new(samples_per_pixel);
    let img_w = 400;
    let img_h = (img_w as f32 / Camera::ASPECT_RATIO) as u32;

    let depth = 50;

    // set up materials
    let material_ground = Rc::new(material::Material::Lambertian {
        albedo: Vec3::new(0.7, 0.3, 0.3),
    });
    let material_center = Rc::new(material::Material::Lambertian {
        albedo: Vec3::new(0.8, 0.8, 0.0),
    });
    // Generate world objects
    let world: HittableList = HittableList(vec![
        Box::new(Sphere {
            center: Vec3::new(0.0, 0.0, -1.0),
            radius: 0.5,
            material: material_ground,
        }),
        Box::new(Sphere {
            center: Vec3::new(0.0, -100.5, -1.0),
            radius: 100.0,
            material: material_center,
        }),
    ]);

    let progbar = ProgressBar::new((img_h * img_w) as u64)
        .with_style(
            ProgressStyle::default_bar()
                .template("[{elapsed_precise}] {prefix} {wide_bar} {pos:>7}/{len:7} ({percent}%)"),
        )
        .with_prefix("Generating pixels");

    // Generate image
    let img_buf: image::RgbImage =
        image::ImageBuffer::from_fn(img_w, img_h, |x: u32, y: u32| -> image::Rgb<u8> {
            let mut color_v = Vec3::ZERO;
            for _ in 0..cam.spp {
                let u: f64 = (x as f32 + rand_f32()) as f64 / (img_w - 1) as f64;
                let v: f64 = ((img_h - y) as f32 + rand_f32()) as f64 / (img_h - 1) as f64;
                let contrib = cam.get_ray(u as f32, v as f32).shade(&world, depth);
                color_v += Vec3::from(contrib);
            }
            color_v /= cam.spp as f32;
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
