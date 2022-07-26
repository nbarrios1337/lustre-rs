//! Scene generation functionality

use std::{path::PathBuf, sync::Arc};

use glam::{UVec2, Vec3A};
use rand::Rng;

use crate::{
    camera::Camera,
    color::Color,
    hittables::{Hittable, HittableList, MovingSphere, Quad, QuadBox, Sphere, Transform},
    material::Material,
    textures::{Checkered, ImageMap, PerlinNoise, SolidColor, Texture},
};

/// Possible hard-coded scenes to choose from.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, clap::clap_derive::ArgEnum)]
pub enum SceneType {
    /// Test scene for materials development
    MaterialDev,
    /// Scene like the cover of "Ray Tracing in One Weekend".
    CoverPhoto,
    /// Two checkered spheres with the camera looking at their point of contact
    TwoSpheres,
    /// Two Perlin noise spheres
    TwoPerlinSpheres,
    /// A single sphere with an image of Earth mapped to it
    Earth,
    /// [SceneType::TwoPerlinSpheres] with a rectangular diffuse light
    SimpleLight,
    /// The famous [Cornell Box scene](https://en.wikipedia.org/wiki/Cornell_box),
    CornellBox,
    /// Cornell Box scene from the [definitive Cornell Box data](https://www.graphics.cornell.edu/online/box/data.html)
    CornellBox2,
    /// The [SceneType::CoverPhoto] in the dark with lights
    RandomLights,
}

/// Returns a [Camera], a list of objects ([HittableList]), and the image dimensions as a tuple.
pub fn get_scene(
    image_width: u32,
    scene_type: SceneType,
    rng: &mut impl Rng,
) -> (Camera, HittableList, UVec2) {
    // Setup default camera properties
    // uncomment the `mut` once its needed
    let mut aspect_ratio = 16.0 / 9.0;
    let mut look_from = Vec3A::new(13.0, 2.0, 3.0);
    let mut look_at = Vec3A::ZERO;
    let /* mut */ view_up = Vec3A::Y;
    let mut vert_fov = 20.0;
    let mut aperture = 0.0;
    let mut focus_dist = 10.0;
    let /* mut */ shutter_time = 0.0..1.0;
    let mut bg_color = Color::new(Vec3A::new(0.7, 0.8, 1.0));

    // Grabs the scene and changes any cam params
    let scene = match scene_type {
        SceneType::MaterialDev => {
            aspect_ratio = 16.0 / 9.0;
            look_from = Vec3A::ZERO;
            look_at = -Vec3A::Z;
            focus_dist = 1.0;
            vert_fov = 90.0;
            get_mat_dev_scene()
        }
        SceneType::CoverPhoto => {
            aperture = 0.1;
            aspect_ratio = 3.0 / 2.0;
            gen_random_scene(rng)
        }

        SceneType::TwoSpheres => gen_two_spheres(),
        SceneType::TwoPerlinSpheres => gen_two_perlin_spheres(),
        SceneType::Earth => gen_earth(),
        SceneType::SimpleLight => {
            bg_color = Color::new(Vec3A::ZERO);
            look_from = Vec3A::new(26.0, 3.0, 6.0);
            look_at = Vec3A::new(0.0, 2.0, 0.0);
            gen_simple_light()
        }
        SceneType::CornellBox => {
            aspect_ratio = 1.0;
            bg_color = Color::new(Vec3A::ZERO);
            look_from = Vec3A::new(278.0, 278.0, -800.0);
            look_at = Vec3A::new(278.0, 278.0, 0.0);
            vert_fov = 40.0;
            gen_cornell_box()
        }
        SceneType::CornellBox2 => {
            aspect_ratio = 1.0;
            bg_color = Color::new(Vec3A::ZERO);
            look_from = Vec3A::new(278.0, 278.0, -800.0);
            look_at = Vec3A::new(278.0, 278.0, 0.0);
            vert_fov = 40.0;
            gen_cornell_box2()
        }
        SceneType::RandomLights => {
            aperture = 0.1;
            aspect_ratio = 3.0 / 2.0;
            bg_color = Color::new(Vec3A::from(bg_color) / 10.0);
            gen_emissive_random(rng)
        }
    };

    // set up camera with (possibly modified) properies
    let cam = Camera::new(
        look_from,
        look_at,
        view_up,
        vert_fov,
        aspect_ratio,
        aperture,
        focus_dist,
        shutter_time,
        bg_color,
    );

    let image_height = (image_width as f32 / aspect_ratio) as u32;
    let dimensions = UVec2::new(image_width, image_height);

    (cam, scene, dimensions)
}

