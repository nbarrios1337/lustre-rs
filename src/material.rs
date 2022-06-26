use glam::Vec3;

use crate::{hittable::HitRecord, ray::Ray};

/// Enumeration of material types
#[derive(Debug)]
pub enum Material {
    Lambertian { albedo: Vec3 },
    Metal { albedo: Vec3 },
    Dielectric { refract_index: f32 },
}

impl Material {
    /// Returns a scattered ray and its attenuation based on the specific material type.
    /// 
    /// Returns `None` if the material type computes a lack of scattering
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<(Ray, Vec3)> {
        match self {
            Material::Lambertian { albedo } => todo!(),
            Material::Metal { albedo } => todo!(),
            Material::Dielectric { refract_index } => todo!(),
        }
    }
}
