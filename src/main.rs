use render::Renderer;
use scenes::get_scene;

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
mod textures;
mod utils;

fn main() {
    // Parsing cli args
    let cli_args = Arguments::parse();
    let output_file = cli_args.output;
    let scene = cli_args.scene;

    // Set up image properties
    let samples_per_pixel = cli_args.samples_per_pixel;
    // set up enviroment
    let mut rng = rand::thread_rng();

    // Get scene
    let (cam, world, dimensions) = get_scene(img_w, scene, &mut rng);
    let world = BvhNode::new(world, 0.0, 1.0, &mut rng);

    let renderer = Renderer::new(dimensions.x, dimensions.y, samples_per_pixel);

    let img_buf = renderer.render_scene((cam, world), &mut rng);

    // write image to file
    match img_buf.save(output_file.clone()) {
        Ok(()) => println!("Done! Image written to {:?}", output_file),
        Err(why) => {
            eprintln!("Failed to write: {}", why);
        }
    }
}
