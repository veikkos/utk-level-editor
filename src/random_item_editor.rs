extern crate sdl2;

use crate::render;
use crate::types::*;
use crate::Context;
use crate::NextMode::*;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::Texture;

fn load_text<'a>(context: &Context<'a>, text: &str) -> Texture<'a> {
    render::get_font_texture(&context.texture_creator, &context.font, text)
}

fn get_value<'a>(context: &'a Context<'_>, index: usize) -> &'a u32 {
    if index < context.level.crates.normal.weapons.len() {
        &context.level.crates.normal.weapons[index]
    } else {
        let index = index - context.level.crates.normal.weapons.len();
        if index < context.level.crates.normal.bullets.len() {
            &context.level.crates.normal.bullets[index]
        } else {
            &context.level.crates.normal.energy
        }
    }
}

fn set_value<'a>(context: &'a mut Context<'_>, index: usize, value: u32) {
    if index < context.level.crates.normal.weapons.len() {
        context.level.crates.normal.weapons[index] = value;
    } else {
        let index = index - context.level.crates.normal.weapons.len();
        if index < context.level.crates.normal.bullets.len() {
            context.level.crates.normal.bullets[index] = value;
        } else {
            context.level.crates.normal.energy = value;
        }
    }
}

pub fn exec(context: &mut Context) -> NextMode {
    let options = [
        "PISTOL",
        "SHOTGUN",
        "UZI",
        "AUTO RIFLE",
        "GRENADE LAUNCHER",
        "AUTO GRENADIER",
        "HEAVY LAUNCHER",
        "AUTO SHOTGUN",
        "C4-ACTIVATOR",
        "FLAME THROWER",
        "MINE DROPPER",
        "9MM BULLETS (50)",
        "12MM BULLETS (50)",
        "SHOTGUN SHELLS (20)",
        "LIGHT GRENADES (15)",
        "MEDIUM GRENADES (10)",
        "HEAVY GRENADES (5)",
        "C4-EXPLOSIVES (5)",
        "GAS (50)",
        "MINES (5)",
        "ENERGY",
    ]
    .map(|name| render::get_font_texture(&context.texture_creator, &context.font, name));
    let esc_instruction_text = &load_text(context, "PRESS ESC TO EXIT");
    let mut selected = 0;

    let mut event_pump = context.sdl.event_pump().unwrap();
    loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    context.sdl.video().unwrap().text_input().stop();
                    return Editor;
                }
                Event::KeyDown { keycode, .. } => match keycode.unwrap() {
                    Keycode::Down => {
                        if selected < options.len() - 1 {
                            selected = selected + 1;
                        }
                    }
                    Keycode::Up => {
                        if selected > 0 {
                            selected = selected - 1;
                        }
                    }
                    Keycode::Right => {
                        let value = get_value(context, selected);
                        set_value(context, selected, value + 1);
                    }
                    Keycode::Left => {
                        let value = get_value(context, selected);
                        if *value > 0 {
                            set_value(context, selected, value - 1);
                        }
                    }
                    _ => (),
                },
                _ => {}
            }
        }

        context.canvas.set_draw_color(Color::from((0, 0, 0)));
        context.canvas.clear();
        let mut option_position = (40, 20);
        let mut value_position = (280, option_position.1);
        for x in 0..options.len() {
            let option = &options[x];
            if selected == x {
                render::render_text_texture(
                    &mut context.canvas,
                    &context.textures.selected_icon,
                    option_position.0 - 20,
                    option_position.1 + 3,
                    None,
                );
            }
            render::render_text_texture(
                &mut context.canvas,
                &option,
                option_position.0,
                option_position.1,
                None,
            );
            let value_texture = render::get_font_texture(
                &context.texture_creator,
                &context.font,
                &get_value(&context, x).to_string(),
            );
            render::render_text_texture(
                &mut context.canvas,
                &value_texture,
                value_position.0,
                value_position.1,
                None,
            );
            if x == 10 {
                option_position.1 = 20;
                value_position.1 = option_position.1;
                option_position.0 = 330;
                value_position.0 = option_position.0 + 250;
            } else {
                option_position.1 += 30;
                value_position.1 = option_position.1;
            }
        }
        render::render_text_texture(&mut context.canvas, esc_instruction_text, 40, 425, None);
        render::render_and_wait(&mut context.canvas);
    }
}
