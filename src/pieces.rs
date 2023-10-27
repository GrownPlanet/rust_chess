use sdl2::render::Canvas;
use sdl2::video::Window;

pub trait Piece {
    fn draw(&self, canvas: &mut Canvas<Window>) -> Result<(), String>;
}

pub struct Pawn {}
