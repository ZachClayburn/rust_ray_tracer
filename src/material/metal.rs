use super::Material;
use crate::geometry::{Color, HitRecord, Ray, Vec3};
use rand::prelude::ThreadRng;
use std::rc::Rc;

pub struct Metal {
    albedo: Color,
    roughness: Option<f64>,
}

impl Metal {
    pub fn new(albedo: Color, roughness: Option<f64>) -> Rc<dyn Material> {
        let roughness = roughness.map(|x| x.clamp(0.0, 1.0));
        Rc::new(Self { albedo, roughness })
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        ray: &Ray,
        hit_record: &HitRecord,
        rng: &mut ThreadRng,
    ) -> Option<(Color, Ray)> {
        let fuzz = self.roughness.map_or_else(
            || Vec3::zeros(),
            |roughness| roughness * Vec3::random_unit_vector(rng),
        );
        let reflected = ray.direction.reflect(hit_record.normal).unit_vector() + fuzz;
        if reflected.dot(hit_record.normal) > 0.0 {
            Some((self.albedo, Ray::new(hit_record.point, reflected)))
        } else {
            None
        }
    }
}
