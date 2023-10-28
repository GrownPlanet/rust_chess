extern crate sdl2;

use sdl2::event::Event;
use sdl2::image::LoadTexture;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

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

        Board::draw_empty_board(&mut canvas, board_size, dark_color, light_color)?;
        board.draw_pieces(&mut canvas, &pieces_texture, board_size)?;

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
    }

    Ok(())
}
