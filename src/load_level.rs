use crate::fn2::create_text_texture;
use crate::get_bottom_text_position;
use crate::get_textures;
use crate::util::TITLE_POSITION;
use std::fs;
extern crate sdl2;

use crate::render;
use crate::types::*;
use crate::Context;
use crate::NextMode::*;
use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::Texture;

struct LoadFile<'a> {
    filename: String,
    texture: Texture<'a>,
}

pub fn exec(context: &mut Context) -> NextMode {
    let load_level_text_texture = create_text_texture(
        &mut context.canvas,
        &context.texture_creator,
        &context.font,
        "LOAD LEVEL:",
    );
    let bottom_instruction_text = create_text_texture(
        &mut context.canvas,
        &context.texture_creator,
        &context.font,
        "ENTER to select or ESC to exit",
    );
    let files: Vec<LoadFile> = fs::read_dir("./")
        .unwrap()
        .filter_map(|read_dir_result| {
            let filename = read_dir_result.unwrap().path().display().to_string();
            if filename.to_uppercase().ends_with(".LEV") {
                Some(filename)
            } else {
                None
            }
        })
        .map(|ref filename| LoadFile {
            filename: filename.to_string(),
            texture: create_text_texture(
                &mut context.canvas,
                context.texture_creator,
                &context.font,
                &filename.clone().to_lowercase(),
            ),
        })
        .collect();
    let mut selected = 0usize;

    let mut event_pump = context.sdl.event_pump().unwrap();
    loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => return Editor,
                Event::Window { win_event, .. } => {
                    if let WindowEvent::Resized(w, h) = win_event {
                        context.graphics.resolution_x = w as u32;
                        context.graphics.resolution_y = h as u32;
                        context.textures = get_textures(
                            &mut context.canvas,
                            context.texture_creator,
                            &context.font,
                        );
                        return Editor;
                    }
                }
                Event::KeyDown { keycode, .. } => match keycode.unwrap() {
                    Keycode::Down => {
                        if selected < files.len() - 1 {
                            selected = selected + 1;
                        }
                    }
                    Keycode::Up => {
                        if selected > 0 {
                            selected = selected - 1;
                        }
                    }
                    Keycode::Return | Keycode::KpEnter => {
                        if files.len() > 0 {
                            context
                                .level
                                .deserialize(&files[selected].filename)
                                .unwrap();
                            let level_name = files[selected]
                                .filename
                                .strip_prefix("./")
                                .unwrap()
                                .to_string();
                            context.textures.saved_level_name = Some(create_text_texture(
                                &mut context.canvas,
                                &context.texture_creator,
                                &context.font,
                                &level_name.clone().to_lowercase(),
                            ));
                            context.level_save_name =
                                level_name.strip_suffix(".LEV").unwrap().to_string();
                        }
                        return Editor;
                    }
                    _ => {}
                },
                _ => {}
            }
        }

        context.canvas.set_draw_color(Color::from((0, 0, 0)));
        context.canvas.clear();
        let text_position = (40, 60);
        let render_size = context.graphics.get_render_size();
        render::render_text_texture_coordinates(
            &mut context.canvas,
            &load_level_text_texture,
            TITLE_POSITION,
            render_size,
            None,
        );
        let line_spacing = 20;
        for x in 0..files.len() {
            if selected == x {
                render::render_text_texture(
                    &mut context.canvas,
                    &context.textures.selected_icon,
                    text_position.0 - 20,
                    text_position.1 + 3 + x as u32 * line_spacing,
                    render_size,
                    None,
                );
            }
            render::render_text_texture(
                &mut context.canvas,
                &files[x].texture,
                text_position.0,
                text_position.1 + line_spacing * x as u32,
                render_size,
                None,
            );
        }
        render::render_text_texture_coordinates(
            &mut context.canvas,
            &bottom_instruction_text,
            get_bottom_text_position(context.graphics.resolution_y),
            render_size,
            None,
        );
        render::render_and_wait(&mut context.canvas);
    }
}
