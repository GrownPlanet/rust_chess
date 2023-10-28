use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture};
use sdl2::video::Window;

pub struct Board {
    board: [[i32; 8]; 8],
}

impl Board {
    pub fn default() -> Self {
        let board = [
            [0, 0, 0, 0, 0, 0, 0, 0],
            [7, 7, 7, 7, 7, 7, 7, 7],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [1, 1, 1, 1, 1, 1, 1, 1],
            [0, 0, 0, 0, 0, 0, 0, 0],
        ];

        Self { board }
    }

    pub fn draw_empty_board(
        canvas: &mut Canvas<Window>,
        board_size: u32,
        dark_color: Color,
        light_color: Color,
    ) -> Result<(), String> {
        let tile_size = board_size / 8;
        for x in 0..8 {
            for y in 0..8 {
                let rect = Rect::new(
                    x * tile_size as i32,
                    y * tile_size as i32,
                    tile_size as u32,
                    tile_size as u32,
                );

                if (x + y) % 2 == 0 {
                    canvas.set_draw_color(light_color);
                } else {
                    canvas.set_draw_color(dark_color);
                }

                canvas.fill_rect(rect)?;
            }
        }
        Ok(())
    }

    pub fn draw_pieces(
        &self,
        canvas: &mut Canvas<Window>,
        texture: &Texture,
        board_size: u32,
    ) -> Result<(), String> {
        let tile_size = (board_size / 8) as i32;

        for x in 0..8 {
            for y in 0..8 {
                let piece = self.board[y][x];

                let dst = Rect::new(
                    x as i32 * tile_size,
                    y as i32 * tile_size,
                    tile_size as u32,
                    tile_size as u32,
                );

                let src = match piece {
                    0 => continue,
                    1 => Rect::new(0, 0, 32, 32),
                    7 => Rect::new(0, 32, 32, 32),
                    _ => return Err(String::from("piece not found")),
                };

                canvas.copy(texture, src, dst)?;
            }
        }
        Ok(())
    }
}
