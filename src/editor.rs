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
use sdl2::render::Texture;
use sdl2::render::TextureQuery;

#[derive(PartialEq)]
enum NewLevelType {
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
enum PromptType {
    None,
    NewLevel(NewLevelType),
    Save(SaveLevelType),
    Quit,
}

#[derive(PartialEq)]
enum SpotlightType {
    Instructions((u32, u32)), // level coordinates of currently manipulated spotlight
    Place,
    Delete,
}

#[derive(PartialEq)]
enum InsertType {
    None,
    Spotlight(SpotlightType),
}

struct Textures<'a> {
    p1_text_texture: Texture<'a>,
    p2_text_texture: Texture<'a>,
    p1_set_text_texture: Texture<'a>,
    p2_set_text_texture: Texture<'a>,
    help_text_texture: Texture<'a>,
    create_new_level_text_texture: Texture<'a>,
    wanna_quit_text_texture: Texture<'a>,
    save_level_text_texture: Texture<'a>,
    filename_text_texture: Texture<'a>,
    press_y_text_texture: Texture<'a>,
    new_level_x_size_text_texture: Texture<'a>,
    new_level_y_size_text_texture: Texture<'a>,
    spotlight_place_text_texture: Texture<'a>,
    spotlight_delete_text_texture: Texture<'a>,
    spotlight_instructions_text_texture: Texture<'a>,
}

