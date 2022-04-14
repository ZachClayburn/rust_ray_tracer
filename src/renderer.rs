mod camera;

use self::camera::Camera;
use crate::geometry::{Color, HitableList, Hittable, Ray, Sphere, Vec3};
use indicatif::{ProgressBar, ProgressIterator};
use rand::prelude::*;

pub type ImageBuffer = image::ImageBuffer<image::Rgb<u8>, Vec<u8>>;

pub struct Renderer {
    image_width: u32,
    image_heigth: u32,
    samples_per_pixle: usize,
}

impl Renderer {
    pub fn new() -> Self {
        Self {
            image_width: 256,
            image_heigth: 256,
            samples_per_pixle: 100,
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
        // World
        let world = {
            let mut world = HitableList::new();
            world.add(Box::new(Sphere::new(-Vec3::k(), 0.5)));
            world.add(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.), 100.0)));
            world
        };

        // Camera
        let camera = Camera::new(self.image_width, self.image_heigth);

        let mut imgbuf = ImageBuffer::new(self.image_width, self.image_heigth);

        let num_pixels = self.image_width as u64 * self.image_heigth as u64;

        let mut rng = thread_rng();
        let dist = rand::distributions::Uniform::new_inclusive(-0.5, 0.5);

        let prog_bar = ProgressBar::new(num_pixels);
        for (x, y, pixel) in imgbuf.enumerate_pixels_mut().progress_with(prog_bar) {
            let x = x as f64;
            let y = (self.image_heigth - y) as f64;
            let image_width = (self.image_width - 1) as f64;
            let image_heigth = (self.image_heigth - 1) as f64;

            *pixel = (std::iter::repeat_with(|| {
                let u = (x + rng.sample(dist)) / image_width;
                let v = (y + rng.sample(dist)) / image_heigth;

                let ray = camera.get_ray(u, v);
                ray_color(ray, &world)
            })
            .take(self.samples_per_pixle)
            .fold(Color::zeros(), |acc, val| acc + val)
                / self.samples_per_pixle as f64)
                .into();
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

fn ray_color(ray: Ray, world: &HitableList) -> Color {
    if let Some(hit_record) = world.hit(&ray, 0.0..f64::INFINITY) {
        0.5 * (hit_record.normal + Color::ones())
    } else {
        let unit_direction = ray.direction.unit_vector();
        let t = 0.5 * (unit_direction.y + 1.);
        (1. - t) * Color::ones() + t * Color::new(0.5, 0.7, 1.0)
    }
}
