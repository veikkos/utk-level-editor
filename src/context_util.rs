use crate::fn2::FN2;
use crate::Context;
use crate::Texture;
use crate::Textures;
use crate::{crates, Renderer};
use sdl2::event::WindowEvent;

fn refresh<'a>(renderer: &'a Renderer, context: &mut Context<'a>, window_size: (u32, u32)) {
    context.graphics.resolution_x = window_size.0;
    context.graphics.resolution_y = window_size.1;
    context.textures = get_textures(renderer, &context.font);
}

pub fn resize<'a>(
    renderer: &'a Renderer,
    context: &mut Context<'a>,
    event: sdl2::event::WindowEvent,
) -> bool {
    match event {
        WindowEvent::Resized(w, h) => {
            refresh(renderer, context, (w as u32, h as u32));
            return true;
        }
        WindowEvent::Maximized => {
            refresh(renderer, context, renderer.window_size());
            return true;
        }
        _ => {
            return false;
        }
    }
}

pub fn get_textures<'a>(renderer: &'a Renderer, font: &FN2) -> Textures<'a> {
    let selected_icon = renderer.create_text_texture(&font, "*");
    let crate_textures: Vec<Texture> = crates::get_crates()
        .iter()
        .flatten()
        .map(|name| renderer.create_text_texture(&font, name))
        .collect();

    Textures {
        floor: renderer.load_texture("./assets/FLOOR1.PNG"),
        walls: renderer.load_texture("./assets/WALLS1.PNG"),
        shadows: renderer.load_texture("./assets/SHADOWS_ALPHA.PNG"),
        selected_icon,
        saved_level_name: None,
        crates: crate_textures,
    }
}
