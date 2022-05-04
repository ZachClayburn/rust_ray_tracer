use std::boxed::Box;
use std::rc::Rc;

use super::{HitRecord, Hittable, Material, Point3, Vec3};

#[derive(Clone)]
pub struct Plane {
    point: Point3,
    normal: Vec3,
    material: Rc<dyn Material>,
}

impl Plane {
    pub fn new(point: Point3, normal: Vec3, material: Rc<dyn Material>) -> Box<dyn Hittable> {
        Box::new(Self {
            point,
            normal: normal.unit_vector(),
            material,
        })
    }
}

impl Hittable for Plane {
    fn hit(&self, ray: &crate::geometry::Ray, t_range: std::ops::Range<f64>) -> Option<HitRecord> {
        let denom = self.normal.dot(ray.direction);
        if denom.abs() <= 1.0e-6 {
            return None;
        }

        let ray_origin_to_plane_point = self.point - ray.origin;

        let t = ray_origin_to_plane_point.dot(self.normal) / denom;

        if t_range.contains(&t) {
            let point = ray.at(t);
            Some(HitRecord::new(
                point,
                t,
                ray,
                self.normal,
                self.material.clone(),
            ))
        } else {
            None
        }
    }
}
