use super::HitEnum;
use super::Ray;
use super::{HitRecord, Hittable};
use serde::{Deserialize, Serialize};
use std::ops::Range;

#[derive(Serialize, Deserialize)]
pub struct HitableList {
    list: Vec<HitEnum>,
}

impl HitableList {
    pub fn new() -> Self {
        Self { list: Vec::new() }
    }

    pub fn add(&mut self, hittable: HitEnum) {
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
