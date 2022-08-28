extern crate sdl2;

use crate::context_util::resize;
use crate::crates::{get_crates, CrateClass};
use crate::create_text_texture;
use crate::editor_textures::EditorTextures;
use crate::level::StaticCrate;
use crate::level::StaticCrateType;
use crate::level::Steam;
use crate::render;
use crate::types::GameType;
use crate::util::*;
use crate::Context;
use crate::Graphics;
use crate::Level;
use crate::NextMode;
use crate::NextMode::*;
use crate::TextureType;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::render::Texture;
use sdl2::render::TextureQuery;

#[derive(PartialEq)]
enum NewLevelState {
    Prompt,
    XSize,
    YSize,
}

#[derive(PartialEq)]
enum SaveLevelType {
    Prompt,
    NameInput,
}

#[derive(PartialEq)]
enum ShadowPromptType {
    Enabled,
    Disabled,
}

#[derive(PartialEq)]
enum PromptType {
    None,
    NewLevel(NewLevelState),
    Save(SaveLevelType),
    CreateShadows(ShadowPromptType),
    Quit,
}

#[derive(PartialEq)]
enum InsertState {
    Instructions((u32, u32)), // level coordinates of currently manipulated item
    Place,
    Delete,
}

#[derive(PartialEq)]
enum InsertType {
    None,
    Spotlight(InsertState),
    Steam(InsertState),
    NormalCrate(InsertState),
    DMCrate(InsertState),
}

