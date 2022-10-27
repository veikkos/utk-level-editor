use crate::crates::CrateClass;
use crate::fn2::FN2;
use crate::level::DIFF_BULLETS;
use crate::level::DIFF_WEAPONS;
use crate::level::{StaticCrate, StaticCrateType};
use crate::types::*;
use crate::util::*;
use crate::Graphics;
use crate::Level;
use crate::Textures;
use sdl2::image::LoadTexture;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::rect::Point;
use sdl2::rect::Rect;
use sdl2::render::TextureQuery;
use sdl2::render::{BlendMode, Texture};
use sdl2::render::{Canvas, TextureCreator};
use sdl2::video::{Window, WindowContext};
use std::cell::{RefCell, RefMut};
use std::collections::HashMap;
use std::time::Duration;

pub const TEXT_SIZE_MULTIPLIER: u32 = 2;

pub enum RendererColor {
    White,
    Red,
    Blue,
    LightBlue,
    LightGreen,
}

fn get_sdl_color(color: &RendererColor) -> Color {
    match &color {
        RendererColor::White => Color::from((255, 255, 255)),
        RendererColor::Red => Color::from((255, 0, 0)),
        RendererColor::Blue => Color::from((0, 0, 255)),
        RendererColor::LightBlue => Color::from((100, 100, 255)),
        RendererColor::LightGreen => Color::from((100, 255, 100)),
    }
}

pub struct Renderer {
    canvas: RefCell<Canvas<Window>>,
    texture_creator: TextureCreator<WindowContext>,
}

impl Renderer {
    pub fn new(window: Window) -> Self {
        let canvas = window.into_canvas().build().unwrap();
        let texture_creator = canvas.texture_creator();
        Self {
            canvas: RefCell::new(canvas),
            texture_creator,
        }
    }

    pub fn load_texture(&self, path: &str) -> Texture {
        self.texture_creator.load_texture(path).unwrap()
    }

    pub fn clear_screen(&self, color: Color) {
        self.canvas_mut().set_draw_color(color);
        self.canvas_mut().clear();
    }

    pub fn highlight_selected_tile(&self, graphics: &Graphics, id: u32, color: &RendererColor) {
        self.canvas_mut().set_draw_color(get_sdl_color(color));

        let render_size = graphics.get_render_size();
        let render_multiplier = graphics.render_multiplier;
        let (x_logical, y_logical) = get_tile_coordinates(
            id,
            graphics.get_x_tiles_per_screen() * graphics.tile_size,
            graphics.tile_size,
        );
        let x = x_logical * render_multiplier;
        let y = y_logical * render_multiplier;

        self.draw_line(x, y, x, y + render_size - 1);
        self.draw_line(x, y, x + render_size - 1, y);
        self.draw_line(
            x + render_size - 1,
            y,
            x + render_size - 1,
            y + render_size - 1,
        );
        self.draw_line(
            x,
            y + render_size - 1,
            x + render_size - 1,
            y + render_size - 1,
        );
    }

    pub fn draw_line(&self, x0: u32, y0: u32, x1: u32, y1: u32) {
        let x0_signed = x0 as i32;
        let y0_signed = y0 as i32;
        let x1_signed = x1 as i32;
        let y1_signed = y1 as i32;

        self.canvas_mut()
            .draw_line(
                Point::from((x0_signed, y0_signed)),
                Point::from((x1_signed, y1_signed)),
            )
            .unwrap();
    }

    pub fn fill_and_render_texture(&self, color: Color, texture: &Texture, dst: Rect) {
        self.canvas_mut().set_draw_color(color);
        self.canvas_mut().fill_rect(dst).unwrap();
        self.canvas_mut().copy(texture, None, dst).unwrap();
    }

    pub fn render_text_texture(
        &self,
        texture: &Texture,
        x: u32,
        y: u32,
        render_size: u32,
        scroll: Option<(u32, u32)>,
    ) {
        let TextureQuery { width, height, .. } = texture.query();
        let scroll = scroll.unwrap_or((0, 0));
        let dst = Rect::new(
            x as i32 - (scroll.0 * render_size) as i32,
            y as i32 - (scroll.1 * render_size) as i32,
            width * TEXT_SIZE_MULTIPLIER,
            height * TEXT_SIZE_MULTIPLIER,
        );

        self.canvas_mut().copy(&texture, None, dst).unwrap();
    }

    pub fn render_text_texture_coordinates(
        &self,
        texture: &Texture,
        coordinates: (u32, u32),
        render_size: u32,
        scroll: Option<(u32, u32)>,
    ) {
        self.render_text_texture(texture, coordinates.0, coordinates.1, render_size, scroll);
    }

