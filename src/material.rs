use crate::geometry::{Color, HitRecord, Ray};
use rand::rngs::ThreadRng;

mod lambertian;
pub use lambertian::Lambertian;
mod metal;
pub use metal::Metal;
mod dialectric;
pub use dialectric::Dialectric;

pub trait Material {
    fn scatter(
        &self,
        ray: &Ray,
        hit_record: &HitRecord,
        rng: &mut ThreadRng,
    ) -> Option<(Color, Ray)>;
}
