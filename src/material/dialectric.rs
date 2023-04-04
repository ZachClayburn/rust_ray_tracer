use rand::Rng;

use crate::geometry::Vec3;

use super::{Color, HitRecord, Material, MaterialEnum, Ray};

#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct Dialectric {
    index_of_refraction: f64,
}

impl Dialectric {
    pub fn new(index_of_refraction: f64) -> MaterialEnum {
        MaterialEnum::Dialectric(Self {
            index_of_refraction,
        })
    }
}

impl Material for Dialectric {
    fn scatter(
        &self,
        ray: &Ray,
        hit_record: &HitRecord,
        rng: &mut rand::prelude::ThreadRng,
    ) -> Option<(Color, Ray)> {
        let refraction_ratio = if hit_record.front_face {
            1.0 / self.index_of_refraction
        } else {
            self.index_of_refraction
        };

        let unit_direction = ray.direction.unit_vector();

        let cos_theta = (-unit_direction).dot(hit_record.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let cannot_refract = refraction_ratio * sin_theta > 1.0;

        let dist = rand::distributions::Uniform::new_inclusive(0.0, 1.0);
        let direction =
            if cannot_refract || reflectance(cos_theta, refraction_ratio) > rng.sample(dist) {
                Vec3::reflect(unit_direction, hit_record.normal)
            } else {
                Vec3::refract(unit_direction, hit_record.normal, refraction_ratio)
            };

        Some((Color::ones(), Ray::new(hit_record.point, direction)))
    }
}

fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
    let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
}
