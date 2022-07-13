extern crate sdl2;

use sdl2::event::Event;
use sdl2::image::{InitFlag, LoadTexture};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use std::time::Duration;

mod level;

const TILE_SIZE: u32 = 20;
const RENDER_MULTIPLIER: u32 = 2;
const RENDER_SIZE: u32 = TILE_SIZE * RENDER_MULTIPLIER;

fn get_tile_coordinates(id: u32) -> (u32, u32) {
    let x = id * TILE_SIZE % 320;
    let y = id * TILE_SIZE / 320 * TILE_SIZE;
    (x, y)
}

fn get_block(id: u32) -> Rect {
    let (x, y) = get_tile_coordinates(id);
    Rect::new(x as i32, y as i32, TILE_SIZE, TILE_SIZE)
}

fn highlight_selected_tile(id: u32, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
    canvas.set_draw_color(Color::from((255, 255, 255)));

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

fn draw_line(
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    x0: u32,
    y0: u32,
    x1: u32,
    y1: u32,
) {
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

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG);
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Ultimate Tapan Kaikki - Level Editor", 640, 480)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let texture_creator = canvas.texture_creator();
    let texture = texture_creator.load_texture("FLOOR1.PNG").unwrap();

    let mut level = [[0u32; 16]; 12];
    init_empty_level(&mut level);

    // Test level export
    level::serialize("./TEST.LEV", level).unwrap();

    let mut tile_select_mode = false;

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::Space),
                    ..
                } => {
                    tile_select_mode = !tile_select_mode;
                }
                _ => {}
            }
        }

        if tile_select_mode {
            canvas.copy(&texture, None, None).unwrap();
        } else {
            render_level(level, &mut canvas, &texture);
        }

        highlight_selected_tile(0, &mut canvas);
        highlight_selected_tile(1, &mut canvas);
        highlight_selected_tile(16, &mut canvas);
        highlight_selected_tile(17, &mut canvas);
        highlight_selected_tile(15, &mut canvas);
        canvas.present();

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

fn render_level(
    level: [[u32; 16]; 12],
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    texture: &sdl2::render::Texture,
) {
    for y in 0..level.len() {
        for x in 0..level[0].len() {
            let src = get_block(level[y][x]);
            let dst = Rect::new(
                (x * RENDER_SIZE as usize).try_into().unwrap(),
                (y * RENDER_SIZE as usize).try_into().unwrap(),
                RENDER_SIZE,
                RENDER_SIZE,
            );
            canvas.copy(&texture, src, dst).unwrap();
        }
    }
}

fn init_empty_level(level: &mut [[u32; 16]; 12]) {
    for x in 0..level[0].len() {
        level[0][x] = 1;
    }
    for y in 1..(level.len() - 1) {
        for x in 0..level[0].len() {
            if x == 0 || x == level[0].len() - 1 {
                level[y][x] = 1;
            } else {
                level[y][x] = 0;
            }
        }
    }
    for x in 0..level[0].len() {
        level[level.len() - 1][x] = 1;
    }
}
