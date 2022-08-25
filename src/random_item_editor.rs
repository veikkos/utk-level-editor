extern crate sdl2;

use crate::create_text_texture;
use crate::get_textures;
use crate::level::Level;
use crate::render;
use crate::types::*;
use crate::util::{get_bottom_text_position, TITLE_POSITION};
use crate::Context;
use crate::NextMode::*;
use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::Texture;

fn load_text<'a>(context: &mut Context<'a>, text: &str) -> Texture<'a> {
    create_text_texture(
        &mut context.canvas,
        &context.texture_creator,
        &context.font,
        text,
    )
}

fn get_value(level: &Level, game_type: &GameType, index: usize) -> u32 {
    let crates = match game_type {
        GameType::Normal => &level.crates.random.normal,
        GameType::Deathmatch => &level.crates.random.deathmatch,
    };
    if index < crates.weapons.len() {
        crates.weapons[index]
    } else {
        let index = index - crates.weapons.len();
        if index < crates.bullets.len() {
            crates.bullets[index]
        } else {
            crates.energy
        }
    }
}

fn set_value(level: &mut Level, game_type: &GameType, index: usize, value: u32) {
    let crates = match game_type {
        GameType::Normal => &mut level.crates.random.normal,
        GameType::Deathmatch => &mut level.crates.random.deathmatch,
    };
    if index < crates.weapons.len() {
        crates.weapons[index] = value;
    } else {
        let index = index - crates.weapons.len();
        if index < crates.bullets.len() {
            crates.bullets[index] = value;
        } else {
            crates.energy = value;
        }
    }
}

pub fn exec(context: &mut Context, game_type: GameType) -> NextMode {
    let normal_game_instruction_text = &load_text(context, "NORMAL GAME CRATES");
    let deatchmatch_instruction_text = &load_text(context, "DEATHMATCH CRATES");
    let esc_instruction_text = &load_text(context, "press ESC to exit");
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
                        if selected < context.textures.crates.len() - 1 {
                            selected = selected + 1;
                        }
                    }
                    Keycode::Up => {
                        if selected > 0 {
                            selected = selected - 1;
                        }
                    }
                    Keycode::Right => {
                        let value = get_value(&context.level, &game_type, selected);
                        set_value(&mut context.level, &game_type, selected, value + 1);
                    }
                    Keycode::Left => {
                        let value = get_value(&context.level, &game_type, selected);
                        if value > 0 {
                            set_value(&mut context.level, &game_type, selected, value - 1);
                        }
                    }
                    _ => (),
                },
                _ => {}
            }
        }

        context.canvas.set_draw_color(Color::from((0, 0, 0)));
        context.canvas.clear();
        let render_size = context.graphics.get_render_size();

        render::render_text_texture_coordinates(
            &mut context.canvas,
            match game_type {
                GameType::Normal => &normal_game_instruction_text,
                GameType::Deathmatch => &deatchmatch_instruction_text,
            },
            TITLE_POSITION,
            render_size,
            None,
        );

        let y = 50;
        let mut option_position = (40, y);
        let mut value_position = (280, option_position.1);
        for x in 0..context.textures.crates.len() {
            let option = &context.textures.crates[x];
            if selected == x {
                render::render_text_texture(
                    &mut context.canvas,
                    &context.textures.selected_icon,
                    option_position.0 - 20,
                    option_position.1 + 3,
                    render_size,
                    None,
                );
            }
            render::render_text_texture(
                &mut context.canvas,
                &option,
                option_position.0,
                option_position.1,
                render_size,
                None,
            );
            let value_texture = create_text_texture(
                &mut context.canvas,
                &context.texture_creator,
                &context.font,
                &get_value(&context.level, &game_type, x).to_string(),
            );
            render::render_text_texture(
                &mut context.canvas,
                &value_texture,
                value_position.0,
                value_position.1,
                render_size,
                None,
            );
            if x == 10 {
                option_position.1 = y;
                value_position.1 = option_position.1;
                option_position.0 = 330;
                value_position.0 = option_position.0 + 250;
            } else {
                option_position.1 += 20;
                value_position.1 = option_position.1;
            }
        }
        render::render_text_texture_coordinates(
            &mut context.canvas,
            esc_instruction_text,
            get_bottom_text_position(context.graphics.resolution_y),
            render_size,
            None,
        );
        render::render_and_wait(&mut context.canvas);
    }
}
