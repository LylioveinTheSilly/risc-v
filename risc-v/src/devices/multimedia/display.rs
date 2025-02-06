use sdl2;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub struct Display {
    canvas: Canvas<Window>,
}
impl Display {
    pub fn new(canvas: Canvas<Window>) -> Self {
        Self { canvas }
    }
}