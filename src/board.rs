use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub struct Board {}

impl Board {
    pub fn draw_empty_board(canvas: &mut Canvas<Window>, window_size: i32) -> Result<(), String> {
        let tile_size = window_size / 8;
        for x in 0..8 {
            for y in 0..8 {
                let rect = Rect::new(
                    x * tile_size as i32,
                    y * tile_size as i32,
                    tile_size as u32,
                    tile_size as u32,
                );

                if (x + y) % 2 == 0 {
                    canvas.set_draw_color(Color::RGB(200, 200, 200));
                } else {
                    canvas.set_draw_color(Color::RGB(80, 40, 10));
                }

                canvas.fill_rect(rect)?;
            }
        }
        Ok(())
    }
}
