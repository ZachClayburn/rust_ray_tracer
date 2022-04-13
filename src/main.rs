mod geometry;
mod renderer;

fn main() {
    let imgbuf = renderer::render(256, 256);
    imgbuf.save("test.png").unwrap();
}
