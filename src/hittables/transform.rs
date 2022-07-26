//! Hittable Instance that tranforms the ontained hittable

use std::sync::Arc;

use glam::{Affine3A, Vec3};

use super::Hittable;

/// A hittable undergoes a transform before and after being hit.
pub struct Transform {
    matrix: Affine3A,
    object: Arc<dyn Hittable>,
}

impl Transform {
    // creators

    /// Creates an affine transform that does not transform the underlying object
    pub fn new(o: &Arc<dyn Hittable>) -> Self {
        Self {
            matrix: Affine3A::IDENTITY,
            object: Arc::clone(o),
        }
    }

    /// Creates an affine transform that changes the size of the object.
    pub fn from_scale_factor(scale: Vec3, o: &Arc<dyn Hittable>) -> Self {
        Self {
            matrix: Affine3A::from_scale(scale),
            object: Arc::clone(o),
        }
    }

    /// Creates an affine transform containing a 3D rotation around an `axis`, of `angle` (in radians).
    pub fn from_axis_angle(axis: Vec3, angle: f32, o: &Arc<dyn Hittable>) -> Self {
        Self {
            matrix: Affine3A::from_axis_angle(axis, angle),
            object: Arc::clone(o),
        }
    }

    /// Creates an affine transform from the given 3D `translation`.
    pub fn from_translation(translation: Vec3, o: &Arc<dyn Hittable>) -> Self {
        Self {
            matrix: Affine3A::from_translation(translation),
            object: Arc::clone(o),
        }
    }

    /// Creates a view transform using a camera position, a focal point, and an up direction.
    pub fn look_at(
        camera_pos: Vec3,
        focal_point: Vec3,
        up_dir: Vec3,
        o: &Arc<dyn Hittable>,
    ) -> Self {
        Self {
            matrix: Affine3A::look_at_rh(camera_pos, focal_point, up_dir),
            object: Arc::clone(o),
        }
    }

    // builders

    /// Adds a scaling factor to the existing affine transform
    pub fn with_scale_factor(&mut self, scale: Vec3) -> &mut Self {
        self.matrix = self.matrix * Affine3A::from_scale(scale);
        self
    }

    /// Adds a rotation based on the axis and angle to the existing affine transform
    pub fn with_axis_angle(&mut self, axis: Vec3, angle: f32) -> &mut Self {
        self.matrix = self.matrix * Affine3A::from_axis_angle(axis, angle);
        self
    }

    /// Adds a translation to the existing affine transform
    pub fn with_translation(&mut self, translation: Vec3) -> &mut Self {
        self.matrix = self.matrix * Affine3A::from_translation(translation);
        self
    }

    pub fn finalize(&mut self) -> Self {
        println!("{:#?}", self.matrix);
        Self {
            matrix: self.matrix,
            object: self.object.to_owned(),
        }
    }
}

impl Hittable for Transform {
    fn hit(&self, ray: &crate::ray::Ray, t_min: f32, t_max: f32) -> Option<super::HitRecord> {
        let transformed_ray = crate::ray::Ray::new(
            self.matrix.transform_point3a(ray.origin),
            self.matrix.transform_vector3a(ray.direction),
            ray.time,
        );

        match self.object.hit(&transformed_ray, t_min, t_max) {
            Some(rec) => {
                let mut transformed_rec = super::HitRecord {
                    point: self.matrix.inverse().transform_point3a(rec.point),
                    ..rec
                };
                transformed_rec.set_face_normal(&transformed_ray, rec.normal);
                Some(transformed_rec)
            }
            None => None,
        }
    }

    fn bounding_box(&self, time0: f32, time1: f32) -> Option<crate::bounds::BoundingBox> {
        let original_box = self.object.bounding_box(time0, time1);
        match original_box {
            Some(original_box) => {
                let new_min = self.matrix.transform_point3a(original_box.min);
                let new_max = self.matrix.transform_point3a(original_box.max);
                Some(crate::bounds::BoundingBox::new(new_min, new_max))
            }
            None => None,
        }
    }
}
