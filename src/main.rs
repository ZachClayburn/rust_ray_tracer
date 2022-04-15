mod geometry;
mod renderer;

fn main() {
    let imgbuf = renderer::Renderer::new()
        .image_width(1920)
        .image_heigth(1080)
        .render();
    imgbuf.save("test.png").unwrap();
}
