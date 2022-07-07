//! Scene generation functionality

use std::{path::PathBuf, rc::Rc, str::FromStr};

use glam::{const_vec3, Vec3};

use crate::{
    camera::Camera,
    color::Color,
    hittable::{Hittable, HittableList},
    material::Material,
    sphere::{MovingSphere, Sphere},
    textures::{Checkered, ImageMap, PerlinNoise, SolidColor, Texture},
    utils::random::*,
};

/// Possible hard-coded scenes to choose from.
#[allow(dead_code)]
pub enum SceneType {
    /// Scene like the cover of "Ray Tracing in One Weekend".
    CoverPhoto,
    /// Two checkered spheres with the camera looking at their point of contact
    TwoSpheres,
    /// Two Perlin noise spheres
    TwoPerlinSpheres,
    /// A single sphere with an image of Earth mapped to it
    Earth,
}

/// Returns a [Camera] along with a corresponding list of objects ([HittableList]).
pub fn get_scene(aspect_ratio: f32, scene_type: SceneType) -> (Camera, HittableList) {
    // Setup camera properties
    let look_form = Vec3::new(13.0, 2.0, 3.0);
    let look_at = Vec3::ZERO;
    let view_up = Vec3::Y;
    let vert_fov = 20.0;
    let aperture = 0.1;
    let focus_dist = 10.0;
    let shutter_open = 0.0;
    let shutter_close = 1.0;
    let bg_color = Color::new(Vec3::new(0.7,0.8,1.0));

    match scene_type {
        SceneType::CoverPhoto => {
            let cam = Camera::new(
                look_form,
                look_at,
                view_up,
                vert_fov,
                aspect_ratio,
                aperture,
                focus_dist,
                shutter_open,
                shutter_close,
                bg_color,
            );
            (cam, gen_random_scene())
        }
        SceneType::TwoSpheres => {
            let aperture = 0.0;
            let cam = Camera::new(
                look_form,
                look_at,
                view_up,
                vert_fov,
                aspect_ratio,
                aperture,
                focus_dist,
                shutter_open,
                shutter_close,
                bg_color,
            );
            (cam, gen_two_spheres())
        }
        SceneType::TwoPerlinSpheres => {
            let cam = Camera::new(
                look_form,
                look_at,
                view_up,
                vert_fov,
                aspect_ratio,
                aperture,
                focus_dist,
                shutter_open,
                shutter_close,
                bg_color,
            );
            (cam, gen_two_perlin_spheres())
        }
        SceneType::Earth => {
            let cam = Camera::new(
                look_form,
                look_at,
                view_up,
                vert_fov,
                aspect_ratio,
                aperture,
                focus_dist,
                shutter_open,
                shutter_close,
                bg_color,
            );
            (cam, gen_earth())
        }
    }
}

/// Returns a [HittableList] containing randomly-generated spheres
fn gen_random_scene() -> HittableList {
    //  Create ground sphere
    let ground_material = Rc::new(Material::Lambertian {
        albedo: Rc::new(Color::new(Vec3::ONE / 2.0)),
    });
    let mut world: HittableList =
        vec![Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0, &ground_material).wrap()];

    // The random generation part
    const ORIGIN: Vec3 = const_vec3!([4.0, 0.2, 0.0]);
    for a in -11..11 {
        for b in -11..11 {
            let center = Vec3::new(
                a as f32 + 0.9 * rand_f32(),
                0.2,
                b as f32 + 0.9 * rand_f32(),
            );

            if (center - ORIGIN).length() > 0.9 {
                let decide_mat = rand_f32();
                // pick a material by "rarity"
                let mat = if (0.0..0.8).contains(&decide_mat) {
                    // diffuse
                    let albedo = Rc::new(Color::new(rand_vec3() * rand_vec3()));
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
                        world.push(sph.wrap())
                    }
                    _ => {
                        let sph = Sphere::new(center, 0.2, &mat);
                        world.push(sph.wrap())
                    }
                }
            }
        }
    }

    // The signature central spheres
    let mat_1 = Material::Dielectric { refract_index: 1.5 };
    let sphere_1 = Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0, &Rc::new(mat_1));

    let mat_2 = Material::Lambertian {
        albedo: Rc::new(Color::new(Vec3::new(0.4, 0.2, 0.1))),
    };
    let sphere_2 = Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0, &Rc::new(mat_2));

    let mat_3 = Material::Metal {
        albedo: Vec3::new(0.7, 0.6, 0.5),
        roughness: 0.0,
    };
    let sphere_3 = Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0, &Rc::new(mat_3));

    world.push(sphere_1.wrap());
    world.push(sphere_2.wrap());
    world.push(sphere_3.wrap());

    world
}

/// Returns a [HittableList] containing two checkered spheres.
fn gen_two_spheres() -> HittableList {
    let checkered = Rc::new(Material::Lambertian {
        albedo: Rc::new(Checkered::new(
            &(Rc::new(SolidColor::new(Vec3::new(0.2, 0.3, 0.1))) as Rc<dyn Texture>),
            &(Rc::new(SolidColor::new(Vec3::new(0.9, 0.9, 0.9))) as Rc<dyn Texture>),
        )),
    });

    vec![
        Sphere::new(Vec3::new(0.0, -10.0, 0.0), 10.0, &checkered).wrap(),
        Sphere::new(Vec3::new(0.0, 10.0, 0.0), 10.0, &checkered).wrap(),
    ]
}

/// Returns a [HittableList] containing two Perlin noise spheres.
fn gen_two_perlin_spheres() -> HittableList {
    let perlin_tex = Rc::new(Material::Lambertian {
        albedo: Rc::new(PerlinNoise::new(4.0)),
    });

    vec![
        Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0, &perlin_tex).wrap(),
        Sphere::new(Vec3::new(0.0, 2.0, 0.0), 2.0, &perlin_tex).wrap(),
    ]
}

fn gen_earth() -> HittableList {
    let earth_tex = Rc::new(Material::Lambertian {
        albedo: Rc::new(ImageMap::new(
            PathBuf::from_str("resources/earthmap.jpg").unwrap(),
        )),
    });

    let globe = Sphere::new(Vec3::ZERO, 2.0, &earth_tex);
    vec![globe.wrap()]
}
