use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture};
use sdl2::video::Window;

const _WP: i32 = 1;
const _WH: i32 = 2;
const _WB: i32 = 3;
const WR: i32 = 4;
const _WQ: i32 = 5;
const _WK: i32 = 6;
const _BP: i32 = 7;
const _BH: i32 = 8;
const _BB: i32 = 9;
const _BR: i32 = 10;
const _BQ: i32 = 11;
const _BK: i32 = 12;

pub struct Board {
    board: [[i32; 8]; 8],
}

impl Board {
    pub fn default() -> Self {
        // let board = [
        //     [10, 9, 8, 11, 12, 8, 9, 10],
        //     [7, 7, 7, 7, 7, 7, 7, 7],
        //     [0, 0, 0, 0, 0, 0, 0, 0],
        //     [0, 0, 0, 0, 0, 0, 0, 0],
        //     [0, 0, 0, 0, 0, 0, 0, 0],
        //     [0, 0, 0, 0, 0, 0, 0, 0],
        //     [1, 1, 1, 1, 1, 1, 1, 1],
        //     [4, 3, 2, 5, 6, 2, 3, 4],
        // ];

        let mut board = [[0; 8]; 8];
        board[4][4] = WR;

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
        let ts = (board_size / 8) as i32;

        for y in 0..8 {
            for x in 0..8 {
                let piece = self.board[y][x];

                let dst = Rect::new(x as i32 * ts, y as i32 * ts, ts as u32, ts as u32);

                let src = match piece {
                    0 => continue,
                    1..=6 => Rect::new(32 * (piece - 1), 0, 32, 32),
                    7..=12 => Rect::new(32 * (piece - 7), 32, 32, 32),
                    _ => return Err(String::from("piece not found")),
                };

                canvas.copy(texture, src, dst)?;
            }
        }
        Ok(())
    }

    pub fn get_moves(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        vec![(4, 3), (4, 2)]
    }
}
