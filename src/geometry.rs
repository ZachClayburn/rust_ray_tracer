mod ray;
mod sphere;
mod vec3;

use std::ops::Range;

pub use ray::Ray;
pub use sphere::Sphere;
pub use vec3::{Color, Point3, Vec3};

pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(point: Point3, t: f64, ray: &Ray, outward_normal: Vec3) -> Self {
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
