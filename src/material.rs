use std::f32::EPSILON;

use glam::Vec3;

use crate::{
    hittable::HitRecord,
    linalg::{reflect, refract},
    rand_util::rand_unit_vec3,
    ray::Ray,
};

/// Enumeration of material types
#[derive(Debug)]
pub enum Material {
    Lambertian { albedo: Vec3 },
    Metal { albedo: Vec3, fuzz: f32 },
    Dielectric { refract_index: f32 },
}

impl Material {
    /// Returns a scattered ray and its attenuation based on the specific material type.
    ///
    /// Returns `None` if the material type computes a lack of scattering
    pub fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<(Ray, Vec3)> {
        match self {
            Material::Lambertian { albedo } => {
                let mut scatter_dir = rec.normal + rand_unit_vec3();

                // If the scatter direction is close to zero in all dimensions
                if scatter_dir.cmplt(Vec3::splat(EPSILON)).all() {
                    scatter_dir = rec.normal;
                }

                Some((Ray::new(rec.point, scatter_dir), *albedo))
            }
            Material::Metal { albedo, fuzz } => {
                let reflected = reflect(&ray.direction.normalize(), &rec.normal);

                let scattered = Ray::new(
                    rec.point,
                    reflected + fuzz.clamp(0.0, 1.0) * rand_unit_vec3(),
                );

                if scattered.direction.dot(rec.normal) > 0.0 {
                    Some((scattered, *albedo))
                } else {
                    None
                }
            }
            Material::Dielectric { refract_index } => {
                let attenuation = Vec3::ONE;
                let refract_ratio = if rec.front_face {
                    1.0 / refract_index
                } else {
                    *refract_index
                };

                let unit_dir = ray.direction.normalize();
                let cos_theta = (-unit_dir).dot(rec.normal).min(1.0);
                let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

                let direction = if refract_ratio * sin_theta > 1.0 {
                    reflect(&unit_dir, &rec.normal)
                } else {
                    refract(&unit_dir, &rec.normal, refract_ratio)
                };

                Some((Ray::new(rec.point, direction), attenuation))
            }
        }
    }
}
