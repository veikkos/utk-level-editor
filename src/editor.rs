extern crate sdl2;

use crate::render;
use crate::types::*;
use crate::util::*;
use crate::Context;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::time::Duration;

pub fn editor_mode(context: &mut Context) {
    let p1_text_texture = render::get_font_texture(&context.texture_creator, &context.font, "P1");
    let p2_text_texture = render::get_font_texture(&context.texture_creator, &context.font, "P2");
    let mut event_pump = context.sdl.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown { keycode, .. } => match keycode.unwrap() {
                    Keycode::Space => {
                        context.tile_select_mode = !context.tile_select_mode;
                    }
                    Keycode::F2 => {
                        context.level.serialize("./TEST.LEV").unwrap();
                    }
                    Keycode::PageDown | Keycode::PageUp => {
                        if context.tile_select_mode {
                            context.texture_type_selected =
                                if context.texture_type_selected == TextureType::FLOOR {
                                    TextureType::WALLS
                                } else {
                                    TextureType::FLOOR
                                }
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
                    if context.tile_select_mode {
                        context.selected_tile_id =
                            get_tile_id_from_coordinate(context.mouse.0, context.mouse.1);
                    } else {
                        let pointed_tile =
                            get_tile_id_from_coordinate(context.mouse.0, context.mouse.1);
                        context.level.put_tile_to_level(
                            pointed_tile,
                            context.selected_tile_id,
                            &context.texture_type_selected,
                        );
                    }
                }
                _ => {}
            }
        }

        if context.tile_select_mode {
            context.canvas.set_draw_color(Color::from((0, 0, 0)));
            context.canvas.clear();
            let dst = Rect::new(0, 0, 640, 400);
            let texture_selected = match context.texture_type_selected {
                TextureType::FLOOR => &context.texture_floor,
                TextureType::WALLS => &context.texture_walls,
            };
            context.canvas.copy(texture_selected, None, dst).unwrap();
        } else {
            render::render_level(
                &mut context.canvas,
                &context.level,
                &context.texture_floor,
                &context.texture_walls,
            );
            render::render_text_texture(
                &mut context.canvas,
                &p1_text_texture,
                context.level.p1_position.0 * RENDER_SIZE,
                context.level.p1_position.1 * RENDER_SIZE,
            );
            render::render_text_texture(
                &mut context.canvas,
                &p2_text_texture,
                context.level.p2_position.0 * RENDER_SIZE,
                context.level.p2_position.1 * RENDER_SIZE,
            );
        }

        let highlighted_id = get_tile_id_from_coordinate(context.mouse.0, context.mouse.1);

        render::highlight_selected_tile(&mut context.canvas, highlighted_id);
        context.canvas.present();

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
