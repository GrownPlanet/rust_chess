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
const BR: i32 = 10;
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
        board[4][2] = WR;

        Self { board }
    }

    pub fn draw_empty_board(
        canvas: &mut Canvas<Window>,
        tile_size: u32,
        dark_color: Color,
        light_color: Color,
    ) -> Result<(), String> {
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
        tile_size: u32,
    ) -> Result<(), String> {
        for y in 0..8 {
            for x in 0..8 {
                let piece = self.board[y][x];

                let dst = Rect::new(
                    x as i32 * tile_size as i32,
                    y as i32 * tile_size as i32,
                    tile_size,
                    tile_size,
                );

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
        let piece = self.board[y][x];

        if piece == 0 {
            return vec![];
        }

        let mut return_vec = vec![];

        match piece {
            WR | BR => {
                for k in 0..x {
                    return_vec.push((k, y))
                }
                for k in (x + 1)..8 {
                    return_vec.push((k, y))
                }
                for k in 0..y {
                    return_vec.push((x, k))
                }
                for k in (y + 1)..8 {
                    return_vec.push((x, k))
                }
            }
            _ => (),
        }

        return_vec
    }

    pub fn move_piece(&mut self, from: (usize, usize), to: (usize, usize)) {
        self.board[to.1][to.0] = self.board[from.1][from.0];
        self.board[from.1][from.0] = 0;
    }
}
