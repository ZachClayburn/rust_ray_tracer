use indicatif::{ProgressBar, ProgressIterator};

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
        let mut imgbuf = ImageBuffer::new(self.image_width, self.image_heigth);

        let num_pixels = self.image_width as u64 * self.image_heigth as u64;

        for (x, y, pixel) in imgbuf
            .enumerate_pixels_mut()
            .progress_with(ProgressBar::new(num_pixels))
        {
            let r = x as f64 / self.image_width as f64;
            let g = (self.image_heigth - y) as f64 / self.image_heigth as f64;
            let b = 0.25;

            let ir = (u8::MAX as f64 * r) as u8;
            let ig = (u8::MAX as f64 * g) as u8;
            let ib = (u8::MAX as f64 * b) as u8;

            *pixel = image::Rgb([ir, ig, ib]);
        }

        imgbuf
    }
}
