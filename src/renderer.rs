mod camera;

use self::camera::Camera;
use crate::geometry::{Color, HitableList, Hittable, Lambertian, Metal, Ray, Sphere, Vec3};
use indicatif::{ProgressBar, ProgressIterator, ProgressStyle};
use rand::prelude::*;
use rand::seq::SliceRandom;

pub type ImageBuffer = image::ImageBuffer<image::Rgb<u8>, Vec<u8>>;

pub struct Renderer {
    image_width: u32,
    image_height: u32,
    samples_per_pixle: usize,
    max_depth: usize,
}

impl Renderer {
    pub fn new() -> Self {
        Self {
            image_width: 256,
            image_height: 256,
            samples_per_pixle: 600,
            max_depth: 50,
        }
    }

    pub fn image_width(mut self, image_width: u32) -> Self {
        self.image_width = image_width;
        self
    }

    pub fn image_height(mut self, image_heigth: u32) -> Self {
        self.image_height = image_heigth;
        self
    }

    pub fn samples_per_pixle(mut self, samples_per_pixle: usize) -> Self {
        self.samples_per_pixle = samples_per_pixle;
        self
    }

    pub fn max_depth(mut self, max_depth: usize) -> Self {
        self.max_depth = max_depth;
        self
    }

    fn get_scene(&self) -> (HitableList, Camera) {
        let ground_material = Lambertian::new(Color::new(0.8, 0.8, 0.0));
        let center_material = Lambertian::new(Color::new(0.7, 0.3, 0.3));
        let left_material = Metal::new(Color::new(0.8, 0.8, 0.8), Some(1.0));
        let right_material = Metal::new(Color::new(0.8, 0.6, 0.2), None);
        let mut world = HitableList::new();
        world.add(Sphere::new(
            (0.0, -100.5, -1.).into(),
            100.0,
            ground_material,
        ));
        world.add(Sphere::new((0.0, 0.0, -1.0).into(), 0.5, center_material));
        world.add(Sphere::new((-1., 0.0, -1.0).into(), 0.5, left_material));
        world.add(Sphere::new((1.0, 0.0, -1.0).into(), 0.5, right_material));
        (world, Camera::new(self.image_width, self.image_height))
    }

    pub fn render(self) -> ImageBuffer {
        let (world, camera) = self.get_scene();

        let mut imgbuf = ImageBuffer::new(self.image_width, self.image_height);

        let num_pixels = self.image_width as u64 * self.image_height as u64;

        let mut rng = thread_rng();
        let dist = rand::distributions::Uniform::new_inclusive(-0.5, 0.5);

        let mut image_data = imgbuf.enumerate_pixels_mut().collect::<Vec<_>>();
        image_data.shuffle(&mut rng);

        let prog_bar = ProgressBar::new(num_pixels)
            .with_style(
                ProgressStyle::default_bar()
                    .template("[{elapsed_precise}/{duration_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
                    .on_finish(indicatif::ProgressFinish::WithMessage("Done!".into()))
                    .progress_chars("█▓▒░")
            )
            .with_message("Rendering image...");
        for (x, y, pixel) in image_data.into_iter().progress_with(prog_bar) {
            let x = x as f64;
            let y = (self.image_height - y) as f64;
            let image_width = (self.image_width - 1) as f64;
            let image_height = (self.image_height - 1) as f64;

            *pixel = (std::iter::repeat_with(|| {
                let u = (x + rng.sample(dist)) / image_width;
                let v = (y + rng.sample(dist)) / image_height;

                let ray = camera.get_ray(u, v);
                ray_color(ray, &world, &mut rng, self.max_depth)
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
        let red = self.x.clamp(0.0, 0.999).sqrt();
        let green = self.y.clamp(0.0, 0.999).sqrt();
        let blue = self.z.clamp(0.0, 0.999).sqrt();

        let conversion_nubmer = u8::MAX as f64 + 1.0;
        let red = (red * conversion_nubmer) as u8;
        let green = (green * conversion_nubmer) as u8;
        let blue = (blue * conversion_nubmer) as u8;

        image::Rgb([red, green, blue])
    }
}

fn ray_color(ray: Ray, world: &HitableList, rng: &mut ThreadRng, depth: usize) -> Color {
    if depth <= 0 {
        return Color::zeros();
    }
    if let Some(hit_record) = world.hit(&ray, 0.001..f64::INFINITY) {
        hit_record
            .material
            .scatter(&ray, &hit_record, rng)
            .map_or(Color::zeros(), |(attenuation, scattered)| {
                attenuation * ray_color(scattered, &world, rng, depth - 1)
            })
    } else {
        let unit_direction = ray.direction.unit_vector();
        let t = 0.5 * (unit_direction.y + 1.);
        (1. - t) * Color::ones() + t * Color::new(0.5, 0.7, 1.0)
    }
}
