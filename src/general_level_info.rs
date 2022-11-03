extern crate sdl2;

use crate::context_util::resize;
use crate::types::*;
use crate::Context;
use crate::Mode::*;
use crate::{get_bottom_text_position, Renderer};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::Texture;

enum Value {
    Comment,
    TimeLimit,
    Number(usize),
}

struct ConfigOption<'a> {
    texture: Texture<'a>,
    value: Value,
}

fn load_text<'a>(renderer: &'a Renderer, context: &Context, text: &str) -> Texture<'a> {
    renderer.create_text_texture(&context.font, text)
}

fn load_value_text<'a>(
    renderer: &'a Renderer,
    context: &Context,
    value: &Value,
) -> Option<Texture<'a>> {
    let string = match value {
        Value::Number(number) => context.level.general_info.enemy_table[*number].to_string(),
        Value::TimeLimit => format!("{} seconds", context.level.general_info.time_limit),
        Value::Comment => context.level.general_info.comment.to_string(),
    };
    if !string.is_empty() {
        Some(renderer.create_text_texture(&context.font, &string))
    } else {
        None
    }
}

fn sanitize_level_comment_input(new_text: &str, target_text: &mut String) {
    if (new_text.chars().all(char::is_alphanumeric) || new_text.chars().all(char::is_whitespace))
        && (target_text.len() + new_text.len() <= 19)
    {
        *target_text += new_text;
    }
}

pub struct GeneralLevelInfoState<'a> {
    renderer: &'a Renderer,
    esc_instruction_text: Texture<'a>,
    options: [ConfigOption<'a>; 10],
    selected: usize,
}

impl<'a> GeneralLevelInfoState<'a> {
    pub fn new(renderer: &'a Renderer, context: &Context<'a>) -> Self {
        let options = [
            ConfigOption {
                texture: load_text(renderer, context, "level comment:"),
                value: Value::Comment,
            },
            ConfigOption {
                texture: load_text(renderer, context, "time limit:"),
                value: Value::TimeLimit,
            },
            ConfigOption {
                texture: load_text(renderer, context, "pistol boys:"),
                value: Value::Number(0),
            },
            ConfigOption {
                texture: load_text(renderer, context, "shotgun maniacs:"),
                value: Value::Number(1),
            },
            ConfigOption {
                texture: load_text(renderer, context, "uzi rebels:"),
                value: Value::Number(2),
            },
            ConfigOption {
                texture: load_text(renderer, context, "commandos:"),
                value: Value::Number(3),
            },
            ConfigOption {
                texture: load_text(renderer, context, "granade mofos:"),
                value: Value::Number(4),
            },
            ConfigOption {
                texture: load_text(renderer, context, "civilians:"),
                value: Value::Number(5),
            },
            ConfigOption {
                texture: load_text(renderer, context, "punishers:"),
                value: Value::Number(6),
            },
            ConfigOption {
                texture: load_text(renderer, context, "flamers:"),
                value: Value::Number(7),
            },
        ];
        let esc_instruction_text = load_text(renderer, context, "press ESC to exit");

        GeneralLevelInfoState {
            renderer,
            options,
            esc_instruction_text,
            selected: 0usize,
        }
    }

    pub fn frame(&mut self, context: &mut Context<'a>) -> Mode {
        self.enable_text_editing_if_needed(context);

        let mut event_pump = context.sdl.event_pump().unwrap();
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
                    if resize(self.renderer, context, win_event) {
                        return Editor;
                    }
                }
                Event::TextInput { text, .. } => {
                    if let Value::Comment = self.options[self.selected].value {
                        sanitize_level_comment_input(&text, &mut context.level.general_info.comment)
                    }
                }
                Event::KeyDown { keycode, .. } => match keycode.unwrap() {
                    Keycode::Down => {
                        if self.selected < self.options.len() - 1 {
                            self.selected += 1;
                            self.enable_text_editing_if_needed(context);
                        }
                    }
                    Keycode::Up => {
                        if self.selected > 0 {
                            self.selected -= 1;
                            self.enable_text_editing_if_needed(context);
                        }
                    }
                    Keycode::Right => match self.options[self.selected].value {
                        Value::Number(index) => context.level.general_info.enemy_table[index] += 1,
                        Value::TimeLimit => context.level.general_info.time_limit += 10,
                        _ => (),
                    },
                    Keycode::Left => match self.options[self.selected].value {
                        Value::Number(index) => {
                            let value = &mut context.level.general_info.enemy_table[index];
                            if *value > 0 {
                                *value -= 1;
                            }
                        }
                        Value::TimeLimit => {
                            let value = &mut context.level.general_info.time_limit;
                            if *value > 0 {
                                *value -= 10;
                            }
                        }
                        _ => (),
                    },
                    Keycode::Backspace => {
                        if let Value::Comment = self.options[self.selected].value {
                            context.level.general_info.comment.pop();
                        }
                    }
                    _ => (),
                },
                _ => {}
            }
        }

        self.renderer.clear_screen(Color::from((0, 0, 0)));
        let mut option_position = (40, 20);
        let mut value_position = (300, option_position.1);
        let render_size = context.graphics.get_render_size();
        for x in 0..self.options.len() {
            let option = &self.options[x];
            if self.selected == x {
                self.renderer.render_text_texture(
                    &context.textures.selected_icon,
                    option_position.0 - 20,
                    option_position.1 + 3,
                    render_size,
                    None,
                );
            }
            self.renderer.render_text_texture(
                &option.texture,
                option_position.0,
                option_position.1,
                render_size,
                None,
            );
            let value_texture = &load_value_text(self.renderer, context, &option.value);
            match value_texture {
                Some(texture) => self.renderer.render_text_texture(
                    texture,
                    value_position.0,
                    value_position.1,
                    render_size,
                    None,
                ),
                None => (),
            };
            option_position.1 += 20;
            value_position.1 = option_position.1;
        }
        self.renderer.render_text_texture_coordinates(
            &self.esc_instruction_text,
            get_bottom_text_position(context.graphics.resolution_y),
            render_size,
            None,
        );
        self.renderer.render_and_wait();
        GeneralLevelInfo
    }

    fn enable_text_editing_if_needed(&self, context: &Context) {
        match self.options[self.selected].value {
            Value::Comment => context.sdl.video().unwrap().text_input().start(),
            _ => context.sdl.video().unwrap().text_input().stop(),
        }
    }
}
