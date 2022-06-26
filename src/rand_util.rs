use rand::Rng;

pub use rand;

pub fn rand_f32() -> f32 {
    rand::thread_rng().gen::<f32>()
}
