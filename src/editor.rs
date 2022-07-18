extern crate sdl2;

use crate::render;
use crate::util::*;
use crate::Context;
use crate::Level;
use crate::NextMode;
use crate::NextMode::*;
use crate::TextureType;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;

#[derive(PartialEq)]
enum PromptType {
    None,
    NewLevel,
    Quit,
}

pub fn exec(context: &mut Context) -> NextMode {
    let p1_text_texture = render::get_font_texture(&context.texture_creator, &context.font, "PL1");
    let p2_text_texture = render::get_font_texture(&context.texture_creator, &context.font, "PL2");
    let p1_set_text_texture = render::get_font_texture(
        &context.texture_creator,
        &context.font,
        "PLACE PL1 START POINT",
    );
    let p2_set_text_texture = render::get_font_texture(
        &context.texture_creator,
        &context.font,
        "PLACE PL2 START POINT",
    );
    let help_text_texture =
        render::get_font_texture(&context.texture_creator, &context.font, "F1 FOR HELP");
    let create_new_level_text_texture =
        render::get_font_texture(&context.texture_creator, &context.font, "CREATE NEW LEVEL?");
    let wanna_quit_text_texture = render::get_font_texture(
        &context.texture_creator,
        &context.font,
        "REALLY WANNA QUIT?",
    );
    let press_y_text_texture = render::get_font_texture(
        &context.texture_creator,
        &context.font,
        "PRESS Y TO CONFIRM",
    );
    let mut set_position: u8 = 0;
    let mut mouse_left_click = false;
    let mut mouse_right_click = false;
    let mut prompt: PromptType = PromptType::None;

    let mut event_pump = context.sdl.event_pump().unwrap();
    loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    prompt = if prompt != PromptType::None {
                        PromptType::None
                    } else {
                        PromptType::Quit
                    };
                }
                Event::KeyDown { keycode, .. } => match keycode.unwrap() {
                    Keycode::Space => {
                        return TileSelect;
                    }
                    Keycode::F1 => {
                        return Help;
                    }
                    Keycode::F2 => {
                        context.level.serialize("./TEST.LEV").unwrap();
                        prompt = PromptType::None;
                    }
                    Keycode::F3 => {
                        context.level.deserialize("./TEST.LEV").unwrap();
                        prompt = PromptType::None;
                    }
                    Keycode::F4 => {
                        prompt = PromptType::NewLevel;
                    }
                    Keycode::Num1 => {
                        set_position = 1;
                        prompt = PromptType::None;
                    }
                    Keycode::Num2 => {
                        set_position = 2;
                        prompt = PromptType::None;
                    }
                    Keycode::Y => {
                        match prompt {
                            PromptType::NewLevel => context.level = Level::get_default_level(),
                            PromptType::Quit => return Quit,
                            PromptType::None => {}
                        }
                        prompt = PromptType::None;
                    }
                    Keycode::Up => {
                        if context.level.scroll.1 > 0 {
                            context.level.scroll.1 = context.level.scroll.1 - 1;
                        }
                    }
                    Keycode::Down => {
                        if context.level.scroll.1 + TILES_Y_PER_SCREEN
                            < (context.level.tiles.len()) as u32
                        {
                            context.level.scroll.1 = context.level.scroll.1 + 1;
                        }
                    }
                    Keycode::Left => {
                        if context.level.scroll.0 > 0 {
                            context.level.scroll.0 = context.level.scroll.0 - 1;
                        }
                    }
                    Keycode::Right => {
                        if context.level.scroll.0 + TILES_X_PER_SCREEN
                            < (context.level.tiles[0].len()) as u32
                        {
                            context.level.scroll.0 = context.level.scroll.0 + 1;
                        }
                    }
                    _ => prompt = PromptType::None,
                },
                Event::MouseMotion { x, y, .. } => {
                    if x >= 0 && y >= 0 {
                        context.mouse.0 = x as u32;
                        context.mouse.1 = y as u32;
                        if mouse_left_click {
                            handle_mouse_left_down(context, &mut set_position);
                        }
                        if mouse_right_click {
                            handle_mouse_right_down(context);
                        }
                    }
                }
                Event::MouseButtonDown {
                    mouse_btn: MouseButton::Left,
                    ..
                } => {
                    mouse_left_click = true;
                    handle_mouse_left_down(context, &mut set_position);
                }
                Event::MouseButtonUp {
                    mouse_btn: MouseButton::Left,
                    ..
                } => {
                    mouse_left_click = false;
                }
                Event::MouseButtonDown {
                    mouse_btn: MouseButton::Right,
                    ..
                } => {
                    mouse_right_click = true;
                    handle_mouse_right_down(context);
                }
                Event::MouseButtonUp {
                    mouse_btn: MouseButton::Right,
                    ..
                } => {
                    mouse_right_click = false;
                }
                _ => {}
            }
        }

        render::render_level(&mut context.canvas, &context.level, &context.textures);
        render::render_text_texture(
            &mut context.canvas,
            &p1_text_texture,
            context.level.p1_position.0 * RENDER_SIZE,
            context.level.p1_position.1 * RENDER_SIZE,
            Some(context.level.scroll),
        );
        render::render_text_texture(
            &mut context.canvas,
            &p2_text_texture,
            context.level.p2_position.0 * RENDER_SIZE,
            context.level.p2_position.1 * RENDER_SIZE,
            Some(context.level.scroll),
        );
        let text_position = (8, 8);
        if set_position == 1 {
            render::render_text_texture_coordinates(
                &mut context.canvas,
                &p1_set_text_texture,
                text_position,
                None,
            );
        } else if set_position == 2 {
            render::render_text_texture_coordinates(
                &mut context.canvas,
                &p2_set_text_texture,
                text_position,
                None,
            );
        } else {
            render::render_text_texture_coordinates(
                &mut context.canvas,
                &help_text_texture,
                text_position,
                None,
            );
        };
        if prompt != PromptType::None {
            let prompt_texture = match prompt {
                PromptType::NewLevel => &create_new_level_text_texture,
                PromptType::Quit => &wanna_quit_text_texture,
                PromptType::None => unreachable!(),
            };
            render::render_text_texture(&mut context.canvas, prompt_texture, 200, 200, None);
            render::render_text_texture(&mut context.canvas, &press_y_text_texture, 200, 230, None);
        }
        let highlighted_id =
            get_tile_id_from_coordinate(context.mouse.0, context.mouse.1, TILES_X_PER_SCREEN, None);

        render::highlight_selected_tile(&mut context.canvas, highlighted_id);
        render::render_and_wait(&mut context.canvas);
    }
}

fn handle_mouse_left_down(context: &mut Context, set_position: &mut u8) {
    if *set_position > 0 {
        let position = if *set_position == 1 {
            &mut context.level.p1_position
        } else {
            &mut context.level.p2_position
        };
        *position =
            get_logical_coordinates(context.mouse.0, context.mouse.1, Some(context.level.scroll));
        *set_position = 0;
    } else {
        let pointed_tile = get_tile_id_from_coordinate(
            context.mouse.0,
            context.mouse.1,
            context.level.tiles[0].len() as u32,
            Some(context.level.scroll),
        );
        context.level.put_tile_to_level(
            pointed_tile,
            Some(context.selected_tile_id),
            &context.texture_type_selected,
        );
    }
}

fn handle_mouse_right_down(context: &mut Context) {
    let pointed_tile = get_tile_id_from_coordinate(
        context.mouse.0,
        context.mouse.1,
        context.level.tiles[0].len() as u32,
        Some(context.level.scroll),
    );
    context
        .level
        .put_tile_to_level(pointed_tile, None, &TextureType::SHADOW);
}
