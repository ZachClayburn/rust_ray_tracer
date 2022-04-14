mod lambertian;
mod ray;
mod sphere;
mod vec3;

use std::{ops::Range, rc::Rc};

pub use lambertian::Lambertian;
use rand::rngs::ThreadRng;
pub use ray::Ray;
pub use sphere::Sphere;
pub use vec3::{Color, Point3, Vec3};

pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub material: Rc<dyn Material>,
}

impl HitRecord {
    pub fn new(
        point: Point3,
        t: f64,
        ray: &Ray,
        outward_normal: Vec3,
        material: Rc<dyn Material>,
    ) -> Self {
        let front_face = ray.direction.dot(outward_normal) < 0.;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };
        Self {
            point,
            normal,
            t,
            front_face,
            material,
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_range: Range<f64>) -> Option<HitRecord>;
}

pub struct HitableList {
    list: Vec<Box<dyn Hittable>>,
}

impl HitableList {
    pub fn new() -> Self {
        Self { list: Vec::new() }
    }

    pub fn add(&mut self, hittable: Box<dyn Hittable>) {
        self.list.push(hittable);
    }
}

impl Hittable for HitableList {
    fn hit(&self, ray: &Ray, t_range: Range<f64>) -> Option<HitRecord> {
        self.list
            .iter()
            .filter_map(|x| x.hit(ray, t_range.clone()))
            .filter(|x| x.t.is_finite())
            .min_by(|lhs, rhs| {
                lhs.t
                    .partial_cmp(&rhs.t)
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
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
