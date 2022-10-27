extern crate sdl2;

use crate::context_util::resize;
use crate::NextMode;
use crate::NextMode::*;
use crate::{Context, Renderer};
use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::render::Texture;

pub fn exec<'a>(renderer: &'a Renderer, context: &mut Context<'a>) -> NextMode {
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
        " ",
        "- WINDOW -",
        "+/- adjust rendering size",
    ];
    let line_textures: Vec<Texture> = lines
        .iter()
        .map(|text| renderer.create_text_texture(&context.font, text))
        .collect();

    let mut event_pump = context.sdl.event_pump().unwrap();
    loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => return Editor,
                Event::KeyDown { .. } => {
                    return Editor;
                }
                Event::Window { win_event, .. } => {
                    if resize(renderer, context, win_event) {
                        return Editor;
                    }
                }
                _ => {}
            }
        }
        renderer.clear_screen(Color::from((0, 0, 0)));
        let mut position = 6;
        for line_texture in &line_textures {
            renderer.render_text_texture(
                &line_texture,
                10,
                position,
                context.graphics.get_render_size(),
                None,
            );
            position += 22;
        }
        renderer.render_and_wait();
    }
}
