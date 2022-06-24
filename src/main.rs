use camera::Camera;
use cli::{Arguments, Parser};
use glam::Vec3;
use hittable::HittableList;
use sphere::Sphere;

mod camera;
mod cli;
mod color;
mod hittable;
mod ray;
mod sphere;

fn main() {
    // Parsing cli args
    let cli_args = Arguments::parse();
    let output_file = cli_args.output;

    // Setup camera properties
    let cam = Camera::new();
    let img_w = 400;
    let img_h = (img_w as f32 / Camera::ASPECT_RATIO) as u32;

    // Generate world objects
    let world: HittableList = HittableList(vec![
        Box::new(Sphere {
            center: Vec3::new(0.0, 0.0, -1.0),
            radius: 0.5,
        }),
        Box::new(Sphere {
            center: Vec3::new(0.0, -100.5, -1.0),
            radius: 100.0,
        }),
    ]);

    // Generate image
    let img_buf: image::RgbImage =
        image::ImageBuffer::from_fn(img_w, img_h, |x: u32, y: u32| -> image::Rgb<u8> {
            let u: f64 = x as f64 / (img_w - 1) as f64;
            let v: f64 = (img_h - y) as f64 / (img_h - 1) as f64;

            cam.get_ray(u as f32, v as f32).shade(&world).into()
        });

    // write image to file
    match img_buf.save(output_file.clone()) {
        Ok(()) => println!("Done! Image written to {:?}", output_file),
        Err(why) => {
            eprintln!("Failed to write: {}", why);
        }
    }
}
