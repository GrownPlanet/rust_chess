use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture};
use sdl2::video::Window;

const WP: i32 = 1;
const WH: i32 = 2;
const WB: i32 = 3;
const WR: i32 = 4;
const WQ: i32 = 5;
const WK: i32 = 6;
const BP: i32 = 7;
const BH: i32 = 8;
const BB: i32 = 9;
const BR: i32 = 10;
const BQ: i32 = 11;
const BK: i32 = 12;

const W_PIECES: [i32; 6] = [WP, WH, WB, WR, WQ, WK];
const B_PIECES: [i32; 6] = [BP, BH, BB, BR, BQ, BK];

pub struct Board {
    board: [[i32; 8]; 8],
}

impl Board {
    pub fn default() -> Self {
        let board = [
            [BR, BH, BB, BQ, BK, BB, BH, BR],
            [BP, BP, BP, BP, BP, BP, BP, BP],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [WP, WP, WP, WP, WP, WP, WP, WP],
            [WR, WH, WB, WQ, WK, WB, WH, WR],
        ];

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

        let dirs = match piece {
            WR | BR => vec![(0, 1), (1, 0), (0, -1), (-1, 0)],
            WB | BB => vec![(1, 1), (-1, 1), (1, -1), (-1, -1)],
            WQ | WK | BQ | BK => vec![
                (1, 1),
                (-1, 1),
                (1, -1),
                (-1, -1),
                (0, 1),
                (1, 0),
                (0, -1),
                (-1, 0),
            ],
            WP => vec![(0, -1)],
            BP => vec![(0, 1)],
            BH | WH => vec![
                (2, 1),
                (-2, 1),
                (2, -1),
                (-2, -1),
                (1, 2),
                (2, 1),
                (1, -2),
                (-2, 1),
                (-1, 2),
                (-1, -2),
            ],
            _ => vec![],
        };

        let mut iters = match piece {
            WR | BR | WB | BB | WQ | BQ => 8,
            WP | WH | WK | BP | BH | BK => 2,
            _ => 0,
        };

        if piece == WP && y == 6 {
            iters = 3;
        }
        if piece == BP && y == 1 {
            iters = 3;
        }

        for (dir_x, dir_y) in dirs {
            if piece == WP || piece == BP {
                let dirs = if is_white {
                    [(-1, -1), (1, -1)]
                } else {
                    [(-1, 1), (1, 1)]
                };
                for (dir_x, dir_y) in dirs {
                    let new_x = (x as i32 + dir_x) as usize;
                    let new_y = (y as i32 + dir_y) as usize;

                    if new_x >= 8 || new_y >= 8 {
                        break;
                    }

                    if is_white && B_PIECES.contains(&self.board[new_y][new_x]) {
                        return_vec.push((new_x, new_y));
                    } else if is_black && W_PIECES.contains(&self.board[new_y][new_x]) {
                        return_vec.push((new_x, new_y));
                    }
                }
            }
            for i in 1..iters {
                let new_x = (x as i32 + dir_x * i) as usize;
                let new_y = (y as i32 + dir_y * i) as usize;

                if new_x >= 8 || new_y >= 8 {
                    break;
                }

                if self.board[new_y][new_x] == 0 {
                    return_vec.push((new_x, new_y));
                } else if piece != WP && piece != BP {
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
        if self.board[y][x] <= 6 {
            return 1;
        }
        return -1;
    }
}
