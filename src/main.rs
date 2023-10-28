extern crate sdl2;

use sdl2::event::Event;
use sdl2::image::LoadTexture;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseState;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::BlendMode;

use std::path::Path;
use std::time::Duration;

use board::Board;

pub mod board;

pub fn main() -> Result<(), String> {
    let tile_size = 100;
    let board_size = 8 * tile_size;

    let dark_color = Color::RGB(34, 32, 52);
    let light_color = Color::RGB(255, 255, 255);

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("chess", board_size, board_size)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    let mut event_pump = sdl_context.event_pump()?;

    let texture_creator = canvas.texture_creator();

    let pieces_texture = texture_creator.load_texture(Path::new("assets/pieces.png"))?;

    let mut board = Board::default();

    let mut mouse_state;

    let mut selected_piece = (0, 0);
    let mut board_coords = (0, 0);

    let mut legal_moves = vec![];

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        // get mouse state
        mouse_state = MouseState::new(&event_pump);

        // check if wanting to select a piece
        if mouse_state.left() {
            board_coords = pos_to_board_coords(mouse_state.x(), mouse_state.y(), tile_size as i32);

            if legal_moves.contains(&board_coords) {
                board.move_piece(selected_piece, board_coords);
            }

            selected_piece = board_coords;
        }

        canvas.set_blend_mode(BlendMode::None);

        Board::draw_empty_board(&mut canvas, tile_size, dark_color, light_color)?;
        board.draw_pieces(&mut canvas, &pieces_texture, tile_size)?;

        legal_moves = board.get_moves(board_coords.0, board_coords.1);

        canvas.set_blend_mode(BlendMode::Mul);

        for m in &legal_moves {
            canvas.set_draw_color(Color::RGB(200, 50, 50));
            canvas.fill_rect(Rect::new(m.0 as i32 * 100, m.1 as i32 * 100, 100, 100))?;
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
    }

    Ok(())
}

fn pos_to_board_coords(x: i32, y: i32, tile_size: i32) -> (usize, usize) {
    let bx = x / tile_size;
    let by = y / tile_size;

    (bx as usize, by as usize)
}
