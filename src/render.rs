use crate::crates::CrateClass;
use crate::level::DIFF_BULLETS;
use crate::level::DIFF_WEAPONS;
use crate::level::{StaticCrate, StaticCrateType};
use crate::types::*;
use crate::util::*;
use crate::Level;
use crate::Textures;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::render::Texture;
use sdl2::render::TextureCreator;
use sdl2::render::TextureQuery;
use sdl2::ttf::Font;
use sdl2::video::Window;
use sdl2::video::WindowContext;
use std::collections::HashMap;
use std::time::Duration;

const TEXT_SIZE_DIVIDER: u32 = 1;

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

pub fn highlight_selected_tile(canvas: &mut Canvas<Window>, id: u32, color: &RendererColor) {
    let sdl_color = get_sdl_color(color);
    canvas.set_draw_color(sdl_color);

    let (x_logical, y_logical) = get_tile_coordinates(id);
    let x = x_logical * RENDER_MULTIPLIER;
    let y = y_logical * RENDER_MULTIPLIER;

    draw_line(canvas, x, y, x, y + RENDER_SIZE - 1);
    draw_line(canvas, x, y, x + RENDER_SIZE - 1, y);
    draw_line(
        canvas,
        x + RENDER_SIZE - 1,
        y,
        x + RENDER_SIZE - 1,
        y + RENDER_SIZE - 1,
    );
    draw_line(
        canvas,
        x,
        y + RENDER_SIZE - 1,
        x + RENDER_SIZE - 1,
        y + RENDER_SIZE - 1,
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

pub fn get_font_texture<'a>(
    texture_creator: &'a TextureCreator<WindowContext>,
    font: &Font,
    text: &str,
) -> Texture<'a> {
    let surface = font
        .render(text)
        .blended(Color::RGBA(255, 0, 0, 255))
        .map_err(|e| e.to_string())
        .unwrap();
    texture_creator
        .create_texture_from_surface(&surface)
        .map_err(|e| e.to_string())
        .unwrap()
}

pub fn render_text_texture(
    canvas: &mut Canvas<Window>,
    texture: &Texture,
    x: u32,
    y: u32,
    scroll: Option<(u32, u32)>,
) {
    let TextureQuery { width, height, .. } = texture.query();
    let scroll = scroll.unwrap_or((0, 0));
    let dst = Rect::new(
        x as i32 - (scroll.0 * RENDER_SIZE) as i32,
        y as i32 - (scroll.1 * RENDER_SIZE) as i32,
        width / TEXT_SIZE_DIVIDER,
        height / TEXT_SIZE_DIVIDER,
    );
    canvas.copy(&texture, None, dst).unwrap();
}

pub fn render_text_texture_coordinates(
    canvas: &mut Canvas<Window>,
    texture: &Texture,
    coordinates: (u32, u32),
    scroll: Option<(u32, u32)>,
) {
    render_text_texture(canvas, texture, coordinates.0, coordinates.1, scroll);
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
    level: &Level,
    textures: &Textures,
    trigonometry: &Trigonometry,
) {
    for y in 0..TILES_Y_PER_SCREEN {
        for x in 0..TILES_X_PER_SCREEN {
            let (x_index, y_index) = get_scroll_corrected_indexes(level.scroll, x, y);
            let src = get_block(level.tiles[y_index][x_index].id);
            let (x_absolute, y_absolute) = get_absolute_coordinates_from_logical(x, y);
            let dst = Rect::new(x_absolute, y_absolute, RENDER_SIZE, RENDER_SIZE);
            let texture = match level.tiles[y_index][x_index].texture_type {
                TextureType::FLOOR => &textures.floor,
                TextureType::WALLS => &textures.walls,
                TextureType::SHADOW => unreachable!(),
            };
            canvas.copy(texture, src, dst).unwrap();
            if level.tiles[y_index][x_index].shadow > 0 {
                let src = get_block(level.tiles[y_index][x_index].shadow - 1);
                canvas.copy(&textures.shadows, src, dst).unwrap();
            }
        }
    }
    for (coordinates, spotlight) in &level.spotlights {
        let (x_screen, y_screen) =
            get_screen_coordinates_from_level_coordinates(coordinates, &level.scroll);
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
            get_screen_coordinates_from_level_coordinates(coordinates, &level.scroll);
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

    render_crates(canvas, &level.scroll, &textures, &level.crates.staticc);
}

fn render_crates(
    canvas: &mut Canvas<Window>,
    scroll: &(u32, u32),
    textures: &Textures,
    crates: &HashMap<(u32, u32), StaticCrateType>,
) {
    for (coordinates, crate_item) in crates {
        let box_size = get_crate_render_size();
        let (x_screen, y_screen) =
            get_screen_coordinates_from_level_coordinates(&coordinates, scroll);
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
            (y_screen - 5 - height as i32) as u32,
            None,
        );
    }
}

pub fn render_and_wait(canvas: &mut Canvas<Window>) {
    canvas.present();
    ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
}

pub fn get_texture_rect(texture: &Texture) -> Rect {
    let TextureQuery { width, height, .. } = texture.query();
    Rect::new(0, 0, width * RENDER_MULTIPLIER, height * RENDER_MULTIPLIER)
}
