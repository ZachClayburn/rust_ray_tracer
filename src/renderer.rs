use indicatif::{ProgressBar, ProgressIterator};

use crate::geometry::{Color, HitableList, Hittable, Point3, Ray, Sphere, Vec3};

pub type ImageBuffer = image::ImageBuffer<image::Rgb<u8>, Vec<u8>>;

pub struct Renderer {
    image_width: u32,
    image_heigth: u32,
}

impl Renderer {
    pub fn new() -> Self {
        Self {
            image_width: 256,
            image_heigth: 256,
        }
    }

    pub fn image_width(mut self, image_width: u32) -> Self {
        self.image_width = image_width;
        self
    }

    pub fn image_heigth(mut self, image_heigth: u32) -> Self {
        self.image_heigth = image_heigth;
        self
    }

    pub fn render(self) -> ImageBuffer {
        // Image
        let aspect_ratio = self.image_width as f64 / self.image_heigth as f64;
        let viewport_heigth = 2.;
        let viewport_width = viewport_heigth * aspect_ratio;
        let focal_length = 1.;

        // World
        let world = {
            let mut world = HitableList::new();
            world.add(Box::new(Sphere::new(-Vec3::k(), 0.5)));
            world.add(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.), 100.0)));
            world
        };

        // Camera
        let origin = Vec3::zeros();
        let horizontal = viewport_width * Vec3::i();
        let vertical = viewport_heigth * Vec3::j();
        let lower_left_corner =
            origin - horizontal / 2. - vertical / 2. - (focal_length * Vec3::k());

        let mut imgbuf = ImageBuffer::new(self.image_width, self.image_heigth);

        let num_pixels = self.image_width as u64 * self.image_heigth as u64;

        let prog_bar = ProgressBar::new(num_pixels);
        prog_bar.set_draw_delta(num_pixels / 10000);
        for (x, y, pixel) in imgbuf.enumerate_pixels_mut().progress_with(prog_bar) {
            let u = x as f64 / (self.image_width - 1) as f64;
            let v = (self.image_heigth - y) as f64 / (self.image_heigth - 1) as f64;

            let ray = Ray::new(
                origin,
                (lower_left_corner + u * horizontal + v * vertical - origin).unit_vector(),
            );

            *pixel = ray_color(ray, &world);
        }

        imgbuf
    }
}

impl Into<image::Rgb<u8>> for Vec3 {
    fn into(self) -> image::Rgb<u8> {
        let conversion_nubmer = u8::MAX as f64 + 1.0 - f64::EPSILON;
        let red = (self.x * conversion_nubmer) as u8;
        let green = (self.y * conversion_nubmer) as u8;
        let blue = (self.z * conversion_nubmer) as u8;

        image::Rgb([red, green, blue])
    }
}

fn ray_color(ray: Ray, world: &HitableList) -> image::Rgb<u8> {
    let color = if let Some(hit_record) = world.hit(&ray, 0.0..f64::INFINITY) {
        0.5 * (hit_record.normal + Color::ones())
    } else {
        let unit_direction = ray.direction.unit_vector();
        let t = 0.5 * (unit_direction.y + 1.);
        (1. - t) * Color::ones() + t * Color::new(0.5, 0.7, 1.0)
    };
    color.into()
}
