use crate::crates;
use crate::create_text_texture;
use crate::fn2::FN2;
use crate::sdl2::image::LoadTexture;
use crate::Context;
use crate::Texture;
use crate::Textures;
use sdl2::event::WindowEvent;
use sdl2::render::Canvas;
use sdl2::render::TextureCreator;
use sdl2::video::Window;
use sdl2::video::WindowContext;

fn refresh(context: &mut Context, window_size: (u32, u32)) {
    context.graphics.resolution_x = window_size.0;
    context.graphics.resolution_y = window_size.1;
    context.textures = get_textures(&mut context.canvas, context.texture_creator, &context.font);
}

pub fn resize(context: &mut Context, event: sdl2::event::WindowEvent) -> bool {
    match event {
        WindowEvent::Resized(w, h) => {
            refresh(context, (w as u32, h as u32));
            return true;
        }
        WindowEvent::Maximized => {
            refresh(context, context.canvas.window().size());
            return true;
        }
        _ => {
            return false;
        }
    }
}

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
