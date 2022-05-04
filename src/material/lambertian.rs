use rand::prelude::ThreadRng;

use super::{Color, HitRecord, Material, Ray};
use std::rc::Rc;

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Rc<dyn Material> {
        Rc::new(Self { albedo })
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        _ray: &Ray,
        hit_record: &HitRecord,
        rng: &mut ThreadRng,
    ) -> Option<(Color, Ray)> {
        let scatter_direction =
            match hit_record.normal + hit_record.normal.random_unit_vector_in_direction(rng) {
                x if x.near_zero() => hit_record.normal,
                x => x,
            };
        let scattered = Ray::new(hit_record.point, scatter_direction);
        Some((self.albedo, scattered))
    }
}
