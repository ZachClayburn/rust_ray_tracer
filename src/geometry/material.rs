use super::{Color, HitRecord, Ray};
use rand::rngs::ThreadRng;

pub mod lambertian;

pub trait Material {
    fn scatter(
        &self,
        ray: &Ray,
        hit_record: &HitRecord,
        rng: &mut ThreadRng,
    ) -> Option<(Color, Ray)>;
}