    fn draw_circle(&self, x_center: i32, y_center: i32, radius: u32, color: &RendererColor) {
        self.canvas_mut().set_draw_color(get_sdl_color(color));

        // https://stackoverflow.com/a/48291620
        let diameter: i32 = radius as i32 * 2;
        let mut x: i32 = radius as i32 - 1;
        let mut y: i32 = 0;
        let mut tx: i32 = 1;
        let mut ty: i32 = 1;
        let mut error: i32 = tx - diameter;

        while x >= y {
            self.canvas_mut()
                .draw_point(Point::new(x_center + x, y_center - y))
                .unwrap();
            self.canvas_mut()
                .draw_point(Point::new(x_center + x, y_center + y))
                .unwrap();
            self.canvas_mut()
                .draw_point(Point::new(x_center - x, y_center - y))
                .unwrap();
            self.canvas_mut()
                .draw_point(Point::new(x_center - x, y_center + y))
                .unwrap();
            self.canvas_mut()
                .draw_point(Point::new(x_center + y, y_center - x))
                .unwrap();
            self.canvas_mut()
                .draw_point(Point::new(x_center + y, y_center + x))
                .unwrap();
            self.canvas_mut()
                .draw_point(Point::new(x_center - y, y_center - x))
                .unwrap();
            self.canvas_mut()
                .draw_point(Point::new(x_center - y, y_center + x))
                .unwrap();

            if error <= 0 {
                y = y + 1;
                error += ty;
                ty += 2;
            }

            if error > 0 {
                x = x - 1;
                tx += 2;
                error += tx - diameter;
            }
        }
    }

    pub fn render_level(
        &self,
        graphics: &Graphics,
        level: &Level,
        textures: &Textures,
        trigonometry: &Trigonometry,
    ) {
        self.canvas_mut().set_draw_color(Color::from((0, 0, 0)));
        self.canvas_mut().clear();
        let render_size = graphics.get_render_size();

        for y in 0..std::cmp::min(level.tiles.len() as u32, graphics.get_y_tiles_per_screen()) {
            for x in 0..std::cmp::min(
                level.tiles[y as usize].len() as u32,
                graphics.get_x_tiles_per_screen(),
            ) {
                let (x_index, y_index) = get_scroll_corrected_indexes(level.scroll, x, y);
                if y_index >= level.tiles.len() || x_index >= level.tiles[y_index].len() {
                    continue;
                }
                let texture = match level.tiles[y_index][x_index].texture_type {
                    TextureType::FLOOR => &textures.floor,
                    TextureType::WALLS => &textures.walls,
                    TextureType::SHADOW => unreachable!(),
                };
                let (texture_width, _texture_height) = get_texture_size(texture);
                let src = get_block(
                    level.tiles[y_index][x_index].id,
                    texture_width,
                    graphics.tile_size,
                );
                let (x_absolute, y_absolute) =
                    get_absolute_coordinates_from_logical(x, y, graphics.get_render_size());
                let dst = Rect::new(x_absolute, y_absolute, render_size, render_size);
                self.canvas_mut().copy(texture, src, dst).unwrap();
                let (shadow_texture_width, _shadow_texture_height) =
                    get_texture_size(&textures.shadows);
                if level.tiles[y_index][x_index].shadow > 0 {
                    let src = get_block(
                        level.tiles[y_index][x_index].shadow - 1,
                        shadow_texture_width,
                        graphics.tile_size,
                    );
                    self.canvas_mut().copy(&textures.shadows, src, dst).unwrap();
                }
            }
        }
        for (coordinates, spotlight) in &level.spotlights {
            let (x_screen, y_screen) =
                get_screen_coordinates_from_level_coordinates(graphics, coordinates, &level.scroll);
            self.draw_circle(
                x_screen,
                y_screen,
                get_spotlight_render_radius(spotlight),
                &RendererColor::Blue,
            );
        }
        for (coordinates, steam) in &level.steams {
            let (x_screen, y_screen) =
                get_screen_coordinates_from_level_coordinates(graphics, coordinates, &level.scroll);
            for x in 0..6 {
                let multiplier = x as f32 * 6.0 * steam.range as f32;
                self.draw_circle(
                    x_screen + (trigonometry.sin[steam.angle as usize] * multiplier) as i32,
                    y_screen + (trigonometry.cos[steam.angle as usize] * multiplier) as i32,
                    get_steam_render_radius() + x * 2,
                    &RendererColor::Red,
                );
            }
        }

        self.render_crates(graphics, &level.scroll, &textures, &level.crates.staticc);
    }

