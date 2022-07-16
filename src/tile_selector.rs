extern crate sdl2;

use crate::render;
use crate::types::*;
use crate::util::*;
use crate::Context;
use crate::NextMode::*;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

pub fn exec(context: &mut Context) -> NextMode {
    let mut event_pump = context.sdl.event_pump().unwrap();
    loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => return Quit,
                Event::KeyDown { keycode, .. } => match keycode.unwrap() {
                    Keycode::Space => {
                        return Editor;
                    }
                    Keycode::PageDown | Keycode::PageUp => {
                        context.texture_type_selected =
                            if context.texture_type_selected == TextureType::FLOOR {
                                TextureType::WALLS
                            } else {
                                TextureType::FLOOR
                            }
                    }
                    _ => {}
                },
                Event::MouseMotion { x, y, .. } => {
                    context.mouse.0 = x as u32;
                    context.mouse.1 = y as u32;
                }
                Event::MouseButtonDown {
                    mouse_btn: MouseButton::Left,
                    ..
                } => {
                    context.selected_tile_id =
                        get_tile_id_from_coordinate(context.mouse.0, context.mouse.1);
                }
                _ => {}
            }
        }

        context.canvas.set_draw_color(Color::from((0, 0, 0)));
        context.canvas.clear();
        let dst = Rect::new(0, 0, 640, 400);
        let texture_selected = match context.texture_type_selected {
            TextureType::FLOOR => &context.texture_floor,
            TextureType::WALLS => &context.texture_walls,
        };
        context.canvas.copy(texture_selected, None, dst).unwrap();
        let highlighted_id = get_tile_id_from_coordinate(context.mouse.0, context.mouse.1);

        render::highlight_selected_tile(&mut context.canvas, highlighted_id);
        render::render_and_wait(&mut context.canvas);
    }
}
