extern crate sdl2;

use crate::render;
use crate::types::*;
use crate::Context;
use crate::NextMode::*;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::Texture;

enum Value {
    String(String),
    Seconds(u32),
    Number(u32),
}

struct ConfigOption<'a> {
    texture: &'a Texture<'a>,
    value: Value,
}

fn load_text<'a>(context: &mut Context<'a>, text: &str) -> Texture<'a> {
    render::get_font_texture(&context.texture_creator, &context.font, text)
}

fn load_value_text<'a>(context: &mut Context<'a>, value: &Value) -> Texture<'a> {
    let string = match value {
        Value::Number(number) => number.to_string(),
        Value::Seconds(seconds) => format!("{} SECONDS", seconds),
        Value::String(string) => if string.len() == 0 { " " } else { &string }.to_string(),
    };
    render::get_font_texture(&context.texture_creator, &context.font, &string)
}

pub fn exec(context: &mut Context) -> NextMode {
    let options = [
        ConfigOption {
            texture: &load_text(context, "LEVEL COMMENT:"),
            value: Value::String("".to_string()),
        },
        ConfigOption {
            texture: &load_text(context, "TIME LIMIT:"),
            value: Value::Seconds(60),
        },
        ConfigOption {
            texture: &load_text(context, "PISTOL BOYS:"),
            value: Value::Number(0),
        },
        ConfigOption {
            texture: &load_text(context, "SHOTGUN MANIACS:"),
            value: Value::Number(0),
        },
        ConfigOption {
            texture: &load_text(context, "UZI REBELS:"),
            value: Value::Number(0),
        },
        ConfigOption {
            texture: &load_text(context, "COMMANDOS:"),
            value: Value::Number(0),
        },
        ConfigOption {
            texture: &load_text(context, "GRANADE MOFOS:"),
            value: Value::Number(0),
        },
        ConfigOption {
            texture: &load_text(context, "CIVILIANS:"),
            value: Value::Number(0),
        },
        ConfigOption {
            texture: &load_text(context, "PUNISHERS:"),
            value: Value::Number(0),
        },
        ConfigOption {
            texture: &load_text(context, "FLAMERS:"),
            value: Value::Number(0),
        },
    ];

    let mut event_pump = context.sdl.event_pump().unwrap();
    loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => return Editor,
                _ => {}
            }
        }

        context.canvas.set_draw_color(Color::from((0, 0, 0)));
        context.canvas.clear();
        let mut option_position = (40, 20);
        let mut value_position = (300, option_position.1);
        for option in &options {
            render::render_text_texture(
                &mut context.canvas,
                option.texture,
                option_position.0,
                option_position.1,
                None,
            );
            let value_texture = &load_value_text(context, &option.value);
            render::render_text_texture(
                &mut context.canvas,
                value_texture,
                value_position.0,
                value_position.1,
                None,
            );
            option_position.1 += 30;
            value_position.1 = option_position.1;
        }
        render::render_and_wait(&mut context.canvas);
    }
}
