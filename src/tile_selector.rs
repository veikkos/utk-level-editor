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

pub fn exec(context: &mut Context) -> NextMode {
    let floor_blocks_text_texture = render::get_font_texture(
        &context.texture_creator,
        &context.font,
        "FLOOR BLOCKS [PAGEGUP/DOWN]",
    );
    let wall_blocks_text_texture = render::get_font_texture(
        &context.texture_creator,
        &context.font,
        "WALL BLOCKS [PAGEGUP/DOWN]",
    );
    let shadow_blocks_text_texture = render::get_font_texture(
        &context.texture_creator,
        &context.font,
        "SHADOW [PAGEGUP/DOWN]",
    );
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
                    Keycode::PageDown => {
                        context.texture_type_selected =
                            if context.texture_type_selected == TextureType::FLOOR {
                                TextureType::WALLS
                            } else if context.texture_type_selected == TextureType::WALLS {
                                TextureType::SHADOW
                            } else {
                                TextureType::FLOOR
                            }
                    }
                    Keycode::PageUp => {
                        context.texture_type_selected =
                            if context.texture_type_selected == TextureType::FLOOR {
                                TextureType::SHADOW
                            } else if context.texture_type_selected == TextureType::SHADOW {
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
        let texture_selected = match context.texture_type_selected {
            TextureType::FLOOR => &context.textures.floor,
            TextureType::WALLS => &context.textures.walls,
            TextureType::SHADOW => &context.textures.shadows,
        };
        let dst = render::get_texture_rect(texture_selected);
        context.canvas.set_draw_color(Color::from((200, 200, 200)));
        context.canvas.fill_rect(dst).unwrap();
        context.canvas.copy(texture_selected, None, dst).unwrap();
        let highlighted_id = get_tile_id_from_coordinate(context.mouse.0, context.mouse.1);

        render::highlight_selected_tile(&mut context.canvas, highlighted_id);
        let text_position = (8, 454);
        let active_text = match context.texture_type_selected {
            TextureType::FLOOR => &floor_blocks_text_texture,
            TextureType::WALLS => &wall_blocks_text_texture,
            TextureType::SHADOW => &shadow_blocks_text_texture,
        };
        render::render_text_texture_coordinates(&mut context.canvas, active_text, text_position);
        render::render_and_wait(&mut context.canvas);
    }
}
