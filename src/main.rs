extern crate sdl2;

use crate::level::Level;
use sdl2::event::Event;
use sdl2::image::{InitFlag, LoadTexture};
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::time::Duration;

mod level;
mod render;
mod types;
mod util;
use types::*;
use util::*;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG);
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string()).unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Ultimate Tapan Kaikki - Level Editor", 640, 480)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let texture_creator = canvas.texture_creator();
    let texture_floor = texture_creator.load_texture("./assets/FLOOR1.PNG").unwrap();
    let texture_walls = texture_creator.load_texture("./assets/WALLS1.PNG").unwrap();
    let mut texture_type_selected = TextureType::FLOOR;

    let font = ttf_context
        .load_font("./assets/TheJewishBitmap.ttf", 128)
        .unwrap();
    let p1_text_texture = render::get_font_texture(&texture_creator, &font, "P1");
    let p2_text_texture = render::get_font_texture(&texture_creator, &font, "P2");

    let mut level = Level::get_default_level();
    let mut tile_select_mode = false;
    let mut selected_tile_id = 0;
    let mut mouse: (u32, u32) = (0, 0);

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
                    level.serialize("./TEST.LEV").unwrap();
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
                Event::MouseMotion { x, y, .. } => {
                    mouse.0 = x as u32;
                    mouse.1 = y as u32;
                }
                Event::MouseButtonDown {
                    mouse_btn: MouseButton::Left,
                    ..
                } => {
                    if tile_select_mode {
                        selected_tile_id = get_tile_id_from_coordinate(mouse.0, mouse.1);
                    } else {
                        let pointed_tile = get_tile_id_from_coordinate(mouse.0, mouse.1);
                        level.put_tile_to_level(
                            pointed_tile,
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
            render::render_level(&mut canvas, &level, &texture_floor, &texture_walls);
            render::render_text_texture(
                &mut canvas,
                &p1_text_texture,
                level.p1_position.0 * RENDER_SIZE,
                level.p1_position.1 * RENDER_SIZE,
            );
            render::render_text_texture(
                &mut canvas,
                &p2_text_texture,
                level.p2_position.0 * RENDER_SIZE,
                level.p2_position.1 * RENDER_SIZE,
            );
        }

        let highlighted_id = get_tile_id_from_coordinate(mouse.0, mouse.1);

        render::highlight_selected_tile(&mut canvas, highlighted_id);
        canvas.present();

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
