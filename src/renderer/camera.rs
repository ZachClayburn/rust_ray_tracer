use crate::geometry::{Point3, Ray, Vec3};

pub struct Camera {
    pub origin: Point3,
    pub lower_left_corner: Point3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
}

impl Camera {
    pub fn new(image_width: u32, image_heigth: u32) -> Self {
        let aspect_ratio = image_width as f64 / image_heigth as f64;
        let viewport_heigth = 2.;
        let viewport_width = viewport_heigth * aspect_ratio;
        let focal_length = 1.;

        let origin = Vec3::zeros();
        let horizontal = viewport_width * Vec3::i();
        let vertical = viewport_heigth * Vec3::j();
        let lower_left_corner =
            origin - horizontal / 2. - vertical / 2. - (focal_length * Vec3::k());

        Self {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + u * self.horizontal + v * self.vertical,
        )
    }
}
