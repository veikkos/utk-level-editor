extern crate sdl2;

use sdl2::event::Event;
use sdl2::image::{InitFlag, LoadTexture};
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use std::time::Duration;

fn get_block(id: u32) -> Rect {
    const SIZE: u32 = 20;
    let x = id * SIZE % 320;
    let y = id * SIZE / 320 * SIZE;
    Rect::new(x as i32, y as i32, SIZE, SIZE)
}

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG);
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Ultimate Tapan Kaikki - Level Editor", 640, 400)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let texture_creator = canvas.texture_creator();
    let texture = texture_creator.load_texture("FLOOR1.PNG").unwrap();

    let mut level = [[0u32; 16]; 10];
    init_empty_level(&mut level);

    for y in 0..level.len() {
        for x in 0..level[0].len() {
            let src = get_block(level[y][x]);
            let dst = Rect::new(
                (x * 40).try_into().unwrap(),
                (y * 40).try_into().unwrap(),
                40,
                40,
            );
            canvas.copy(&texture, src, dst).unwrap();
        }
    }

    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();
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
        // TODO :|

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

fn init_empty_level(level: &mut [[u32; 16]; 10]) {
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
        level[9][x] = 1;
    }
}
