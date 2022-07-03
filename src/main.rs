use std::rc::Rc;

use glam::{const_vec3, Vec3};
use indicatif::{ProgressBar, ProgressStyle};

use crate::{
    camera::Camera,
    cli::{Arguments, Parser},
    color::Color,
    hittable::HittableList,
    material::Material,
    rand_util::{rand_f32, rand_range_f32, rand_vec3},
    sphere::{MovingSphere, Sphere},
};

mod camera;
mod cli;
mod color;
mod hittable;
mod material;
mod rand_util;
mod ray;
mod scatter;
mod sphere;

/// Returns a `HittableList` containing many randomly-generated spheres
fn gen_random_scene() -> HittableList {
    //  Create ground sphere
    let ground_material = Rc::new(Material::Lambertian {
        albedo: Vec3::ONE / 2.0,
    });
    let mut world = HittableList(vec![Box::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        &ground_material,
    ))]);

    // The random generation part
    const DELIMITER: Vec3 = const_vec3!([4.0, 0.2, 0.0]);
    for a in -11..11 {
        for b in -11..11 {
            let center = Vec3::new(
                a as f32 + 0.9 * rand_f32(),
                0.2,
                b as f32 + 0.9 * rand_f32(),
            );

            if (center - DELIMITER).length() > 0.9 {
                let decide_mat = rand_f32();
                // pick a material by "rarity"
                let mat = if (0.0..0.8).contains(&decide_mat) {
                    // diffuse
                    let albedo = rand_vec3() * rand_vec3();
                    Rc::new(Material::Lambertian { albedo })
                } else if (0.0..0.95).contains(&decide_mat) {
                    // metal
                    let albedo = rand_vec3();
                    let roughness = rand_f32();
                    Rc::new(Material::Metal { albedo, roughness })
                } else {
                    // glass
                    Rc::new(Material::Dielectric { refract_index: 1.5 })
                };

                // make the diffuse spheres moveable
                match mat.as_ref() {
                    Material::Lambertian { .. } => {
                        let center2 = center + Vec3::Y * rand_range_f32(0.0, 0.5);
                        let sph = MovingSphere::new(center, center2, 0.0, 1.0, 0.2, &mat);
                        world.push(Box::new(sph));
                    }
                    _ => {
                        let sph = Sphere::new(center, 0.2, &mat);
                        world.push(Box::new(sph));
                    }
                }
            }
        }
    }

    // The signature central spheres
    let mat_1 = Material::Dielectric { refract_index: 1.5 };
    let sphere_1 = Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0, &Rc::new(mat_1));

    let mat_2 = Material::Lambertian {
        albedo: Vec3::new(0.4, 0.2, 0.1),
    };
    let sphere_2 = Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0, &Rc::new(mat_2));

    let mat_3 = Material::Metal {
        albedo: Vec3::new(0.7, 0.6, 0.5),
        roughness: 0.0,
    };
    let sphere_3 = Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0, &Rc::new(mat_3));

    world.push(Box::new(sphere_1));
    world.push(Box::new(sphere_2));
    world.push(Box::new(sphere_3));

    world
}

fn main() {
    // Parsing cli args
    let cli_args = Arguments::parse();
    let output_file = cli_args.output;

    // Set up image properties
    let samples_per_pixel = 100;
    let aspect_ratio = 3.0 / 2.0;
    let img_w = 1200;
    let img_h = (img_w as f32 / aspect_ratio) as u32;

    // Setup camera properties
    let look_form = Vec3::new(13.0, 2.0, 3.0);
    let look_at = Vec3::ZERO;
    let view_up = Vec3::Y;
    let dist_to_focus = 10.0;
    let aperture = 0.1;

    let cam = Camera::new(
        look_form,
        look_at,
        view_up,
        20.0,
        aspect_ratio,
        aperture,
        dist_to_focus,
        0.0,
        1.0,
    );

    let world = gen_random_scene();

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