pub fn exec(context: &mut Context) -> NextMode {
    let mut textures = EditorTextures::new(context);
    let mut set_position: u8 = 0;
    let mut mouse_left_click: Option<(u32, u32)> = None;
    let mut mouse_right_click = false;
    let mut prompt = PromptType::None;
    let mut insert_item = InsertType::None;
    let mut new_level_size_x: String = String::new();
    let mut new_level_size_y: String = String::new();
    let mut drag_tiles = false;
    let crates = get_crates();

    let mut event_pump = context.sdl.event_pump().unwrap();
    loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    prompt = if prompt != PromptType::None
                        || insert_item != InsertType::None
                        || set_position > 0
                    {
                        insert_item = InsertType::None;
                        context.sdl.video().unwrap().text_input().stop();
                        set_position = 0;
                        PromptType::None
                    } else {
                        PromptType::Quit
                    };
                }
                Event::TextInput { text, .. } => match &prompt {
                    PromptType::NewLevel(new_level_state) => match new_level_state {
                        NewLevelState::XSize => {
                            sanitize_numeric_input(&text, &mut new_level_size_x)
                        }
                        NewLevelState::YSize => {
                            sanitize_numeric_input(&text, &mut new_level_size_y)
                        }
                        _ => {}
                    },
                    PromptType::Save(save_level_state) => match save_level_state {
                        SaveLevelType::NameInput => {
                            sanitize_level_name_input(&text, &mut context.level_save_name)
                        }
                        _ => {}
                    },
                    _ => (),
                },
                Event::Window { win_event, .. } => {
                    if resize(context, win_event) {
                        textures = EditorTextures::new(context);
                    }
                }
                Event::KeyDown { keycode, .. } => {
                    if let Some(key) = keycode {
                        match key {
                            Keycode::Space => {
                                return TileSelect;
                            }
                            Keycode::F1 => {
                                return Help;
                            }
                            Keycode::F2 => {
                                context.sdl.video().unwrap().text_input().stop();
                                prompt = PromptType::Save(SaveLevelType::Prompt);
                            }
                            Keycode::F3 => {
                                context.sdl.video().unwrap().text_input().stop();
                                return LoadLevel;
                            }
                            Keycode::F4 => {
                                prompt = PromptType::NewLevel(NewLevelState::Prompt);
                                new_level_size_x.clear();
                                new_level_size_y.clear();
                            }
                            Keycode::F6 => {
                                context.sdl.video().unwrap().text_input().stop();
                                prompt = PromptType::CreateShadows(if context.automatic_shadows {
                                    ShadowPromptType::Enabled
                                } else {
                                    ShadowPromptType::Disabled
                                });
                            }
                            Keycode::F7 => {
                                return GeneralLevelInfo;
                            }
                            Keycode::F8 => {
                                return RandomItemEditor(GameType::Normal);
                            }
                            Keycode::F9 => {
                                return RandomItemEditor(GameType::Deathmatch);
                            }
                            Keycode::Num1 | Keycode::Num2 => {
                                if !matches!(prompt, PromptType::NewLevel(_))
                                    && !matches!(prompt, PromptType::Save(_))
                                {
                                    set_position = if key == Keycode::Num1 { 1 } else { 2 };
                                    prompt = PromptType::None;
                                }
                            }
                            Keycode::Q | Keycode::W => {
                                if !matches!(prompt, PromptType::Save(_)) {
                                    insert_item = if key == Keycode::Q {
                                        InsertType::Spotlight(InsertState::Place)
                                    } else {
                                        InsertType::Spotlight(InsertState::Delete)
                                    };
                                    context.sdl.video().unwrap().text_input().stop();
                                    prompt = PromptType::None;
                                }
                            }
                            Keycode::A | Keycode::S => {
                                if !matches!(prompt, PromptType::Save(_)) {
                                    insert_item = if key == Keycode::A {
                                        InsertType::Steam(InsertState::Place)
                                    } else {
                                        InsertType::Steam(InsertState::Delete)
                                    };
                                    context.sdl.video().unwrap().text_input().stop();
                                    prompt = PromptType::None;
                                }
                            }
                            Keycode::Z | Keycode::X | Keycode::C => {
                                if !matches!(prompt, PromptType::Save(_)) {
                                    insert_item = if key == Keycode::Z {
                                        InsertType::NormalCrate(InsertState::Place)
                                    } else if key == Keycode::X {
                                        InsertType::DMCrate(InsertState::Place)
                                    } else {
                                        InsertType::NormalCrate(InsertState::Delete)
                                    };
                                    context.sdl.video().unwrap().text_input().stop();
                                    prompt = PromptType::None;
                                }
                            }
                            Keycode::Y => match &prompt {
                                PromptType::NewLevel(new_level_state) => match new_level_state {
                                    NewLevelState::Prompt => {
                                        prompt = PromptType::NewLevel(NewLevelState::XSize);
                                        context.sdl.video().unwrap().text_input().start();
                                    }
                                    _ => {}
                                },
                                PromptType::Save(save_level_state) => match save_level_state {
                                    SaveLevelType::Prompt => {
                                        prompt = PromptType::Save(SaveLevelType::NameInput);
                                        context.sdl.video().unwrap().text_input().start();
                                    }
                                    _ => {}
                                },
                                PromptType::CreateShadows(shadow_state) => {
                                    context.automatic_shadows = match shadow_state {
                                        ShadowPromptType::Enabled => false,
                                        ShadowPromptType::Disabled => {
                                            context.level.create_shadows();
                                            true
                                        }
                                    };
                                    prompt = PromptType::None;
                                }
                                PromptType::Quit => return Quit,
                                PromptType::None => {
                                    prompt = PromptType::None;
                                }
                            },
                            Keycode::Up => match &insert_item {
                                InsertType::Spotlight(state) => match state {
                                    InsertState::Instructions(coordinates) => {
                                        let spotlight_intensity =
                                            context.level.get_spotlight_from_level(&coordinates);
                                        context.level.put_spotlight_to_level(
                                            &coordinates,
                                            spotlight_intensity + 1,
                                        )
                                    }
                                    _ => (),
                                },
                                InsertType::Steam(state) => match state {
                                    InsertState::Instructions(coordinates) => {
                                        let steam =
                                            context.level.get_steam_from_level(&coordinates);
                                        if steam.range < 6 {
                                            context.level.put_steam_to_level(
                                                &coordinates,
                                                &Steam {
                                                    angle: steam.angle,
                                                    range: steam.range + 1,
                                                },
                                            )
                                        }
                                    }
                                    _ => (),
                                },
                                InsertType::NormalCrate(state) | InsertType::DMCrate(state) => {
                                    match state {
                                        InsertState::Instructions(coordinates) => {
                                            let mut crate_item = context
                                                .level
                                                .get_crate_from_level(&coordinates)
                                                .clone();
                                            if (crate_item.crate_class as u32)
                                                < CrateClass::Energy as u32
                                            {
                                                crate_item.crate_type = 0;
                                                crate_item.crate_class = CrateClass::from_u32(
                                                    crate_item.crate_class as u32 + 1,
                                                );
                                                context
                                                    .level
                                                    .put_crate_to_level(&coordinates, &crate_item)
                                            }
                                        }
                                        _ => (),
                                    }
                                }
                                _ => {
                                    if context.level.scroll.1 > 0 {
                                        context.level.scroll.1 = context.level.scroll.1 - 1
                                    }
                                }
                            },
                            Keycode::Down => match &insert_item {
                                InsertType::Spotlight(state) => match state {
                                    InsertState::Instructions(coordinates) => {
                                        let spotlight_intensity =
                                            context.level.get_spotlight_from_level(&coordinates);
                                        if spotlight_intensity > 0 {
                                            context.level.put_spotlight_to_level(
                                                &coordinates,
                                                spotlight_intensity - 1,
                                            )
                                        }
                                    }
                                    _ => (),
                                },
                                InsertType::Steam(state) => match state {
                                    InsertState::Instructions(coordinates) => {
                                        let steam =
                                            context.level.get_steam_from_level(&coordinates);
                                        if steam.range > 0 {
                                            context.level.put_steam_to_level(
                                                &coordinates,
                                                &Steam {
                                                    angle: steam.angle,
                                                    range: steam.range - 1,
                                                },
                                            )
                                        }
                                    }
                                    _ => (),
                                },
                                InsertType::NormalCrate(state) | InsertType::DMCrate(state) => {
                                    match state {
                                        InsertState::Instructions(coordinates) => {
                                            let mut crate_item = context
                                                .level
                                                .get_crate_from_level(&coordinates)
                                                .clone();
                                            if crate_item.crate_class as u32 > 0 {
                                                crate_item.crate_type = 0;
                                                crate_item.crate_class = CrateClass::from_u32(
                                                    crate_item.crate_class as u32 - 1,
                                                );
                                                context
                                                    .level
                                                    .put_crate_to_level(&coordinates, &crate_item)
                                            }
                                        }
                                        _ => (),
                                    }
                                }
                                _ => {
                                    if context.level.scroll.1
                                        + context.graphics.get_full_y_tiles_per_screen()
                                        < (context.level.tiles.len()) as u32
                                    {
                                        context.level.scroll.1 = context.level.scroll.1 + 1;
                                    }
                                }
                            },
                            Keycode::Left => match &insert_item {
                                InsertType::Steam(state) => match state {
                                    InsertState::Instructions(coordinates) => {
                                        let steam =
                                            context.level.get_steam_from_level(&coordinates);
                                        context.level.put_steam_to_level(
                                            &coordinates,
                                            &Steam {
                                                angle: (steam.angle + 360 - 5) % 360,
                                                range: steam.range,
                                            },
                                        )
                                    }
                                    _ => (),
                                },
                                InsertType::NormalCrate(state) | InsertType::DMCrate(state) => {
                                    match state {
                                        InsertState::Instructions(coordinates) => {
                                            let mut crate_item = context
                                                .level
                                                .get_crate_from_level(&coordinates)
                                                .clone();
                                            if crate_item.crate_type > 0 {
                                                crate_item.crate_type = crate_item.crate_type - 1;
                                                context
                                                    .level
                                                    .put_crate_to_level(coordinates, &crate_item);
                                            }
                                        }
                                        _ => (),
                                    }
                                }
                                _ => {
                                    if context.level.scroll.0 > 0 {
                                        context.level.scroll.0 = context.level.scroll.0 - 1;
                                    }
                                }
                            },
                            Keycode::Right => match &insert_item {
                                InsertType::Steam(state) => match state {
                                    InsertState::Instructions(coordinates) => {
                                        let steam =
                                            context.level.get_steam_from_level(&coordinates);
                                        context.level.put_steam_to_level(
                                            &coordinates,
                                            &Steam {
                                                angle: (steam.angle + 5) % 360,
                                                range: steam.range,
                                            },
                                        )
                                    }
                                    _ => (),
                                },
                                InsertType::NormalCrate(state) | InsertType::DMCrate(state) => {
                                    match state {
                                        InsertState::Instructions(coordinates) => {
                                            let mut crate_item = context
                                                .level
                                                .get_crate_from_level(&coordinates)
                                                .clone();
                                            if crate_item.crate_type
                                                < (crates[crate_item.crate_class as usize].len()
                                                    - 1)
                                                    as u8
                                            {
                                                crate_item.crate_type = crate_item.crate_type + 1;
                                                context
                                                    .level
                                                    .put_crate_to_level(coordinates, &crate_item);
                                            }
                                        }
                                        _ => (),
                                    }
                                }
                                _ => {
                                    if context.level.scroll.0
                                        + context.graphics.get_full_x_tiles_per_screen()
                                        < (context.level.tiles[0].len()) as u32
                                    {
                                        context.level.scroll.0 = context.level.scroll.0 + 1;
                                    }
                                }
                            },
                            Keycode::Return | Keycode::KpEnter => {
                                if matches!(
                                    insert_item,
                                    InsertType::Spotlight(InsertState::Instructions(_))
                                ) {
                                    insert_item = InsertType::Spotlight(InsertState::Place);
                                }
                                if matches!(
                                    insert_item,
                                    InsertType::Steam(InsertState::Instructions(_))
                                ) {
                                    insert_item = InsertType::Steam(InsertState::Place);
                                }
                                if matches!(
                                    insert_item,
                                    InsertType::NormalCrate(InsertState::Instructions(_))
                                ) {
                                    insert_item = InsertType::NormalCrate(InsertState::Place);
                                }
                                if matches!(
                                    insert_item,
                                    InsertType::DMCrate(InsertState::Instructions(_))
                                ) {
                                    insert_item = InsertType::DMCrate(InsertState::Place);
                                } else if prompt == PromptType::NewLevel(NewLevelState::XSize)
                                    && new_level_size_x.len() > 1
                                    && new_level_size_x.parse::<u8>().unwrap() >= 16
                                {
                                    prompt = PromptType::NewLevel(NewLevelState::YSize);
                                } else if prompt == PromptType::NewLevel(NewLevelState::YSize)
                                    && new_level_size_x.len() > 1
                                    && new_level_size_y.parse::<u8>().unwrap() >= 12
                                {
                                    context.level = Level::get_default_level((
                                        new_level_size_x.parse::<u8>().unwrap(),
                                        new_level_size_y.parse::<u8>().unwrap(),
                                    ));
                                    context.sdl.video().unwrap().text_input().stop();
                                    context.textures.saved_level_name = None;
                                    context.level_save_name.clear();
                                    prompt = PromptType::None;
                                } else if prompt == PromptType::Save(SaveLevelType::NameInput)
                                    && context.level_save_name.len() > 1
                                {
                                    let level_save_name_uppercase =
                                        context.level_save_name.to_uppercase();
                                    let level_saved_name =
                                        format!("{}.LEV", &level_save_name_uppercase);
                                    context.level.serialize(&level_saved_name).unwrap();
                                    context.sdl.video().unwrap().text_input().stop();
                                    context.textures.saved_level_name = Some(create_text_texture(
                                        &mut context.canvas,
                                        &context.texture_creator,
                                        &context.font,
                                        &level_saved_name.clone().to_lowercase(),
                                    ));
                                    prompt = PromptType::None;
                                }
                            }
                            Keycode::Backspace => match &prompt {
                                PromptType::NewLevel(new_level_state) => match new_level_state {
                                    NewLevelState::XSize => {
                                        new_level_size_x.pop();
                                    }
                                    NewLevelState::YSize => {
                                        new_level_size_y.pop();
                                    }
                                    _ => {}
                                },
                                PromptType::Save(save_level_state) => match save_level_state {
                                    SaveLevelType::NameInput => {
                                        context.level_save_name.pop();
                                    }
                                    _ => {}
                                },
                                _ => (),
                            },
                            Keycode::Plus | Keycode::KpPlus => {
                                if context.graphics.render_multiplier == 1 {
                                    context.graphics.render_multiplier = 2;
                                }
                            }
                            Keycode::Minus | Keycode::KpMinus => {
                                if context.graphics.render_multiplier == 2 {
                                    context.graphics.render_multiplier = 1;
                                    context.level.scroll = (0, 0);
                                }
                            }
                            _ => {
                                if prompt != PromptType::NewLevel(NewLevelState::XSize)
                                    && prompt != PromptType::NewLevel(NewLevelState::YSize)
                                    && prompt != PromptType::Save(SaveLevelType::NameInput)
                                {
                                    prompt = PromptType::None
                                }
                            }
                        }
                    }
                }
                Event::MouseMotion { x, y, .. } => {
                    if x >= 0 && y >= 0 {
                        context.mouse.0 = x as u32;
                        context.mouse.1 = y as u32;
                        if mouse_left_click.is_some() {
                            handle_mouse_left_down(
                                context,
                                &mut set_position,
                                &mut insert_item,
                                &mut drag_tiles,
                            );
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
                    mouse_left_click = Some(context.mouse);
                    handle_mouse_left_down(
                        context,
                        &mut set_position,
                        &mut insert_item,
                        &mut drag_tiles,
                    );
                }
                Event::MouseButtonUp {
                    mouse_btn: MouseButton::Left,
                    ..
                } => {
                    if drag_tiles {
                        drag_tiles = false;
                        if let Some(coordinates) = mouse_left_click {
                            let selected_level_tiles = get_selected_level_tiles(
                                &context.graphics,
                                &coordinates,
                                &get_limited_screen_level_size(
                                    &context.graphics,
                                    &context.mouse,
                                    &context.level,
                                    context.graphics.get_render_size(),
                                ),
                                context.level.tiles[0].len() as u32,
                                Some(context.level.scroll),
                            );
                            for level_tile_id in selected_level_tiles {
                                context.level.put_tile_to_level(
                                    level_tile_id,
                                    Some(context.selected_tile_id),
                                    &context.texture_type_selected,
                                );
                            }
                            if context.texture_type_selected == TextureType::SHADOW {
                                context.automatic_shadows = false;
                            } else if context.automatic_shadows {
                                context.level.create_shadows();
                            }
                        }
                    };
                    mouse_left_click = None;
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
        render::render_level(
            &mut context.canvas,
            &context.graphics,
            &context.level,
            &context.textures,
            &context.trigonometry,
        );
        let highlighted_id = get_tile_id_from_coordinates(
            &context.graphics,
            &get_limited_screen_level_size(
                &context.graphics,
                &context.mouse,
                &context.level,
                context.graphics.get_render_size(),
            ),
            context.graphics.get_x_tiles_per_screen(),
            None,
        );
        render::highlight_selected_tile(
            &mut context.canvas,
            &context.graphics,
            highlighted_id,
            &render::RendererColor::White,
        );
        let render_size = context.graphics.get_render_size();
        render::render_text_texture(
            &mut context.canvas,
            &textures.p1_text_texture,
            context.level.p1_position.0 * render_size,
            context.level.p1_position.1 * render_size,
            render_size,
            Some(context.level.scroll),
        );
        render::render_text_texture(
            &mut context.canvas,
            &textures.p2_text_texture,
            context.level.p2_position.0 * render_size,
            context.level.p2_position.1 * render_size,
            render_size,
            Some(context.level.scroll),
        );
        let text_position = (8, 8);
        let text_texture = if set_position == 1 {
            &textures.p1_set_text_texture
        } else if set_position == 2 {
            &textures.p2_set_text_texture
        } else if matches!(
            insert_item,
            InsertType::Spotlight(InsertState::Instructions(_))
        ) {
            &textures.spotlight_instructions_text_texture
        } else if matches!(insert_item, InsertType::Spotlight(InsertState::Place)) {
            &textures.spotlight_place_text_texture
        } else if matches!(insert_item, InsertType::Spotlight(InsertState::Delete)) {
            &textures.spotlight_delete_text_texture
        } else if matches!(insert_item, InsertType::Steam(InsertState::Instructions(_))) {
            &textures.steam_instructions_text_texture
        } else if matches!(insert_item, InsertType::Steam(InsertState::Place)) {
            &textures.steam_place_text_texture
        } else if matches!(insert_item, InsertType::Steam(InsertState::Delete)) {
            &textures.steam_delete_text_texture
        } else if matches!(insert_item, InsertType::NormalCrate(InsertState::Place)) {
            &textures.place_normal_crate_text_texture
        } else if matches!(insert_item, InsertType::DMCrate(InsertState::Place)) {
            &textures.place_deathmatch_create_text_texture
        } else if matches!(
            insert_item,
            InsertType::NormalCrate(InsertState::Instructions(_))
        ) || matches!(
            insert_item,
            InsertType::DMCrate(InsertState::Instructions(_))
        ) {
            &textures.insert_crate_text_texture
        } else if matches!(insert_item, InsertType::NormalCrate(InsertState::Delete))
            || matches!(insert_item, InsertType::DMCrate(InsertState::Delete))
        {
            &textures.delete_crate_text_texture
        } else {
            &textures.help_text_texture
        };
        render::render_text_texture_coordinates(
            &mut context.canvas,
            text_texture,
            text_position,
            render_size,
            None,
        );
        render_prompt_if_needed(
            context,
            &textures,
            &prompt,
            &new_level_size_x,
            &new_level_size_y,
        );
        if insert_item == InsertType::None {
            if let Some(coordinates) = mouse_left_click {
                let selected_screen_tiles = get_selected_level_tiles(
                    &context.graphics,
                    &coordinates,
                    &get_limited_screen_level_size(
                        &context.graphics,
                        &context.mouse,
                        &context.level,
                        context.graphics.get_render_size(),
                    ),
                    context.graphics.get_x_tiles_per_screen(),
                    None,
                );
                for screen_tile_id in selected_screen_tiles {
                    render::highlight_selected_tile(
                        &mut context.canvas,
                        &context.graphics,
                        screen_tile_id,
                        &render::RendererColor::White,
                    );
                }
            }
        }
        if let Some(texture) = &context.textures.saved_level_name {
            render::render_text_texture_coordinates(
                &mut context.canvas,
                &texture,
                get_bottom_text_position(context.graphics.resolution_y),
                render_size,
                None,
            );
        }
        render::render_and_wait(&mut context.canvas);
    }
}

fn sanitize_numeric_input(new_text: &str, target_text: &mut String) {
    if new_text.chars().all(char::is_numeric) && (target_text.len() + new_text.len() <= 3) {
        *target_text += new_text;
    }
}

fn sanitize_level_name_input(new_text: &str, target_text: &mut String) {
    if new_text.chars().all(char::is_alphanumeric) && (target_text.len() + new_text.len() <= 11) {
        *target_text += new_text;
    }
}

fn render_input_prompt(
    context: &mut Context,
    prompt_position: (u32, u32),
    prompt_line_spacing: u32,
    instruction_texture: &Texture,
    input_field: &str,
) {
    let render_size = context.graphics.get_render_size();
    render::render_text_texture(
        &mut context.canvas,
        instruction_texture,
        prompt_position.0,
        prompt_position.1 + 2 * prompt_line_spacing,
        render_size,
        None,
    );

    if !input_field.is_empty() {
        let input_text_texture = create_text_texture(
            &mut context.canvas,
            &context.texture_creator,
            &context.font,
            &input_field,
        );
        let TextureQuery { width, .. } = instruction_texture.query();
        render::render_text_texture(
            &mut context.canvas,
            &input_text_texture,
            prompt_position.0 + width * render::TEXT_SIZE_MULTIPLIER + 10,
            prompt_position.1 + 2 * prompt_line_spacing,
            render_size,
            None,
        );
    }
}

fn render_prompt_if_needed(
    context: &mut Context,
    textures: &EditorTextures,
    prompt: &PromptType,
    new_level_size_x: &str,
    new_level_size_y: &str,
) {
    if *prompt != PromptType::None {
        let prompt_position = (context.graphics.resolution_x / 2 - 100, 200);
        let prompt_line_spacing = 30;
        let prompt_texture = match &prompt {
            PromptType::NewLevel(state) => {
                match state {
                    NewLevelState::Prompt => (),
                    input_state => {
                        if *input_state == NewLevelState::XSize
                            || *input_state == NewLevelState::YSize
                        {
                            render_input_prompt(
                                context,
                                prompt_position,
                                prompt_line_spacing,
                                &textures.new_level_x_size_text_texture,
                                new_level_size_x,
                            );
                        }
                        if *input_state == NewLevelState::YSize {
                            render_input_prompt(
                                context,
                                (prompt_position.0, prompt_position.1 + prompt_line_spacing),
                                prompt_line_spacing,
                                &textures.new_level_y_size_text_texture,
                                new_level_size_y,
                            );
                        }
                    }
                }
                &textures.create_new_level_text_texture
            }
            PromptType::Save(save_level_state) => {
                match save_level_state {
                    SaveLevelType::Prompt => (),
                    SaveLevelType::NameInput => {
                        let level_save_name = context.level_save_name.clone();
                        render_input_prompt(
                            context,
                            prompt_position,
                            prompt_line_spacing,
                            &textures.filename_text_texture,
                            &level_save_name,
                        );
                    }
                };
                &textures.save_level_text_texture
            }
            PromptType::Quit => &textures.wanna_quit_text_texture,
            PromptType::CreateShadows(shadow_state) => match shadow_state {
                ShadowPromptType::Enabled => {
                    &textures.create_shadows_enabled_instructions_text_texture
                }
                ShadowPromptType::Disabled => {
                    &textures.create_shadows_disabled_instructions_text_texture
                }
            },
            PromptType::None => unreachable!(),
        };
        let render_size = context.graphics.get_render_size();
        render::render_text_texture(
            &mut context.canvas,
            prompt_texture,
            prompt_position.0,
            prompt_position.1,
            render_size,
            None,
        );
        render::render_text_texture(
            &mut context.canvas,
            &textures.press_y_text_texture,
            prompt_position.0,
            prompt_position.1 + prompt_line_spacing,
            render_size,
            None,
        );
    }
}

fn handle_mouse_left_down(
    context: &mut Context,
    set_position: &mut u8,
    insert_item: &mut InsertType,
    drag_tiles: &mut bool,
) {
    if *drag_tiles {
        return;
    }

    if *set_position > 0 {
        let position = if *set_position == 1 {
            &mut context.level.p1_position
        } else {
            &mut context.level.p2_position
        };
        *position = get_logical_coordinates(
            &context.graphics,
            context.mouse.0,
            context.mouse.1,
            Some(context.level.scroll),
        );
        *set_position = 0;
    } else {
        let level_coordinates = get_level_coordinates_from_screen_coordinates(
            &context.graphics,
            &context.mouse,
            &context.level.scroll,
        );
        if matches!(insert_item, InsertType::Spotlight(InsertState::Place)) {
            *insert_item = InsertType::Spotlight(InsertState::Instructions(level_coordinates));
            context.level.put_spotlight_to_level(&level_coordinates, 0);
        } else if matches!(insert_item, InsertType::Spotlight(InsertState::Delete)) {
            context
                .level
                .delete_spotlight_if_near(&level_coordinates, context.graphics.render_multiplier);
        } else if matches!(insert_item, InsertType::Steam(InsertState::Place)) {
            *insert_item = InsertType::Steam(InsertState::Instructions(level_coordinates));
            context
                .level
                .put_steam_to_level(&level_coordinates, &Steam { angle: 0, range: 1 });
        } else if matches!(insert_item, InsertType::Steam(InsertState::Delete)) {
            context
                .level
                .delete_steam_if_near(&level_coordinates, context.graphics.render_multiplier);
        } else if matches!(insert_item, InsertType::NormalCrate(InsertState::Place)) {
            *insert_item = InsertType::NormalCrate(InsertState::Instructions(level_coordinates));
            context.level.put_crate_to_level(
                &level_coordinates,
                &StaticCrateType {
                    crate_variant: StaticCrate::Normal,
                    crate_class: CrateClass::Weapon,
                    crate_type: 0,
                },
            );
        } else if matches!(insert_item, InsertType::DMCrate(InsertState::Place)) {
            *insert_item = InsertType::DMCrate(InsertState::Instructions(level_coordinates));
            context.level.put_crate_to_level(
                &level_coordinates,
                &StaticCrateType {
                    crate_variant: StaticCrate::Deathmatch,
                    crate_class: CrateClass::Weapon,
                    crate_type: 0,
                },
            );
        } else if matches!(insert_item, InsertType::NormalCrate(InsertState::Delete)) {
            context
                .level
                .delete_crate_if_near(&level_coordinates, context.graphics.render_multiplier);
        } else if *insert_item == InsertType::None {
            *drag_tiles = true;
        }
    }
}

fn handle_mouse_right_down(context: &mut Context) {
    let pointed_tile = get_tile_id_from_coordinates(
        &context.graphics,
        &limit_coordinates(
            &context.mouse,
            &(context.graphics.resolution_x, context.graphics.resolution_y),
        ),
        context.level.tiles[0].len() as u32,
        Some(context.level.scroll),
    );
    context
        .level
        .put_tile_to_level(pointed_tile, None, &TextureType::SHADOW);
    context.automatic_shadows = false;
}

fn get_limited_screen_level_size(
    graphics: &Graphics,
    mouse: &(u32, u32),
    level: &Level,
    render_size: u32,
) -> (u32, u32) {
    limit_coordinates(
        &(
            std::cmp::min(
                mouse.0,
                (level.tiles[0].len() as u32 - level.scroll.0) * render_size - 1,
            ),
            std::cmp::min(
                mouse.1,
                (level.tiles.len() as u32 - level.scroll.1) * render_size - 1,
            ),
        ),
        &(graphics.resolution_x, graphics.resolution_y),
    )
}
