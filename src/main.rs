mod geometry;
mod renderer;

fn main() {
    let imgbuf = renderer::Renderer::new()
        .image_width(256)
        .image_heigth(256)
        .render();
    imgbuf.save("test.png").unwrap();
}
