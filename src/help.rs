extern crate sdl2;

use crate::fn2::create_text_texture;
use crate::render;
use crate::Context;
use crate::NextMode;
use crate::NextMode::*;
use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::render::Texture;

pub fn exec(context: &mut Context) -> NextMode {
    let lines = [
        "ESC - quit",
        "F1   - this help",
        "F2   - save level",
        "F3   - load level",
        "F4   - create new level",
        "F6   - enable/disable automatic shadows",
        "F7   - edit general level variables",
        "F8/F9 - edit random crates for normal/dm games",
        " ",
        "- EDITOR -",
        "Q/W  - place/delete spotlights",
        "A/S  - place/delete steams",
        "Z/X/C - place/delete crates",
        "1/2  - place pl1/pl2 start",
        "SPACE - tile selection/editing mode",
        "ARROW KEYS - move viewport",
    ];
    let line_textures: Vec<Texture> = lines
        .iter()
        .map(|text| {
            create_text_texture(
                &mut context.canvas,
                &context.texture_creator,
                &context.font,
                text,
            )
        })
        .collect();

    let mut event_pump = context.sdl.event_pump().unwrap();
    loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => return Editor,
                Event::KeyDown { .. } => {
                    return Editor;
                }
                _ => {}
            }
        }
        context.canvas.set_draw_color(Color::from((0, 0, 0)));
        context.canvas.clear();
        let mut position = 6;
        for line_texture in &line_textures {
            render::render_text_texture(
                &mut context.canvas,
                &line_texture,
                10,
                position,
                context.graphics.get_render_size(),
                None,
            );
            position += 22;
        }
        render::render_and_wait(&mut context.canvas);
    }
}
