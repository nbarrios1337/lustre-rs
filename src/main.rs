use cli::{Arguments, Parser};
use glam::Vec3;
use utils::clamp;

mod camera;
mod cli;
mod utils;
mod ray;

const IMAGE_W: u32 = 256;
const IMAGE_H: u32 = 256;

fn coord_to_pixel(x: u32, y: u32) -> image::Rgb<u8> {
    let r: f64 = x as f64 / (IMAGE_W - 1) as f64;
    let g: f64 = (IMAGE_H - y) as f64 / (IMAGE_H - 1) as f64;
    let b = 0.25;

    let ir = (256.0 * clamp(r, 0.0, 1.0)) as u8;
    let ig = (256.0 * clamp(g, 0.0, 1.0)) as u8;
    let ib = (256.0 * clamp(b, 0.0, 1.0)) as u8;

    image::Rgb([ir, ig, ib])
}

fn main() {
    // Parsing cli args
    let cli_args = Arguments::parse();
    let output_file = cli_args.output;

    // write image to file
    let img_buf: image::RgbImage = image::ImageBuffer::from_fn(IMAGE_W, IMAGE_H, coord_to_pixel);
    if let Err(why) = img_buf.save(output_file) {
        eprintln!("Failed to write: {}", why);
    }

    let v = Vec3::new(0.0, 0.0, 0.0);
    println!("{}", v)
}
