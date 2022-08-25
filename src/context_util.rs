use crate::crates;
use crate::create_text_texture;
use crate::fn2::FN2;
use crate::sdl2::image::LoadTexture;
use crate::Texture;
use crate::Textures;
use sdl2::render::Canvas;
use sdl2::render::TextureCreator;
use sdl2::video::Window;
use sdl2::video::WindowContext;

pub fn get_textures<'a>(
    canvas: &mut Canvas<Window>,
    texture_creator: &'a TextureCreator<WindowContext>,
    font: &FN2,
) -> Textures<'a> {
    let selected_icon = create_text_texture(canvas, &texture_creator, &font, "*");
    let crate_textures: Vec<Texture> = crates::get_crates()
        .iter()
        .flatten()
        .map(|name| create_text_texture(canvas, &texture_creator, &font, name))
        .collect();

    Textures {
        floor: texture_creator.load_texture("./assets/FLOOR1.PNG").unwrap(),
        walls: texture_creator.load_texture("./assets/WALLS1.PNG").unwrap(),
        shadows: texture_creator
            .load_texture("./assets/SHADOWS_ALPHA.PNG")
            .unwrap(),
        selected_icon,
        saved_level_name: None,
        crates: crate_textures,
    }
}
