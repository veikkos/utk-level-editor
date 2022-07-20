extern crate sdl2;

use crate::render;
use crate::types::*;
use crate::Context;
use crate::NextMode::*;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::Texture;

enum Value<'a> {
    StrRef(&'a str),
    Seconds(&'a u32),
    Number(&'a u32),
}

struct ConfigOption<'a> {
    texture: &'a Texture<'a>,
    value: Value<'a>,
}

fn load_text<'a>(context: &Context<'a>, text: &str) -> Texture<'a> {
    render::get_font_texture(&context.texture_creator, &context.font, text)
}

fn load_value_text<'a>(context: &Context<'a>, value: &Value<'a>) -> Texture<'a> {
    let string = match value {
        Value::Number(number) => number.to_string(),
        Value::Seconds(seconds) => format!("{} SECONDS", seconds),
        Value::StrRef(str_ref) => str_ref.to_string(),
    };
    render::get_font_texture(&context.texture_creator, &context.font, &string)
}

pub fn exec(context: &mut Context) -> NextMode {
    let options = [
        ConfigOption {
            texture: &load_text(context, "LEVEL COMMENT:"),
            value: Value::StrRef(&context.level.general_info.comment),
        },
        ConfigOption {
            texture: &load_text(context, "TIME LIMIT:"),
            value: Value::Seconds(&context.level.general_info.time_limit),
        },
        ConfigOption {
            texture: &load_text(context, "PISTOL BOYS:"),
            value: Value::Number(&context.level.general_info.enemy_table[0]),
        },
        ConfigOption {
            texture: &load_text(context, "SHOTGUN MANIACS:"),
            value: Value::Number(&context.level.general_info.enemy_table[1]),
        },
        ConfigOption {
            texture: &load_text(context, "UZI REBELS:"),
            value: Value::Number(&context.level.general_info.enemy_table[2]),
        },
        ConfigOption {
            texture: &load_text(context, "COMMANDOS:"),
            value: Value::Number(&context.level.general_info.enemy_table[3]),
        },
        ConfigOption {
            texture: &load_text(context, "GRANADE MOFOS:"),
            value: Value::Number(&context.level.general_info.enemy_table[4]),
        },
        ConfigOption {
            texture: &load_text(context, "CIVILIANS:"),
            value: Value::Number(&context.level.general_info.enemy_table[5]),
        },
        ConfigOption {
            texture: &load_text(context, "PUNISHERS:"),
            value: Value::Number(&context.level.general_info.enemy_table[6]),
        },
        ConfigOption {
            texture: &load_text(context, "FLAMERS:"),
            value: Value::Number(&context.level.general_info.enemy_table[7]),
        },
    ];
    let esc_instruction_text = &load_text(context, "PRESS ESC TO EXIT");

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
        render::render_text_texture(
            &mut context.canvas,
            esc_instruction_text,
            option_position.0,
            425,
            None,
        );
        render::render_and_wait(&mut context.canvas);
    }
}
