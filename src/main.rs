extern crate sdl2;

use sdl2::event::Event;
use sdl2::image::{InitFlag, LoadTexture};
use sdl2::keyboard::Keycode;
use std::time::Duration;

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

    canvas.copy(&texture, None, None).unwrap();
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
