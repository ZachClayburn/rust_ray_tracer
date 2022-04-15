use super::Material;
use crate::geometry::{Color, HitRecord, Ray};
use rand::prelude::ThreadRng;
use std::rc::Rc;

pub struct Metal {
    albedo: Color,
}

impl Metal {
    pub fn new(albedo: Color) -> Rc<dyn Material> {
        Rc::new(Self { albedo })
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        ray: &Ray,
        hit_record: &HitRecord,
        _rng: &mut ThreadRng,
    ) -> Option<(Color, Ray)> {
        let reflected = ray.direction.reflect(hit_record.normal).unit_vector();
        if reflected.dot(hit_record.normal) > 0.0 {
            Some((self.albedo, Ray::new(hit_record.point, reflected)))
        } else {
            None
        }
    }
}
