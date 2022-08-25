extern crate sdl2;

use crate::context::Textures;
use crate::context_util::get_textures;
use crate::fn2::create_text_texture;
use crate::fn2::load_font;
use crate::graphics::Graphics;
use crate::level::Level;
use crate::types::NextMode::*;
use sdl2::image::InitFlag;
use sdl2::render::Texture;
mod context;
mod context_util;
mod crates;
mod editor;
mod general_level_info;
mod help;
mod level;
mod load_level;
mod random_item_editor;
mod render;
mod tile_selector;
mod types;
mod util;
use context::Context;
use types::*;
use util::*;
mod editor_textures;
mod fn2;
mod graphics;

pub fn main() {
    let sdl = sdl2::init().unwrap();
    let _image_context = sdl2::image::init(InitFlag::PNG);
    let video_subsystem = sdl.video().unwrap();
    let graphics = Graphics::new();
    let window = video_subsystem
        .window(
            "Ultimate Tapan Kaikki - Level Editor",
            graphics.resolution_x,
            graphics.resolution_y,
        )
        .position_centered()
        .resizable()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();
    let font = load_font("./assets/TETRIS.FN2");
    let textures = get_textures(&mut canvas, &texture_creator, &font);
    let mut context = Context {
        sdl,
        graphics,
        canvas,
        texture_creator: &texture_creator,
        font,
        textures,
        level: Level::get_default_level((32, 22)),
        selected_tile_id: 0,
        texture_type_selected: TextureType::FLOOR,
        texture_type_scrolled: TextureType::FLOOR,
        mouse: (0, 0),
        level_save_name: String::new(),
        trigonometry: Trigonometry::new(),
        automatic_shadows: true,
    };

    let mut next_mode = NextMode::Editor;

    'running: loop {
        next_mode = match next_mode {
            Editor => editor::exec(&mut context),
            TileSelect => tile_selector::exec(&mut context),
            Help => help::exec(&mut context),
            GeneralLevelInfo => general_level_info::exec(&mut context),
            RandomItemEditor(game_type) => random_item_editor::exec(&mut context, game_type),
            LoadLevel => load_level::exec(&mut context),
            Quit => break 'running,
        }
    }
}
