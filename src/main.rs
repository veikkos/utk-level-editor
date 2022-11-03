extern crate sdl2;

use crate::context::Textures;
use crate::context_util::get_textures;
use crate::fn2::load_font;
use crate::graphics::Graphics;
use crate::level::Level;
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
use crate::editor::EditorState;
use crate::general_level_info::GeneralLevelInfoState;
use crate::help::HelpState;
use crate::load_level::LoadLevelState;
use crate::random_item_editor::RandomItemEditorState;
use crate::render::Renderer;
use crate::tile_selector::TileSelectState;
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
    let renderer = Renderer::new(window);
    let font = load_font("./assets/TETRIS.FN2");
    let textures = get_textures(&renderer, &font);
    let mut context = Context {
        sdl,
        graphics,
        font,
        textures,
        level: Level::get_default_level((32, 22)),
        selected_tile_id: 0,
        texture_type_selected: TextureType::Floor,
        texture_type_scrolled: TextureType::Floor,
        mouse: (0, 0),
        level_save_name: String::new(),
        trigonometry: Trigonometry::new(),
        automatic_shadows: true,
    };

    let mut editor = EditorState::new(&renderer, &context);
    let tile_select = TileSelectState::new(&renderer, &context);
    let help = HelpState::new(&renderer, &context);
    let mut general_level_info = GeneralLevelInfoState::new(&renderer, &context);
    let mut random_item_editor = RandomItemEditorState::new(&renderer, &context);
    let mut load_level = LoadLevelState::new(&renderer, &context);

    let mut mode = Mode::Editor;
    loop {
        mode = match mode {
            Mode::Editor => editor.frame(&mut context),
            Mode::TileSelect => tile_select.frame(&mut context),
            Mode::Help => help.frame(&mut context),
            Mode::GeneralLevelInfo => general_level_info.frame(&mut context),
            Mode::RandomItemEditor(game_type) => random_item_editor.frame(&mut context, game_type),
            Mode::LoadLevel => load_level.frame(&mut context),
            Mode::Quit => break,
        };
    }
}
