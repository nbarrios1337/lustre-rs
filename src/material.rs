//! Implementation of material types

use std::{f32::EPSILON, sync::Arc};

use glam::Vec3A;
use rand::Rng;

use crate::{
    color::Color,
    hittables::HitRecord,
    ray::Ray,
    scatter::{reflect, refract},
    textures::Texture,
};

/// Enumeration of possible material types.
#[derive(Debug)]
pub enum Material {
    /// An approximation of a diffuse, or matte, material.
    ///
    /// See the [Wikipedia page on Lambertian reflectance](https://en.wikipedia.org/wiki/Lambertian_reflectance) for more information.
    Lambertian { albedo: Arc<dyn Texture> },
    /// A metallic material that reflects rays based on the given roughness.
    Metal {
        albedo: Arc<dyn Texture>,
        roughness: f32,
    },
    /// A glass material that scatters rays based on the given refractive index.
    Dielectric { refract_index: f32 },
    /// A material emitting diffuse light
    DiffuseLight {
        albedo: Arc<dyn Texture>,
        brightness: f32,
    },
}

impl Material {
    /// Computes reflectance using Schlick's approximation
    fn reflectance(cosine: f32, refract_idx: f32) -> f32 {
        let r0 = (1.0 - refract_idx) / (1.0 + refract_idx);
        let r0_doubled = r0 * r0;
        r0_doubled + (1.0 - r0_doubled) * (1.0 - cosine).powi(5)
    }

    /// Returns a scattered ray and its attenuation based on the specific material type.
    ///
    /// Returns `None` if the material type computes a lack of scattering
    pub fn scatter(&self, ray: &Ray, rec: &HitRecord, rng: &mut impl Rng) -> Option<(Ray, Vec3A)> {
        // common calcs
        let normed_dir = ray.direction.normalize();
        let rand_unit_v = crate::utils::random::rand_vec3_in_unit_sphere(rng);
        match self {
            Material::Lambertian { albedo } => {
                let mut scatter_dir = rec.normal + rand_unit_v;

                // If the scatter direction is close to zero in all dimensions
                if scatter_dir.cmplt(Vec3A::splat(EPSILON)).all() {
                    scatter_dir = rec.normal;
                }

                Some((
                    Ray::new(rec.point, scatter_dir, ray.time),
                    albedo.color(rec.u, rec.v, rec.point).into(),
                ))
            }
            Material::Metal { albedo, roughness } => {
                let reflected = reflect(normed_dir, rec.normal);

                let scattered = Ray::new(
                    rec.point,
                    reflected + roughness.clamp(0.0, 1.0) * rand_unit_v,
                    ray.time,
                );

                if scattered.direction.dot(rec.normal) > 0.0 {
                    Some((scattered, albedo.color(rec.u, rec.v, rec.point).into()))
                } else {
                    None
                }
            }
            Material::Dielectric { refract_index } => {
                let attenuation = Vec3A::ONE;
                let refract_ratio = if rec.front_face {
                    1.0 / refract_index
                } else {
                    *refract_index
                };

                let cos_theta = (-normed_dir).dot(rec.normal).min(1.0);
                let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

                let no_refract = refract_ratio * sin_theta > 1.0;
                let reflect_chance = Self::reflectance(cos_theta, refract_ratio);
                let do_reflect = reflect_chance > rng.gen();
                let direction = if no_refract || do_reflect {
                    // must reflect
                    reflect(normed_dir, rec.normal)
                } else {
                    // can refract
                    refract(normed_dir, rec.normal, refract_ratio)
                };

                Some((Ray::new(rec.point, direction, ray.time), attenuation))
            }
            Material::DiffuseLight { .. } => None,
        }
    }

    /// Returns the emmited color of light from the material, if any.
    pub fn emit(&self, u: f32, v: f32, point: Vec3A) -> Option<Color> {
        match self {
            Material::DiffuseLight { albedo, brightness } => {
                let color = albedo.color(u, v, point);
                let val = *brightness * Vec3A::from(color);
                Some(Color::new(val))
            }
            // Make emission explicit; nothing emits unless specifically implemented.
            _ => None,
        }
    }
}
