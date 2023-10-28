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
    let board_size = 800;

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

    let board = Board::default();

    let mut mouse_state;
    let mut board_coords = (0, 0);

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

        mouse_state = MouseState::new(&event_pump);

        if mouse_state.left() {
            board_coords = pos_to_board_coords(mouse_state.x(), mouse_state.y(), board_size as i32);
        }

        canvas.set_blend_mode(BlendMode::None);
        Board::draw_empty_board(&mut canvas, board_size, dark_color, light_color)?;
        board.draw_pieces(&mut canvas, &pieces_texture, board_size)?;

        let legal_moves = board.get_moves(board_coords.0, board_coords.1);

        canvas.set_blend_mode(BlendMode::Mul);
        for m in legal_moves {
            canvas.set_draw_color(Color::RGB(200, 50, 50));
            canvas.fill_rect(Rect::new(m.0 as i32 * 100, m.1 as i32 * 100, 100, 100))?;
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
    }

    Ok(())
}

fn pos_to_board_coords(x: i32, y: i32, board_size: i32) -> (usize, usize) {
    let ts = board_size / 8;

    let bx = x / ts;
    let by = y / ts;

    (bx as usize, by as usize)
}