/// Retusn a [HittableList] containing a few spheres with unique materials
fn get_mat_dev_scene() -> HittableList {
    //  Create ground sphere
    let ground_material = Arc::new(Material::Lambertian {
        albedo: Arc::new(Color::new(Vec3A::new(0.8, 0.2, 0.2))),
    });
    let ground_sph = Sphere::new(Vec3A::new(0.0, -1000.5, 0.0), 1000.0, &ground_material);

    let mat_left = Arc::new(Material::Dielectric { refract_index: 1.5 });
    let mat_right = Arc::new(Material::Metal {
        albedo: Arc::new(SolidColor::new(Vec3A::new(0.8, 0.6, 0.2))),
        roughness: 0.1,
    });
    let mat_center = Arc::new(Material::Lambertian {
        albedo: Arc::new(SolidColor::new(Vec3A::new(0.1, 0.2, 0.5))),
    });

    let left_sph = Sphere::new(Vec3A::new(-1.0, 0.0, -1.0), 0.5, &mat_left);
    let right_sph = Sphere::new(Vec3A::new(1.0, 0.0, -1.0), 0.5, &mat_right);
    let center_sph = Sphere::new(Vec3A::new(0.0, 0.0, -1.0), 0.5, &mat_center);

    vec![
        ground_sph.wrap(),
        left_sph.wrap(),
        right_sph.wrap(),
        center_sph.wrap(),
    ]
}

