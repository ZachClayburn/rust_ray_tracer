mod geometry;
mod renderer;

fn main() {
    let imgbuf = renderer::Renderer::new()
        .image_width(400)
        .image_heigth(225)
        .render();
    imgbuf.save("test.png").unwrap();
}
