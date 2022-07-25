use rand::SeedableRng;
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
    let bounce_depth = cli_args.bounce_depth;

    // Set up image properties
    let samples_per_pixel = cli_args.samples_per_pixel;
    let img_w = 1200;

    // set up enviroment
    let mut rng = if cfg!(debug_assertions) {
        // if debugging, use deterministic seed
        rand::rngs::SmallRng::seed_from_u64(0)
    } else {
        // otherwise real psuedo-randomness
        rand::rngs::SmallRng::from_entropy()
    };

    // Get scene
    let (cam, world, dimensions) = get_scene(img_w, scene, &mut rng);
    let world = BvhNode::new(world, 0.0, 1.0, &mut rng);

    let renderer = Renderer::new(dimensions.x, dimensions.y, samples_per_pixel, bounce_depth);

    let img_buf = renderer.render_scene((cam, world));

    // write image to file
    match img_buf.save(output_file.clone()) {
        Ok(()) => println!("Done! Image written to {:?}", output_file),
        Err(why) => {
            eprintln!("Failed to write: {}", why);
        }
    }
}
