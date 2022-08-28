use crate::crates::CrateClass;
use crate::level::DIFF_BULLETS;
use crate::level::DIFF_WEAPONS;
use crate::level::{StaticCrate, StaticCrateType};
use crate::types::*;
use crate::util::*;
use crate::Graphics;
use crate::Level;
use crate::Textures;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::render::Texture;
use sdl2::render::TextureQuery;
use sdl2::video::Window;
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

pub fn highlight_selected_tile(
    canvas: &mut Canvas<Window>,
    graphics: &Graphics,
    id: u32,
    color: &RendererColor,
) {
    let sdl_color = get_sdl_color(color);
    canvas.set_draw_color(sdl_color);

    let render_size = graphics.get_render_size();
    let render_multiplier = graphics.render_multiplier;
    let (x_logical, y_logical) = get_tile_coordinates(
        id,
        graphics.get_x_tiles_per_screen() * graphics.tile_size,
        graphics.tile_size,
    );
    let x = x_logical * render_multiplier;
    let y = y_logical * render_multiplier;

    draw_line(canvas, x, y, x, y + render_size - 1);
    draw_line(canvas, x, y, x + render_size - 1, y);
    draw_line(
        canvas,
        x + render_size - 1,
        y,
        x + render_size - 1,
        y + render_size - 1,
    );
    draw_line(
        canvas,
        x,
        y + render_size - 1,
        x + render_size - 1,
        y + render_size - 1,
    );
}

pub fn draw_line(canvas: &mut Canvas<Window>, x0: u32, y0: u32, x1: u32, y1: u32) {
    let x0_signed = x0 as i32;
    let y0_signed = y0 as i32;
    let x1_signed = x1 as i32;
    let y1_signed = y1 as i32;

    canvas
        .draw_line(
            Point::from((x0_signed, y0_signed)),
            Point::from((x1_signed, y1_signed)),
        )
        .unwrap();
}

pub fn render_text_texture(
    canvas: &mut Canvas<Window>,
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
    canvas.copy(&texture, None, dst).unwrap();
}

pub fn render_text_texture_coordinates(
    canvas: &mut Canvas<Window>,
    texture: &Texture,
    coordinates: (u32, u32),
    render_size: u32,
    scroll: Option<(u32, u32)>,
) {
    render_text_texture(
        canvas,
        texture,
        coordinates.0,
        coordinates.1,
        render_size,
        scroll,
    );
}

fn draw_circle(
    canvas: &mut Canvas<Window>,
    x_center: i32,
    y_center: i32,
    radius: u32,
    color: &RendererColor,
) {
    let sdl_color = get_sdl_color(color);
    canvas.set_draw_color(sdl_color);

    // https://stackoverflow.com/a/48291620
    let diameter: i32 = radius as i32 * 2;
    let mut x: i32 = radius as i32 - 1;
    let mut y: i32 = 0;
    let mut tx: i32 = 1;
    let mut ty: i32 = 1;
    let mut error: i32 = tx - diameter;

    while x >= y {
        canvas
            .draw_point(Point::new(x_center + x, y_center - y))
            .unwrap();
        canvas
            .draw_point(Point::new(x_center + x, y_center + y))
            .unwrap();
        canvas
            .draw_point(Point::new(x_center - x, y_center - y))
            .unwrap();
        canvas
            .draw_point(Point::new(x_center - x, y_center + y))
            .unwrap();
        canvas
            .draw_point(Point::new(x_center + y, y_center - x))
            .unwrap();
        canvas
            .draw_point(Point::new(x_center + y, y_center + x))
            .unwrap();
        canvas
            .draw_point(Point::new(x_center - y, y_center - x))
            .unwrap();
        canvas
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
    canvas: &mut Canvas<Window>,
    graphics: &Graphics,
    level: &Level,
    textures: &Textures,
    trigonometry: &Trigonometry,
) {
    canvas.set_draw_color(Color::from((0, 0, 0)));
    canvas.clear();
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
            canvas.copy(texture, src, dst).unwrap();
            let (shadow_texture_width, _shadow_texture_height) =
                get_texture_size(&textures.shadows);
            if level.tiles[y_index][x_index].shadow > 0 {
                let src = get_block(
                    level.tiles[y_index][x_index].shadow - 1,
                    shadow_texture_width,
                    graphics.tile_size,
                );
                canvas.copy(&textures.shadows, src, dst).unwrap();
            }
        }
    }
    for (coordinates, spotlight) in &level.spotlights {
        let (x_screen, y_screen) =
            get_screen_coordinates_from_level_coordinates(graphics, coordinates, &level.scroll);
        draw_circle(
            canvas,
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
            draw_circle(
                canvas,
                x_screen + (trigonometry.sin[steam.angle as usize] * multiplier) as i32,
                y_screen + (trigonometry.cos[steam.angle as usize] * multiplier) as i32,
                get_steam_render_radius() + x * 2,
                &RendererColor::Red,
            );
        }
    }

    render_crates(
        canvas,
        graphics,
        &level.scroll,
        &textures,
        &level.crates.staticc,
    );
}

fn render_crates(
    canvas: &mut Canvas<Window>,
    graphics: &Graphics,
    scroll: &(u32, u32),
    textures: &Textures,
    crates: &HashMap<(u32, u32), StaticCrateType>,
) {
    for (coordinates, crate_item) in crates {
        let box_size = get_crate_render_size();
        let (x_screen, y_screen) =
            get_screen_coordinates_from_level_coordinates(graphics, &coordinates, scroll);
        canvas.set_draw_color(get_sdl_color(match crate_item.crate_variant {
            StaticCrate::Normal => &RendererColor::LightGreen,
            StaticCrate::Deathmatch => &RendererColor::LightBlue,
        }));
        canvas
            .draw_rect(Rect::new(x_screen, y_screen, box_size, box_size))
            .unwrap();
        canvas
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
        render_text_texture(
            canvas,
            texture,
            (x_screen - 10) as u32,
            (y_screen - 9 - height as i32) as u32,
            graphics.get_render_size(),
            None,
        );
    }
}

pub fn render_and_wait(canvas: &mut Canvas<Window>) {
    canvas.present();
    ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
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
