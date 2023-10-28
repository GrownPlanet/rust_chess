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

const W_PIECES: [i32; 6] = [_WP, _WH, _WB, WR, _WQ, _WK];
const B_PIECES: [i32; 6] = [_BP, _BH, _BB, BR, _BQ, _BK];

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
        board[4][1] = BR;
        board[7][3] = WR;
        board[1][6] = BR;

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

    pub fn get_moves(&self, (x, y): (usize, usize)) -> Vec<(usize, usize)> {
        let piece = self.board[y][x];

        let is_white = W_PIECES.contains(&piece);
        let is_black = B_PIECES.contains(&piece);

        if piece == 0 {
            return vec![];
        }

        let mut return_vec = vec![];

        for (dir_x, dir_y) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
            for i in 1..8 {
                let new_x = (x as i32 + dir_x * i) as usize;
                let new_y = (y as i32 + dir_y * i) as usize;

                if new_x >= 8 || new_y >= 8 {
                    break;
                }

                if self.board[new_y][new_x] == 0 {
                    return_vec.push((new_x, new_y));
                } else {
                    if is_white && B_PIECES.contains(&self.board[new_y][new_x]) {
                        return_vec.push((new_x, new_y));
                    } else if is_black && W_PIECES.contains(&self.board[new_y][new_x]) {
                        return_vec.push((new_x, new_y));
                    }
                    break;
                }
            }
        }

        return_vec
    }

    pub fn move_piece(&mut self, from: (usize, usize), to: (usize, usize)) {
        self.board[to.1][to.0] = self.board[from.1][from.0];
        self.board[from.1][from.0] = 0;
    }

    pub fn is_piece(&self, (x, y): (usize, usize)) -> bool {
        self.board[y][x] != 0
    }

    pub fn get_piece_color(&self, (x, y): (usize, usize)) -> i32 {
        if self.board[y][x] < 6 {
            return 1;
        }
        return -1;
    }
}
