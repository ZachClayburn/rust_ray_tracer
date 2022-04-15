use std::boxed::Box;
use std::rc::Rc;

use super::{HitRecord, Hittable, Material, Point3};

#[derive(Clone)]
pub struct Sphere {
    center: Point3,
    radius: f64,
    material: Rc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, material: Rc<dyn Material>) -> Box<dyn Hittable> {
        Box::new(Sphere {
            center,
            radius,
            material,
        })
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &super::Ray, t_range: std::ops::Range<f64>) -> Option<super::HitRecord> {
        let origin_to_center = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = origin_to_center.dot(ray.direction);
        let c = origin_to_center.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0. {
            return None;
        }
        let squrt_d = discriminant.sqrt();

        let t = match squrt_d {
            x if t_range.contains(&((-half_b - x) / a)) => ((-half_b - x) / a),
            x if t_range.contains(&((-half_b + x) / a)) => ((-half_b + x) / a),
            _ => return None,
        };

        let point = ray.at(t);
        let outward_normal = (point - self.center) / self.radius;

        Some(HitRecord::new(
            point,
            t,
            ray,
            outward_normal,
            self.material.clone(),
        ))
    }
}