/// Returns a [HittableList] containing randomly-generated spheres
fn gen_random_scene(rng: &mut impl Rng) -> HittableList {
    //  Create ground sphere
    let ground_material = Arc::new(Material::Lambertian {
        albedo: Arc::new(Color::new(Vec3A::ONE / 2.0)),
    });
    let mut world: HittableList =
        vec![Sphere::new(Vec3A::new(0.0, -1000.0, 0.0), 1000.0, &ground_material).wrap()];

    // The random generation part
    const ORIGIN: Vec3A = Vec3A::from_array([4.0, 0.2, 0.0]);
    for a in -11..11 {
        for b in -11..11 {
            let center = Vec3A::new(
                a as f32 + 0.9 * rng.gen::<f32>(),
                0.2,
                b as f32 + 0.9 * rng.gen::<f32>(),
            );

            if (center - ORIGIN).length() > 0.9 {
                let decide_mat = rng.gen();
                // pick a material by "rarity"
                let mat = if (0.0..0.8).contains(&decide_mat) {
                    // diffuse
                    let rand_color_v = rng.gen::<Vec3A>() * rng.gen::<Vec3A>();
                    let albedo = Arc::new(Color::new(rand_color_v));
                    Arc::new(Material::Lambertian { albedo })
                } else if (0.0..0.95).contains(&decide_mat) {
                    // metal
                    let albedo = Arc::new(SolidColor::new(rng.gen()));
                    let roughness = rng.gen();
                    Arc::new(Material::Metal { albedo, roughness })
                } else {
                    // glass
                    Arc::new(Material::Dielectric { refract_index: 1.5 })
                };

                // make the diffuse spheres moveable
                match mat.as_ref() {
                    Material::Lambertian { .. } => {
                        let center2 = center + Vec3A::Y * rng.gen_range(0.0..0.5);
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
    let sphere_1 = Sphere::new(Vec3A::new(0.0, 1.0, 0.0), 1.0, &Arc::new(mat_1));

    let mat_2 = Material::Lambertian {
        albedo: Arc::new(Color::new(Vec3A::new(0.4, 0.2, 0.1))),
    };
    let sphere_2 = Sphere::new(Vec3A::new(-4.0, 1.0, 0.0), 1.0, &Arc::new(mat_2));

    let mat_3 = Material::Metal {
        albedo: Arc::new(SolidColor::new(Vec3A::new(0.7, 0.6, 0.5))),
        roughness: 0.0,
    };
    let sphere_3 = Sphere::new(Vec3A::new(4.0, 1.0, 0.0), 1.0, &Arc::new(mat_3));

    world.push(sphere_1.wrap());
    world.push(sphere_2.wrap());
    world.push(sphere_3.wrap());

    world
}

/// Returns a [HittableList] containing two checkered spheres.
fn gen_two_spheres() -> HittableList {
    let checkered = Arc::new(Material::Lambertian {
        albedo: Arc::new(Checkered::new(
            &(Arc::new(SolidColor::new(Vec3A::new(0.2, 0.3, 0.1))) as Arc<dyn Texture>),
            &(Arc::new(SolidColor::new(Vec3A::new(0.9, 0.9, 0.9))) as Arc<dyn Texture>),
        )),
    });

    vec![
        Sphere::new(Vec3A::new(0.0, -10.0, 0.0), 10.0, &checkered).wrap(),
        Sphere::new(Vec3A::new(0.0, 10.0, 0.0), 10.0, &checkered).wrap(),
    ]
}

/// Returns a [HittableList] containing two Perlin noise spheres.
fn gen_two_perlin_spheres() -> HittableList {
    let perlin_tex = Arc::new(Material::Lambertian {
        albedo: Arc::new(PerlinNoise::new(4.0)),
    });

    vec![
        Sphere::new(Vec3A::new(0.0, -1000.0, 0.0), 1000.0, &perlin_tex).wrap(),
        Sphere::new(Vec3A::new(0.0, 2.0, 0.0), 2.0, &perlin_tex).wrap(),
    ]
}

/// Returns a [HittableList] containing a single image-backed sphere.
fn gen_earth() -> HittableList {
    let earth_tex = Arc::new(Material::Lambertian {
        albedo: Arc::new(ImageMap::new(PathBuf::from("resources/earthmap.jpg"))),
    });

    let globe = Sphere::new(Vec3A::ZERO, 2.0, &earth_tex);
    vec![globe.wrap()]
}

/// Returns a [HittableList] resembling [gen_two_perlin_spheres], with a rectangular diffuse light
fn gen_simple_light() -> HittableList {
    let diff_light = Arc::new(Material::DiffuseLight {
        albedo: Arc::new(SolidColor::new(Vec3A::ONE)),
        brightness: 4.0,
    });

    let mut world = gen_two_perlin_spheres();
    world.push(
        Quad::from_two_points_z(
            Vec3A::new(3.0, 1.0, 0.0),
            Vec3A::new(5.0, 3.0, 0.0),
            -2.0,
            &diff_light,
        )
        .wrap(),
    );

    world
}

fn gen_cornell_box() -> HittableList {
    let red_diffuse = Arc::new(Material::Lambertian {
        albedo: Arc::new(SolidColor::new(Vec3A::new(0.65, 0.05, 0.05))),
    });
    let white_diffuse = Arc::new(Material::Lambertian {
        albedo: Arc::new(SolidColor::new(Vec3A::splat(0.73))),
    });
    let green_diffuse = Arc::new(Material::Lambertian {
        albedo: Arc::new(SolidColor::new(Vec3A::new(0.12, 0.45, 0.15))),
    });
    let light = Arc::new(Material::DiffuseLight {
        albedo: Arc::new(SolidColor::new(Vec3A::ONE)),
        brightness: 15.0,
    });

    // yz rect - zero x
    let left_side = Quad::from_bounds_k(0.0, 555.0, 0.0, 555.0, 555.0, 0, &green_diffuse);

    // yz rect - zero x
    let right_side = Quad::from_bounds_k(0.0, 555.0, 0.0, 555.0, 0.0, 0, &red_diffuse);

    // xz rect - zero y
    let light_rec = Quad::from_bounds_k(213.0, 343.0, 227.0, 332.0, 554.9, 1, &light);

    // xz rect - zero y
    let bottom_side = Quad::from_bounds_k(0.0, 555.0, 0.0, 555.0, 0.0, 1, &white_diffuse);

    // xz rect - zero y
    let top_side = Quad::from_bounds_k(0.0, 555.0, 0.0, 555.0, 555.0, 1, &white_diffuse);

    // xy rect - zero z
    let back_side = Quad::from_bounds_k(0.0, 555.0, 0.0, 555.0, 555.0, 2, &white_diffuse);

    let squarish_min = Vec3A::ZERO;
    let squarish_max = Vec3A::splat(165.0);
    let squarish_box = QuadBox::new(squarish_min, squarish_max, &white_diffuse);
    let squarish_box: Arc<dyn Hittable> = squarish_box.wrap();
    let squarish_box = Transform::new(&squarish_box)
        .with_translation(glam::Vec3::new(130.0, 0.0, 65.0))
        .with_axis_angle(glam::Vec3::Y, -18.0f32.to_radians())
        .finalize();

    let tall_min = Vec3A::ZERO;
    let tall_max = Vec3A::new(165.0, 330.0, 165.0);
    let tall_box = QuadBox::new(tall_min, tall_max, &white_diffuse);
    let tall_box: Arc<dyn Hittable> = tall_box.wrap();
    let tall_box = Transform::new(&tall_box)
        .with_translation(glam::Vec3::new(265.0, 0.0, 295.0))
        .with_axis_angle(glam::Vec3::Y, 15.0f32.to_radians())
        .finalize();

    vec![
        left_side.wrap(),
        right_side.wrap(),
        bottom_side.wrap(),
        top_side.wrap(),
        back_side.wrap(),
        light_rec.wrap(),
        squarish_box.wrap(),
        tall_box.wrap(),
    ]
}

/// Returns the cornell box scene adapted from the original physical measurements
fn gen_cornell_box2() -> HittableList {
    // materials
    let red_diffuse = Arc::new(Material::Lambertian {
        albedo: Arc::new(SolidColor::new(Vec3A::new(0.65, 0.05, 0.05))),
    });
    let white_diffuse = Arc::new(Material::Lambertian {
        albedo: Arc::new(SolidColor::new(Vec3A::splat(0.73))),
    });
    let green_diffuse = Arc::new(Material::Lambertian {
        albedo: Arc::new(SolidColor::new(Vec3A::new(0.12, 0.45, 0.15))),
    });
    let light = Arc::new(Material::DiffuseLight {
        albedo: Arc::new(SolidColor::new(Vec3A::ONE)),
        brightness: 12.0,
    });

    let _mirror_like = Arc::new(Material::Metal {
        albedo: Arc::new(SolidColor::new(Vec3A::splat(0.999))),
        roughness: 0.0,
    });
    let _glass_like = Arc::new(Material::Dielectric { refract_index: 1.5 });

    // walls
    let floor = Quad::new(
        Vec3A::new(552.8, 0.0, 0.0),
        Vec3A::new(0.0, 0.0, 0.0),
        Vec3A::new(0.0, 0.0, 559.2),
        Vec3A::new(549.6, 0.0, 559.2),
        &white_diffuse,
    );

    let light_quad = Quad::new(
        Vec3A::new(343.0, 548.8, 227.0),
        Vec3A::new(343.0, 548.8, 332.0),
        Vec3A::new(213.0, 548.8, 332.0),
        Vec3A::new(213.0, 548.8, 227.0),
        &light,
    );

    let ceiling = Quad::new(
        Vec3A::new(556.0, 548.8, 0.0),
        Vec3A::new(556.0, 548.8, 559.2),
        Vec3A::new(0.0, 548.8, 559.2),
        Vec3A::new(0.0, 548.8, 0.0),
        &white_diffuse,
    );

    let back_wall = Quad::new(
        Vec3A::new(549.6, 0.0, 559.2),
        Vec3A::new(0.0, 0.0, 559.2),
        Vec3A::new(0.0, 548.8, 559.2),
        Vec3A::new(556.0, 548.8, 559.2),
        &white_diffuse,
    );

    let right_wall = Quad::new(
        Vec3A::new(0.0, 0.0, 559.2),
        Vec3A::new(0.0, 0.0, 0.0),
        Vec3A::new(0.0, 548.8, 0.0),
        Vec3A::new(0.0, 548.8, 559.2),
        &green_diffuse,
    );

    let left_wall = Quad::new(
        Vec3A::new(552.8, 0.0, 0.0),
        Vec3A::new(549.6, 0.0, 559.2),
        Vec3A::new(556.0, 548.8, 559.2),
        Vec3A::new(556.0, 548.8, 0.0),
        &red_diffuse,
    );

    // boxes
    let short_box: HittableList = vec![
        Quad::new(
            Vec3A::new(130.0, 165.0, 65.0),
            Vec3A::new(82.0, 165.0, 225.0),
            Vec3A::new(240.0, 165.0, 272.0),
            Vec3A::new(290.0, 165.0, 114.0),
            &white_diffuse,
        )
        .wrap(),
        Quad::new(
            Vec3A::new(290.0, 0.0, 114.0),
            Vec3A::new(290.0, 165.0, 114.0),
            Vec3A::new(240.0, 165.0, 272.0),
            Vec3A::new(240.0, 0.0, 272.0),
            &white_diffuse,
        )
        .wrap(),
        Quad::new(
            Vec3A::new(130.0, 0.0, 65.0),
            Vec3A::new(130.0, 165.0, 65.0),
            Vec3A::new(290.0, 165.0, 114.0),
            Vec3A::new(290.0, 0.0, 114.0),
            &white_diffuse,
        )
        .wrap(),
        Quad::new(
            Vec3A::new(82.0, 0.0, 225.0),
            Vec3A::new(82.0, 165.0, 225.0),
            Vec3A::new(130.0, 165.0, 65.0),
            Vec3A::new(130.0, 0.0, 65.0),
            &white_diffuse,
        )
        .wrap(),
        Quad::new(
            Vec3A::new(240.0, 0.0, 272.0),
            Vec3A::new(240.0, 165.0, 272.0),
            Vec3A::new(82.0, 165.0, 225.0),
            Vec3A::new(82.0, 0.0, 225.0),
            &white_diffuse,
        )
        .wrap(),
    ];

    let tall_box: HittableList = vec![
        Quad::new(
            Vec3A::new(423.0, 330.0, 247.0),
            Vec3A::new(265.0, 330.0, 296.0),
            Vec3A::new(314.0, 330.0, 456.0),
            Vec3A::new(472.0, 330.0, 406.0),
            &white_diffuse,
        )
        .wrap(),
        Quad::new(
            Vec3A::new(423.0, 0.0, 247.0),
            Vec3A::new(423.0, 330.0, 247.0),
            Vec3A::new(472.0, 330.0, 406.0),
            Vec3A::new(472.0, 0.0, 406.0),
            &white_diffuse,
        )
        .wrap(),
        Quad::new(
            Vec3A::new(472.0, 0.0, 406.0),
            Vec3A::new(472.0, 330.0, 406.0),
            Vec3A::new(314.0, 330.0, 456.0),
            Vec3A::new(314.0, 0.0, 456.0),
            &white_diffuse,
        )
        .wrap(),
        Quad::new(
            Vec3A::new(314.0, 0.0, 456.0),
            Vec3A::new(314.0, 330.0, 456.0),
            Vec3A::new(265.0, 330.0, 296.0),
            Vec3A::new(265.0, 0.0, 296.0),
            &white_diffuse,
        )
        .wrap(),
        Quad::new(
            Vec3A::new(265.0, 0.0, 296.0),
            Vec3A::new(265.0, 330.0, 296.0),
            Vec3A::new(423.0, 330.0, 247.0),
            Vec3A::new(423.0, 0.0, 247.0),
            &white_diffuse,
        )
        .wrap(),
    ];
    vec![
        floor.wrap(),
        light_quad.wrap(),
        ceiling.wrap(),
        back_wall.wrap(),
        right_wall.wrap(),
        left_wall.wrap(),
        short_box.wrap(),
        tall_box.wrap(),
    ]
}

/// Returns a [HittableList] containing randomly-generated spheres, some emissive
fn gen_emissive_random(rng: &mut impl Rng) -> HittableList {
    // the set of objects with estimated capacity
    let mut world: HittableList = Vec::with_capacity(4 + (-11..11).len().pow(2));

    //  Create ground sphere
    let ground_material = Arc::new(Material::Lambertian {
        albedo: Arc::new(Color::new(Vec3A::ONE / 2.0)),
    });

    let ground = Sphere::new(Vec3A::new(0.0, -1000.0, 0.0), 1000.0, &ground_material);
    world.push(ground.wrap());

    // The random generation part
    const ORIGIN: Vec3A = Vec3A::from_array([4.0, 0.2, 0.0]);
    for a in -11..11 {
        for b in -11..11 {
            let center = Vec3A::new(
                a as f32 + 0.9 * rng.gen::<f32>(),
                0.2,
                b as f32 + 0.9 * rng.gen::<f32>(),
            );

            if (center - ORIGIN).length() > 0.9 {
                let decide_mat = rng.gen();
                // pick a material by "rarity"
                let mat = if (0.0..0.75).contains(&decide_mat) {
                    // diffuse
                    let rand_color_v = rng.gen::<Vec3A>() * rng.gen::<Vec3A>();
                    let albedo = Arc::new(Color::new(rand_color_v));
                    Arc::new(Material::Lambertian { albedo })
                } else if (0.0..0.85).contains(&decide_mat) {
                    // metal
                    let albedo = Arc::new(SolidColor::new(rng.gen()));
                    let roughness = rng.gen();
                    Arc::new(Material::Metal { albedo, roughness })
                } else if (0.0..0.90).contains(&decide_mat) {
                    // emissive
                    let albedo = Arc::new(SolidColor::new(rng.gen()));
                    let brightness = rng.gen_range(2.0..10.0);
                    Arc::new(Material::DiffuseLight { albedo, brightness })
                } else {
                    // glass
                    Arc::new(Material::Dielectric { refract_index: 1.5 })
                };

                let sph = Sphere::new(center, 0.2, &mat);
                world.push(sph.wrap())
            }
        }
    }

    // The signature central spheres
    let mat_1 = Material::Dielectric { refract_index: 1.5 };
    let sphere_1 = Sphere::new(Vec3A::new(0.0, 1.0, 0.0), 1.0, &Arc::new(mat_1));

    let mat_2 = Material::Lambertian {
        albedo: Arc::new(Color::new(Vec3A::new(0.4, 0.2, 0.1))),
    };
    let sphere_2 = Sphere::new(Vec3A::new(-4.0, 1.0, 0.0), 1.0, &Arc::new(mat_2));

    let mat_3 = Material::Metal {
        albedo: Arc::new(SolidColor::new(Vec3A::new(0.7, 0.6, 0.5))),
        roughness: 0.0,
    };
    let sphere_3 = Sphere::new(Vec3A::new(4.0, 1.0, 0.0), 1.0, &Arc::new(mat_3));

    world.push(sphere_1.wrap());
    world.push(sphere_2.wrap());
    world.push(sphere_3.wrap());

    world
}
