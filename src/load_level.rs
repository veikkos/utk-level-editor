use std::fs;
extern crate sdl2;

use crate::render;
use crate::types::*;
use crate::Context;
use crate::NextMode::*;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::Texture;

struct LoadFile<'a> {
    filename: String,
    texture: Texture<'a>,
}

pub fn exec(context: &mut Context) -> NextMode {
    let load_level_text_texture =
        render::get_font_texture(&context.texture_creator, &context.font, "LOAD LEVEL:");
    let bottom_instruction_text = render::get_font_texture(
        &context.texture_creator,
        &context.font,
        "ENTER TO SELECT OR ESC TO EXIT",
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
            texture: render::get_font_texture(context.texture_creator, &context.font, &filename),
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
                        context
                            .level
                            .deserialize(&files[selected].filename)
                            .unwrap();
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
        render::render_text_texture_coordinates(
            &mut context.canvas,
            &load_level_text_texture,
            (text_position.0, 20),
            None,
        );
        let line_spacing = 30;
        for x in 0..files.len() {
            if selected == x {
                render::render_text_texture(
                    &mut context.canvas,
                    &context.textures.selected_icon,
                    text_position.0 - 20,
                    text_position.1 + 3 + x as u32 * line_spacing,
                    None,
                );
            }
            render::render_text_texture(
                &mut context.canvas,
                &files[x].texture,
                text_position.0,
                text_position.1 + line_spacing * x as u32,
                None,
            );
        }
        render::render_text_texture_coordinates(
            &mut context.canvas,
            &bottom_instruction_text,
            (text_position.0, 425),
            None,
        );
        render::render_and_wait(&mut context.canvas);
    }
}
