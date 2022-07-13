extern crate sdl2;

use sdl2::event::Event;
use sdl2::image::{InitFlag, LoadTexture};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::time::Duration;

mod level;
mod util;
use util::*;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG);
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Ultimate Tapan Kaikki - Level Editor", 640, 480)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let texture_creator = canvas.texture_creator();
    let texture = texture_creator.load_texture("FLOOR1.PNG").unwrap();

    let mut level = [[0u32; 16]; 12];
    init_empty_level(&mut level);

    // Test level export
    level::serialize("./TEST.LEV", level).unwrap();

    let mut tile_select_mode = false;

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::Space),
                    ..
                } => {
                    tile_select_mode = !tile_select_mode;
                }
                _ => {}
            }
        }

        if tile_select_mode {
            canvas.set_draw_color(Color::from((0, 0, 0)));
            canvas.clear();
            let dst = Rect::new(0, 0, 640, 400);
            canvas.copy(&texture, None, dst).unwrap();
        } else {
            render_level(level, &mut canvas, &texture);
        }

        let state = event_pump.mouse_state();
        let highlighted_id = get_tile_id_from_coordinate(state.x() as u32, state.y() as u32);

        highlight_selected_tile(highlighted_id, &mut canvas);
        canvas.present();

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

fn render_level(
    level: [[u32; 16]; 12],
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    texture: &sdl2::render::Texture,
) {
    for y in 0..level.len() {
        for x in 0..level[0].len() {
            let src = get_block(level[y][x]);
            let dst = Rect::new(
                (x * RENDER_SIZE as usize).try_into().unwrap(),
                (y * RENDER_SIZE as usize).try_into().unwrap(),
                RENDER_SIZE,
                RENDER_SIZE,
            );
            canvas.copy(&texture, src, dst).unwrap();
        }
    }
}

fn init_empty_level(level: &mut [[u32; 16]; 12]) {
    for x in 0..level[0].len() {
        level[0][x] = 1;
    }
    for y in 1..(level.len() - 1) {
        for x in 0..level[0].len() {
            if x == 0 || x == level[0].len() - 1 {
                level[y][x] = 1;
            } else {
                level[y][x] = 0;
            }
        }
    }
    for x in 0..level[0].len() {
        level[level.len() - 1][x] = 1;
    }
}
