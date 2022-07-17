extern crate sdl2;

use crate::render;
use crate::Context;
use crate::NextMode;
use crate::NextMode::*;
use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::render::Texture;

pub fn exec(context: &mut Context) -> NextMode {
    let lines = [
        "ESC - QUIT",
        "F1   - THIS HELP",
        "F2   - SAVE LEVEL",
        "1/2  - PLACE PL1/PL2 START",
        " ",
        "SPACE - SELECTION/EDITING MODE",
    ];
    let line_textures: Vec<Texture> = lines
        .iter()
        .map(|text| render::get_font_texture(&context.texture_creator, &context.font, text))
        .collect();

    let mut event_pump = context.sdl.event_pump().unwrap();
    loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => return Quit,
                Event::KeyDown { .. } => {
                    return Editor;
                }
                _ => {}
            }
        }
        context.canvas.set_draw_color(Color::from((0, 0, 0)));
        context.canvas.clear();
        let mut position = 10;
        for line_texture in &line_textures {
            render::render_text_texture(&mut context.canvas, &line_texture, 10, position);
            position += 30;
        }
        render::render_and_wait(&mut context.canvas);
    }
}
