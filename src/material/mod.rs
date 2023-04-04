use crate::geometry::{Color, HitRecord, Ray};
use rand::rngs::ThreadRng;

mod lambertian;
pub use lambertian::Lambertian;
mod metal;
pub use metal::Metal;
mod dialectric;
pub use dialectric::Dialectric;

#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub enum MaterialEnum {
    Dialectric(Dialectric),
    Lambertian(Lambertian),
    Metal(Metal),
}

impl Material for MaterialEnum {
    fn scatter(
        &self,
        ray: &Ray,
        hit_record: &HitRecord,
        rng: &mut ThreadRng,
    ) -> Option<(Color, Ray)> {
        match self {
            MaterialEnum::Dialectric(x) => x.scatter(ray, hit_record, rng),
            MaterialEnum::Lambertian(x) => x.scatter(ray, hit_record, rng),
            MaterialEnum::Metal(x) => x.scatter(ray, hit_record, rng),
        }
    }
}

pub trait Material {
    fn scatter(
        &self,
        ray: &Ray,
        hit_record: &HitRecord,
        rng: &mut ThreadRng,
    ) -> Option<(Color, Ray)>;
}
