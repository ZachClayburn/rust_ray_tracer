use serde::{Deserialize, Serialize};

mod vec3;
pub use vec3::{Color, Point3, Vec3};

mod ray;
pub use ray::Ray;

mod hittable;
pub use hittable::hitable_list::HitableList;
pub use hittable::plane::Plane;
pub use hittable::sphere::Sphere;
pub use hittable::{HitRecord, Hittable};

#[derive(Serialize, Deserialize)]
pub struct SceneDescription {
    pub hittable: HitableList,
    pub camera_width: u32,
    pub camera_height: u32,
}
