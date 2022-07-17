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
    let p1_set_text_texture = render::get_font_texture(
        &context.texture_creator,
        &context.font,
        "PLACE PL1 START POINT",
    );
    let p2_set_text_texture = render::get_font_texture(
        &context.texture_creator,
        &context.font,
        "PLACE PL2 START POINT",
    );
    let help_text_texture =
        render::get_font_texture(&context.texture_creator, &context.font, "F1 FOR HELP");
    let mut set_position: u8 = 0;
    let mut mouse_left_click = false;

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
                    Keycode::Num1 => {
                        set_position = 1;
                    }
                    Keycode::Num2 => {
                        set_position = 2;
                    }
                    _ => {}
                },
                Event::MouseMotion { x, y, .. } => {
                    if x >= 0 && y >= 0 {
                        context.mouse.0 = x as u32;
                        context.mouse.1 = y as u32;
                        if mouse_left_click {
                            handle_mouse_down(context, &mut set_position);
                        }
                    }
                }
                Event::MouseButtonDown {
                    mouse_btn: MouseButton::Left,
                    ..
                } => {
                    mouse_left_click = true;
                    handle_mouse_down(context, &mut set_position);
                }
                Event::MouseButtonUp {
                    mouse_btn: MouseButton::Left,
                    ..
                } => {
                    mouse_left_click = false;
                }
                _ => {}
            }
        }

        render::render_level(&mut context.canvas, &context.level, &context.textures);
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
        let text_position = (8, 8);
        if set_position == 1 {
            render::render_text_texture_coordinates(
                &mut context.canvas,
                &p1_set_text_texture,
                text_position,
            );
        } else if set_position == 2 {
            render::render_text_texture_coordinates(
                &mut context.canvas,
                &p2_set_text_texture,
                text_position,
            );
        } else {
            render::render_text_texture_coordinates(
                &mut context.canvas,
                &help_text_texture,
                text_position,
            );
        };

        let highlighted_id = get_tile_id_from_coordinate(context.mouse.0, context.mouse.1);

        render::highlight_selected_tile(&mut context.canvas, highlighted_id);
        render::render_and_wait(&mut context.canvas);
    }
}

fn handle_mouse_down(context: &mut Context, set_position: &mut u8) {
    if *set_position > 0 {
        let position = if *set_position == 1 {
            &mut context.level.p1_position
        } else {
            &mut context.level.p2_position
        };
        *position = get_logical_coordinates(context.mouse.0, context.mouse.1);
        *set_position = 0;
    } else {
        let pointed_tile = get_tile_id_from_coordinate(context.mouse.0, context.mouse.1);
        context.level.put_tile_to_level(
            pointed_tile,
            context.selected_tile_id,
            &context.texture_type_selected,
        );
    }
}
