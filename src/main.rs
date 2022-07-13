extern crate sdl2;

use sdl2::event::Event;
use sdl2::image::{InitFlag, LoadTexture};
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
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
    let texture_floor = texture_creator.load_texture("./ASSETS/FLOOR1.PNG").unwrap();
    let texture_walls = texture_creator.load_texture("./ASSETS/WALLS1.PNG").unwrap();
    let mut texture_type_selected = TextureType::FLOOR;

    let mut level = [[Tile {
        texture_type: TextureType::FLOOR,
        id: 0,
    }; 16]; 12];
    init_empty_level(&mut level);

    let mut tile_select_mode = false;
    let mut selected_tile_id = 0;
    let mut mouse = (0, 0);

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
                Event::KeyDown {
                    keycode: Some(Keycode::F2),
                    ..
                } => {
                    level::serialize("./TEST.LEV", level).unwrap();
                }
                Event::KeyDown {
                    keycode: Some(Keycode::PageDown) | Some(Keycode::PageUp),
                    ..
                } => {
                    if tile_select_mode {
                        texture_type_selected = if texture_type_selected == TextureType::FLOOR {
                            TextureType::WALLS
                        } else {
                            TextureType::FLOOR
                        }
                    }
                }
                Event::MouseButtonDown {
                    mouse_btn: MouseButton::Left,
                    ..
                } => {
                    if tile_select_mode {
                        selected_tile_id = get_tile_id_from_coordinate(mouse.0, mouse.1);
                    } else {
                        let pointed_tile = get_tile_id_from_coordinate(mouse.0, mouse.1);
                        put_tile_to_level(
                            pointed_tile,
                            &mut level,
                            selected_tile_id,
                            &texture_type_selected,
                        );
                    }
                }
                _ => {}
            }
        }

        if tile_select_mode {
            canvas.set_draw_color(Color::from((0, 0, 0)));
            canvas.clear();
            let dst = Rect::new(0, 0, 640, 400);
            let texture_selected = match texture_type_selected {
                TextureType::FLOOR => &texture_floor,
                TextureType::WALLS => &texture_walls,
            };
            canvas.copy(texture_selected, None, dst).unwrap();
        } else {
            render_level(level, &mut canvas, &texture_floor, &texture_walls);
        }

        let state = event_pump.mouse_state();
        mouse.0 = state.x() as u32;
        mouse.1 = state.y() as u32;
        let highlighted_id = get_tile_id_from_coordinate(mouse.0, mouse.1);

        highlight_selected_tile(highlighted_id, &mut canvas);
        canvas.present();

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
