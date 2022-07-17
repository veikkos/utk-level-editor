extern crate sdl2;

use crate::render;
use crate::util::*;
use crate::Context;
use crate::NextMode;
use crate::NextMode::*;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;

pub fn exec(context: &mut Context) -> NextMode {
    let p1_text_texture = render::get_font_texture(&context.texture_creator, &context.font, "PL1");
    let p2_text_texture = render::get_font_texture(&context.texture_creator, &context.font, "PL2");

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
                        return TileSelect;
                    }
                    Keycode::F1 => {
                        return Help;
                    }
                    Keycode::F2 => {
                        context.level.serialize("./TEST.LEV").unwrap();
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
                    let pointed_tile =
                        get_tile_id_from_coordinate(context.mouse.0, context.mouse.1);
                    context.level.put_tile_to_level(
                        pointed_tile,
                        context.selected_tile_id,
                        &context.texture_type_selected,
                    );
                }
                _ => {}
            }
        }

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

        let highlighted_id = get_tile_id_from_coordinate(context.mouse.0, context.mouse.1);

        render::highlight_selected_tile(&mut context.canvas, highlighted_id);
        render::render_and_wait(&mut context.canvas);
    }
}