    fn render_crates(
        &self,
        graphics: &Graphics,
        scroll: &(u32, u32),
        textures: &Textures,
        crates: &HashMap<(u32, u32), StaticCrateType>,
    ) {
        for (coordinates, crate_item) in crates {
            let box_size = get_crate_render_size();
            let (x_screen, y_screen) =
                get_screen_coordinates_from_level_coordinates(graphics, &coordinates, scroll);
            self.canvas_mut()
                .set_draw_color(get_sdl_color(match crate_item.crate_variant {
                    StaticCrate::Normal => &RendererColor::LightGreen,
                    StaticCrate::Deathmatch => &RendererColor::LightBlue,
                }));
            self.canvas_mut()
                .draw_rect(Rect::new(x_screen, y_screen, box_size, box_size))
                .unwrap();
            self.canvas_mut()
                .draw_rect(Rect::new(
                    x_screen + 1,
                    y_screen + 1,
                    box_size - 2,
                    box_size - 2,
                ))
                .unwrap();

            let texture_index = match crate_item.crate_class {
                CrateClass::Weapon => 0,
                CrateClass::Bullet => DIFF_WEAPONS,
                CrateClass::Energy => DIFF_BULLETS + DIFF_WEAPONS,
            } + crate_item.crate_type as u32;
            let texture = &textures.crates[texture_index as usize];
            let TextureQuery { height, .. } = texture.query();
            self.render_text_texture(
                texture,
                (x_screen - 10) as u32,
                (y_screen - 9 - height as i32) as u32,
                graphics.get_render_size(),
                None,
            );
        }
    }

    pub fn render_and_wait(&self) {
        self.canvas_mut().present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    pub fn create_text_texture(&self, font: &FN2, text: &str) -> Texture {
        let (width, height) = get_text_texture_size(font, text);
        let mut texture = self
            .texture_creator
            .create_texture_target(
                PixelFormatEnum::RGBA8888,
                if width > 0 {
                    width + TEXT_SHADOW_PIXELS
                } else {
                    1
                },
                if height > 0 {
                    height + TEXT_SHADOW_PIXELS
                } else {
                    1
                },
            )
            .map_err(|e| e.to_string())
            .unwrap();
        texture.set_blend_mode(BlendMode::Blend);

        self.canvas_mut()
            .with_texture_canvas(&mut texture, |texture_canvas| {
                texture_canvas.set_draw_color(Color::RGB(0, 0, 0));
                self.render_text_to_canvas(
                    texture_canvas,
                    font,
                    TEXT_SHADOW_PIXELS,
                    TEXT_SHADOW_PIXELS,
                    text,
                );
                texture_canvas.set_draw_color(Color::RGB(255, 0, 0));
                self.render_text_to_canvas(texture_canvas, font, 0, 0, text);
            })
            .map_err(|e| e.to_string())
            .unwrap();

        texture
    }

    fn render_text_to_canvas(
        &self,
        canvas: &mut Canvas<Window>,
        font: &FN2,
        x: u32,
        y: u32,
        text: &str,
    ) {
        let mut offset = 0;
        for c in text.chars() {
            let character_index = char_to_index(c);
            if character_index < INDEX_OFFSET {
                offset += SPACE_WIDTH as u32;
            } else {
                offset += self.render_character(
                    canvas,
                    &font,
                    (character_index - INDEX_OFFSET) as usize,
                    x + offset,
                    y,
                );
            }
        }
    }

    pub fn render_character(
        &self,
        canvas: &mut Canvas<Window>,
        characters: &FN2,
        index: usize,
        x: u32,
        y: u32,
    ) -> u32 {
        let character = &characters[index];
        for line in &character.lines {
            canvas
                .draw_line(
                    Point::new(line.x as i32 + x as i32, line.y as i32 + y as i32),
                    Point::new(
                        line.x as i32 + x as i32 + line.width as i32 - 1,
                        line.y as i32 + y as i32,
                    ),
                )
                .unwrap();
        }
        character.width
    }

    pub fn window_size(&self) -> (u32, u32) {
        self.canvas_mut().window().size()
    }

    fn canvas_mut(&self) -> RefMut<Canvas<Window>> {
        self.canvas.borrow_mut()
    }
}

pub fn get_texture_rect(texture: &Texture, render_multiplier: u32) -> Rect {
    let (width, height) = get_texture_render_size(&texture, render_multiplier);
    Rect::new(0, 0, width, height)
}

pub fn get_texture_size(texture: &Texture) -> (u32, u32) {
    let TextureQuery { width, height, .. } = texture.query();
    (width, height)
}

pub fn get_texture_render_size(texture: &Texture, render_multiplier: u32) -> (u32, u32) {
    let (width, height) = get_texture_size(&texture);
    (width * render_multiplier, height * render_multiplier)
}

fn get_text_texture_size(font: &FN2, text: &str) -> (u32, u32) {
    let mut width = 0;
    let mut height = 0;

    for c in text.chars() {
        let character_index = char_to_index(c);
        if character_index < INDEX_OFFSET {
            width += SPACE_WIDTH as u32;
        } else {
            let index = (character_index - INDEX_OFFSET) as usize;
            let character = &font[index];
            width += character.width;
            if character.height > height {
                height = character.height;
            }
        }
    }

    (width, height)
}

fn char_to_index(c: char) -> usize {
    (c as u8).into()
}
static INDEX_OFFSET: usize = 0x21;
static SPACE_WIDTH: u8 = 5;
static TEXT_SHADOW_PIXELS: u32 = 1;
