use super::{Point3, Ray, Vec3};
use crate::material::{Material, MaterialEnum};
use serde::{Deserialize, Serialize};
use std::ops::Range;

pub mod hitable_list;
pub mod plane;
pub mod sphere;

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_range: Range<f64>) -> Option<HitRecord>;
}

#[derive(Serialize, Deserialize)]
pub enum HitEnum {
    List(hitable_list::HitableList),
    Plane(plane::Plane),
    Sphere(sphere::Sphere),
}

impl Hittable for HitEnum {
    fn hit(&self, ray: &Ray, t_range: Range<f64>) -> Option<HitRecord> {
        match self {
            Self::List(x) => x.hit(ray, t_range),
            Self::Plane(x) => x.hit(ray, t_range),
            Self::Sphere(x) => x.hit(ray, t_range),
        }
    }
}

pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub material: MaterialEnum,
}

impl HitRecord {
    pub fn new(
        point: Point3,
        t: f64,
        ray: &Ray,
        outward_normal: Vec3,
        material: MaterialEnum,
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
