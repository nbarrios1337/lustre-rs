use render::Renderer;
use scenes::{get_scene, SceneType};

use crate::{
    bvh::BvhNode,
    cli::{Arguments, Parser},
};

mod bounds;
mod bvh;
mod camera;
mod cli;
mod color;
mod hittables;
mod material;
mod ray;
mod render;
mod scatter;
mod scenes;
mod sphere;
mod textures;
mod utils;

fn main() {
    // Parsing cli args
    let cli_args = Arguments::parse();
    let output_file = cli_args.output;

    // Set up image properties
    let samples_per_pixel = 100;
    let aspect_ratio = 3.0 / 2.0;
    let img_w = 1200;
    let img_h = (img_w as f32 / aspect_ratio) as u32;

    // Get scene
    let (cam, world) = get_scene(aspect_ratio, SceneType::CoverPhoto);
    let world = BvhNode::new(world, 0.0, 1.0);

    let renderer = Renderer::new(img_h, img_w, samples_per_pixel);

    let img_buf = renderer.render_scene((cam, world));

    // write image to file
    match img_buf.save(output_file.clone()) {
        Ok(()) => println!("Done! Image written to {:?}", output_file),
        Err(why) => {
            eprintln!("Failed to write: {}", why);
        }
    }
}
