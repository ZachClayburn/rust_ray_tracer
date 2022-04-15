use super::Ray;
use super::{HitRecord, Hittable};
use std::ops::Range;

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
