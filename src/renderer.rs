use indicatif::{ProgressBar, ProgressIterator};

pub type ImageBuffer = image::ImageBuffer<image::Rgb<u8>, Vec<u8>>;

pub fn render(width: u32, height: u32) -> ImageBuffer {
    let mut imgbuf = ImageBuffer::new(width, height);

    let num_pixels = width as u64 * height as u64;

    for (x, y, pixel) in imgbuf
        .enumerate_pixels_mut()
        .progress_with(ProgressBar::new(num_pixels))
    {
        let r = x as f64 / width as f64;
        let g = (height - y) as f64 / height as f64;
        let b = 0.25;

        let ir = (u8::MAX as f64 * r) as u8;
        let ig = (u8::MAX as f64 * g) as u8;
        let ib = (u8::MAX as f64 * b) as u8;

        *pixel = image::Rgb([ir, ig, ib]);
    }

    imgbuf
}
