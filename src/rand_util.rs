use glam::Vec3;
use rand::Rng;

pub fn rand_f32() -> f32 {
    rand::thread_rng().gen::<f32>()
}

pub fn rand_range_f32(min: f32, max: f32) -> f32 {
    rand::thread_rng().gen_range(min..max)
}

pub fn rand_vec3() -> Vec3 {
    rand::thread_rng().gen::<Vec3>()
}

pub fn rand_range_vec3(min: f32, max: f32) -> Vec3 {
    Vec3::new(
        rand_range_f32(min, max),
        rand_range_f32(min, max),
        rand_range_f32(min, max),
    )
}

pub fn rand_vec3_in_unit_sphere() -> Vec3 {
    loop {
        let v = rand_range_vec3(-1.0, 1.0);
        if v.length_squared() < 1.0 {
            return v;
        }
    }
}

pub fn rand_unit_vec3() -> Vec3 {
    rand_vec3_in_unit_sphere().normalize()
}
