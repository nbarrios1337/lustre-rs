//! Intersectable volumes/participating media

use std::{f32::INFINITY, ops::Neg, sync::Arc};

use rand::Rng;

use crate::{material::Material, textures::Texture};

use super::{HitRecord, Hittable};

/// A Volume with a constant density
pub struct ConstantMedium {
    boundary: Arc<dyn Hittable>,
    material: Arc<Material>,
    density: f32,
}

impl ConstantMedium {
    pub fn new(boundary: &Arc<dyn Hittable>, material: &Arc<dyn Texture>, density: f32) -> Self {
        Self {
            boundary: Arc::clone(boundary),
            material: Arc::new(Material::Isotropic {
                albedo: Arc::clone(material),
            }),
            density,
        }
    }
}

impl Hittable for ConstantMedium {
    fn hit(&self, ray: &crate::ray::Ray, t_min: f32, t_max: f32) -> Option<super::HitRecord> {
        let neg_inv_d = self.density.recip().neg();

        let mut min_rec = match self.boundary.hit(ray, -INFINITY, INFINITY) {
            Some(rec) => rec,
            None => return None,
        };

        let mut max_rec = match self.boundary.hit(ray, min_rec.t + 0.0001, INFINITY) {
            Some(rec) => rec,
            None => return None,
        };

        min_rec.t = min_rec.t.max(t_min);
        max_rec.t = max_rec.t.min(t_max);

        if min_rec.t >= max_rec.t {
            return None;
        }

        min_rec.t = min_rec.t.max(0.0);

        let ray_dir_length = ray.direction.length();
        let dist_inside_boundary = (max_rec.t - min_rec.t) * ray_dir_length;
        let hit_dist = neg_inv_d * (1.0 - rand::thread_rng().gen::<f32>()).log10();
        if hit_dist > dist_inside_boundary {
            return None;
        }

        let t = min_rec.t + hit_dist / ray_dir_length;
        let point = ray.at(t);

        Some(HitRecord {
            point,
            t,
            material: Arc::clone(&self.material),
            // arbitrary values below
            normal: glam::Vec3A::X,
            u: min_rec.u,
            v: min_rec.v,
            front_face: true,
        })
    }

    fn bounding_box(&self, time0: f32, time1: f32) -> Option<crate::bounds::BoundingBox> {
        self.boundary.bounding_box(time0, time1)
    }
}

/// A Volume with density as a function of a provided [noise::NoiseFn]
pub struct NonConstantMedium<N>
where
    N: ::noise::NoiseFn<[f64; 3]> + Send + Sync,
{
    boundary: Arc<dyn Hittable>,
    material: Arc<Material>,
    density_fn: N,
    scale: f32,
}

impl<N> NonConstantMedium<N>
where
    N: ::noise::NoiseFn<[f64; 3]> + Send + Sync,
{
    pub fn new(
        boundary: &Arc<dyn Hittable>,
        material: &Arc<dyn Texture>,
        density_fn: N,
        scale: f32,
    ) -> Self {
        Self {
            boundary: Arc::clone(boundary),
            material: Arc::new(Material::Isotropic {
                albedo: Arc::clone(material),
            }),
            density_fn,
            scale,
        }
    }
}

impl<N> Hittable for NonConstantMedium<N>
where
    N: ::noise::NoiseFn<[f64; 3]> + Send + Sync,
{
    fn hit(&self, ray: &crate::ray::Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut min_rec = match self.boundary.hit(ray, -INFINITY, INFINITY) {
            Some(rec) => rec,
            None => return None,
        };

        let mut max_rec = match self.boundary.hit(ray, min_rec.t + 0.0001, INFINITY) {
            Some(rec) => rec,
            None => return None,
        };

        min_rec.t = min_rec.t.max(t_min);
        max_rec.t = max_rec.t.min(t_max);

        if min_rec.t >= max_rec.t {
            return None;
        }

        min_rec.t = min_rec.t.max(0.0);

        let ray_dir_length = ray.direction.length();
        let dist_inside_boundary = (max_rec.t - min_rec.t) * ray_dir_length;

        // See
        // https://psgraphics.blogspot.com/2009/05/neat-trick-for-ray-collisions-in.html
        let mut hit_dist = 0.0;
        loop {
            let rand_num = rand::thread_rng().gen::<f32>();
            hit_dist += -1.0 * (1.0 - rand_num).log10();
            let second_rand = rand::thread_rng().gen::<f32>();
            let t = min_rec.t + hit_dist / ray_dir_length;
            let point = ray.at(t);
            let noised = self
                .density_fn
                .get((self.scale * point).as_dvec3().to_array());
            if noised as f32 > second_rand {
                break;
            }
        }

        if hit_dist > dist_inside_boundary {
            return None;
        }

        let t = min_rec.t + hit_dist / ray_dir_length;
        let point = ray.at(t);

        Some(HitRecord {
            point,
            t,
            material: Arc::clone(&self.material),
            // arbitrary values below
            normal: glam::Vec3A::X,
            u: min_rec.u,
            v: min_rec.v,
            front_face: true,
        })
    }

    fn bounding_box(&self, time0: f32, time1: f32) -> Option<crate::bounds::BoundingBox> {
        self.boundary.bounding_box(time0, time1)
    }
}