pub fn exec(context: &mut Context) -> NextMode {
    let textures = Textures {
        p1_text_texture: render::get_font_texture(&context.texture_creator, &context.font, "PL1"),
        p2_text_texture: render::get_font_texture(&context.texture_creator, &context.font, "PL2"),
        p1_set_text_texture: render::get_font_texture(
            &context.texture_creator,
            &context.font,
            "PLACE PL1 START POINT",
        ),
        p2_set_text_texture: render::get_font_texture(
            &context.texture_creator,
            &context.font,
            "PLACE PL2 START POINT",
        ),
        help_text_texture: render::get_font_texture(
            &context.texture_creator,
            &context.font,
            "F1 FOR HELP",
        ),
        create_new_level_text_texture: render::get_font_texture(
            &context.texture_creator,
            &context.font,
            "CREATE NEW LEVEL?",
        ),
        wanna_quit_text_texture: render::get_font_texture(
            &context.texture_creator,
            &context.font,
            "REALLY WANNA QUIT?",
        ),
        save_level_text_texture: render::get_font_texture(
            &context.texture_creator,
            &context.font,
            "SAVE LEVEL?",
        ),
        filename_text_texture: render::get_font_texture(
            &context.texture_creator,
            &context.font,
            "FILENAME:",
        ),
        press_y_text_texture: render::get_font_texture(
            &context.texture_creator,
            &context.font,
            "PRESS Y TO CONFIRM",
        ),
        new_level_x_size_text_texture: render::get_font_texture(
            &context.texture_creator,
            &context.font,
            "X-SIZE (>= 16 BLOCKS):",
        ),
        new_level_y_size_text_texture: render::get_font_texture(
            &context.texture_creator,
            &context.font,
            "Y-SIZE (>= 12 BLOCKS):",
        ),
        spotlight_place_text_texture: render::get_font_texture(
            &context.texture_creator,
            &context.font,
            "PLACE SPOTLIGHT (ESC TO CANCEL)",
        ),
        spotlight_delete_text_texture: render::get_font_texture(
            &context.texture_creator,
            &context.font,
            "DELETE SPOTLIGHT (ESC TO CANCEL)",
        ),
        spotlight_instructions_text_texture: render::get_font_texture(
            &context.texture_creator,
            &context.font,
            "USE UP AND DOWN KEYS TO ADJUST SIZE, ENTER TO ACCEPT",
        ),
    };
    let mut set_position: u8 = 0;
    let mut mouse_left_click: Option<(u32, u32)> = None;
    let mut mouse_right_click = false;
    let mut prompt = PromptType::None;
    let mut insert_item = InsertType::None;
    let mut new_level_size_x: String = String::new();
    let mut new_level_size_y: String = String::new();
    let mut drag_tiles = false;

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
                        NewLevelType::XSize => sanitize_numeric_input(&text, &mut new_level_size_x),
                        NewLevelType::YSize => sanitize_numeric_input(&text, &mut new_level_size_y),
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
                                prompt = PromptType::NewLevel(NewLevelType::Prompt);
                                new_level_size_x.clear();
                                new_level_size_y.clear();
                            }
                            Keycode::F7 => {
                                return GeneralLevelInfo;
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
                                        InsertType::Spotlight(SpotlightType::Place)
                                    } else {
                                        InsertType::Spotlight(SpotlightType::Delete)
                                    };
                                    context.sdl.video().unwrap().text_input().stop();
                                    prompt = PromptType::None;
                                }
                            }
                            Keycode::Y => match &prompt {
                                PromptType::NewLevel(new_level_state) => match new_level_state {
                                    NewLevelType::Prompt => {
                                        prompt = PromptType::NewLevel(NewLevelType::XSize);
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
                                PromptType::Quit => return Quit,
                                PromptType::None => {
                                    prompt = PromptType::None;
                                }
                            },
                            Keycode::Up => match &insert_item {
                                InsertType::Spotlight(state) => match state {
                                    SpotlightType::Instructions(coordinates) => {
                                        let spotlight_intensity =
                                            context.level.get_spotlight_from_level(&coordinates);
                                        context.level.put_spotlight_to_level(
                                            &coordinates,
                                            spotlight_intensity + 1,
                                        )
                                    }
                                    _ => (),
                                },
                                _ => {
                                    if context.level.scroll.1 > 0 {
                                        context.level.scroll.1 = context.level.scroll.1 - 1
                                    }
                                }
                            },
                            Keycode::Down => match &insert_item {
                                InsertType::Spotlight(state) => match state {
                                    SpotlightType::Instructions(coordinates) => {
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
                                _ => {
                                    if context.level.scroll.1 + TILES_Y_PER_SCREEN
                                        < (context.level.tiles.len()) as u32
                                    {
                                        context.level.scroll.1 = context.level.scroll.1 + 1;
                                    }
                                }
                            },
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
                            Keycode::Return | Keycode::KpEnter => {
                                if matches!(
                                    insert_item,
                                    InsertType::Spotlight(SpotlightType::Instructions(_))
                                ) {
                                    insert_item = InsertType::Spotlight(SpotlightType::Place);
                                } else if prompt == PromptType::NewLevel(NewLevelType::XSize)
                                    && new_level_size_x.len() > 1
                                    && new_level_size_x.parse::<u8>().unwrap() >= 16
                                {
                                    prompt = PromptType::NewLevel(NewLevelType::YSize);
                                } else if prompt == PromptType::NewLevel(NewLevelType::YSize)
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
                                    context.textures.saved_level_name =
                                        Some(render::get_font_texture(
                                            &context.texture_creator,
                                            &context.font,
                                            &level_saved_name,
                                        ));
                                    prompt = PromptType::None;
                                }
                            }
                            Keycode::Backspace => match &prompt {
                                PromptType::NewLevel(new_level_state) => match new_level_state {
                                    NewLevelType::XSize => {
                                        new_level_size_x.pop();
                                    }
                                    NewLevelType::YSize => {
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
                            _ => {
                                if prompt != PromptType::NewLevel(NewLevelType::XSize)
                                    && prompt != PromptType::NewLevel(NewLevelType::YSize)
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
                                &coordinates,
                                &context.mouse,
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
        render::render_level(&mut context.canvas, &context.level, &context.textures);
        let highlighted_id =
            get_tile_id_from_coordinate(context.mouse.0, context.mouse.1, TILES_X_PER_SCREEN, None);
        render::highlight_selected_tile(
            &mut context.canvas,
            highlighted_id,
            &render::RendererColor::White,
        );
        render::render_text_texture(
            &mut context.canvas,
            &textures.p1_text_texture,
            context.level.p1_position.0 * RENDER_SIZE,
            context.level.p1_position.1 * RENDER_SIZE,
            Some(context.level.scroll),
        );
        render::render_text_texture(
            &mut context.canvas,
            &textures.p2_text_texture,
            context.level.p2_position.0 * RENDER_SIZE,
            context.level.p2_position.1 * RENDER_SIZE,
            Some(context.level.scroll),
        );
        let text_position = (8, 8);
        let text_texture = if set_position == 1 {
            &textures.p1_set_text_texture
        } else if set_position == 2 {
            &textures.p2_set_text_texture
        } else if matches!(
            insert_item,
            InsertType::Spotlight(SpotlightType::Instructions(_))
        ) {
            &textures.spotlight_instructions_text_texture
        } else if matches!(insert_item, InsertType::Spotlight(SpotlightType::Place)) {
            &textures.spotlight_place_text_texture
        } else if matches!(insert_item, InsertType::Spotlight(SpotlightType::Delete)) {
            &textures.spotlight_delete_text_texture
        } else {
            &textures.help_text_texture
        };
        render::render_text_texture_coordinates(
            &mut context.canvas,
            text_texture,
            text_position,
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
                    &coordinates,
                    &context.mouse,
                    TILES_X_PER_SCREEN,
                    None,
                );
                for screen_tile_id in selected_screen_tiles {
                    render::highlight_selected_tile(
                        &mut context.canvas,
                        screen_tile_id,
                        &render::RendererColor::White,
                    );
                }
            }
        }
        if let Some(texture) = &context.textures.saved_level_name {
            render::render_text_texture_coordinates(&mut context.canvas, &texture, (10, 455), None);
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
    render::render_text_texture(
        &mut context.canvas,
        instruction_texture,
        prompt_position.0,
        prompt_position.1 + 2 * prompt_line_spacing,
        None,
    );

    if !input_field.is_empty() {
        let input_text_texture =
            render::get_font_texture(&context.texture_creator, &context.font, &input_field);
        let TextureQuery { width, .. } = instruction_texture.query();
        render::render_text_texture(
            &mut context.canvas,
            &input_text_texture,
            prompt_position.0 + width + 10,
            prompt_position.1 + 2 * prompt_line_spacing,
            None,
        );
    }
}

fn render_prompt_if_needed(
    context: &mut Context,
    textures: &Textures,
    prompt: &PromptType,
    new_level_size_x: &str,
    new_level_size_y: &str,
) {
    if *prompt != PromptType::None {
        let prompt_position = (200, 200);
        let prompt_line_spacing = 30;
        let prompt_texture = match &prompt {
            PromptType::NewLevel(state) => {
                match state {
                    NewLevelType::Prompt => (),
                    input_state => {
                        if *input_state == NewLevelType::XSize
                            || *input_state == NewLevelType::YSize
                        {
                            render_input_prompt(
                                context,
                                prompt_position,
                                prompt_line_spacing,
                                &textures.new_level_x_size_text_texture,
                                new_level_size_x,
                            );
                        }
                        if *input_state == NewLevelType::YSize {
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
            PromptType::None => unreachable!(),
        };
        render::render_text_texture(
            &mut context.canvas,
            prompt_texture,
            prompt_position.0,
            prompt_position.1,
            None,
        );
        render::render_text_texture(
            &mut context.canvas,
            &textures.press_y_text_texture,
            prompt_position.0,
            prompt_position.1 + prompt_line_spacing,
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
        *position =
            get_logical_coordinates(context.mouse.0, context.mouse.1, Some(context.level.scroll));
        *set_position = 0;
    } else if matches!(insert_item, InsertType::Spotlight(SpotlightType::Place))
        || matches!(insert_item, InsertType::Spotlight(SpotlightType::Delete))
    {
        let level_coordinates =
            get_level_coordinates_from_screen_coordinates(&context.mouse, &context.level.scroll);
        if matches!(insert_item, InsertType::Spotlight(SpotlightType::Place)) {
            *insert_item = InsertType::Spotlight(SpotlightType::Instructions(level_coordinates));
            context.level.put_spotlight_to_level(&level_coordinates, 0);
        } else {
            context.level.delete_spotlight_if_near(&level_coordinates);
        }
    } else if *insert_item == InsertType::None {
        *drag_tiles = true;
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
