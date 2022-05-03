mod vec3;
pub use vec3::{Color, Point3, Vec3};

mod ray;
pub use ray::Ray;

mod hittable;
pub use hittable::hitable_list::HitableList;
pub use hittable::sphere::Sphere;
pub use hittable::{HitRecord, Hittable};

mod material;
pub use material::dialectric::Dialectric;
pub use material::lambertian::Lambertian;
pub use material::metal::Metal;
pub use material::Material;
