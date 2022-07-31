use crate::create_text_texture;
use crate::Context;
use sdl2::render::Texture;

pub struct EditorTextures<'a> {
    pub p1_text_texture: Texture<'a>,
    pub p2_text_texture: Texture<'a>,
    pub p1_set_text_texture: Texture<'a>,
    pub p2_set_text_texture: Texture<'a>,
    pub help_text_texture: Texture<'a>,
    pub create_new_level_text_texture: Texture<'a>,
    pub wanna_quit_text_texture: Texture<'a>,
    pub save_level_text_texture: Texture<'a>,
    pub filename_text_texture: Texture<'a>,
    pub press_y_text_texture: Texture<'a>,
    pub new_level_x_size_text_texture: Texture<'a>,
    pub new_level_y_size_text_texture: Texture<'a>,
    pub spotlight_place_text_texture: Texture<'a>,
    pub spotlight_delete_text_texture: Texture<'a>,
    pub spotlight_instructions_text_texture: Texture<'a>,
    pub steam_place_text_texture: Texture<'a>,
    pub steam_delete_text_texture: Texture<'a>,
    pub steam_instructions_text_texture: Texture<'a>,
    pub create_shadows_enabled_instructions_text_texture: Texture<'a>,
    pub create_shadows_disabled_instructions_text_texture: Texture<'a>,
    pub place_normal_crate_text_texture: Texture<'a>,
    pub place_deathmatch_create_text_texture: Texture<'a>,
    pub insert_crate_text_texture: Texture<'a>,
    pub delete_crate_text_texture: Texture<'a>,
}

impl EditorTextures<'_> {
    pub fn new<'a>(context: &mut Context<'a>) -> EditorTextures<'a> {
        EditorTextures {
            p1_text_texture: create_text_texture(
                &mut context.canvas,
                &context.texture_creator,
                &context.font,
                "PL1",
            ),
            p2_text_texture: create_text_texture(
                &mut context.canvas,
                &context.texture_creator,
                &context.font,
                "PL2",
            ),
            p1_set_text_texture: create_text_texture(
                &mut context.canvas,
                &context.texture_creator,
                &context.font,
                "PLACE PL1 START POINT",
            ),
            p2_set_text_texture: create_text_texture(
                &mut context.canvas,
                &context.texture_creator,
                &context.font,
                "PLACE PL2 START POINT",
            ),
            help_text_texture: create_text_texture(
                &mut context.canvas,
                &context.texture_creator,
                &context.font,
                "F1 FOR HELP",
            ),
            create_new_level_text_texture: create_text_texture(
                &mut context.canvas,
                &context.texture_creator,
                &context.font,
                "CREATE NEW LEVEL?",
            ),
            wanna_quit_text_texture: create_text_texture(
                &mut context.canvas,
                &context.texture_creator,
                &context.font,
                "REALLY WANNA QUIT?",
            ),
            save_level_text_texture: create_text_texture(
                &mut context.canvas,
                &context.texture_creator,
                &context.font,
                "SAVE LEVEL?",
            ),
            filename_text_texture: create_text_texture(
                &mut context.canvas,
                &context.texture_creator,
                &context.font,
                "FILENAME:",
            ),
            press_y_text_texture: create_text_texture(
                &mut context.canvas,
                &context.texture_creator,
                &context.font,
                "PRESS Y TO CONFIRM",
            ),
            new_level_x_size_text_texture: create_text_texture(
                &mut context.canvas,
                &context.texture_creator,
                &context.font,
                "X-SIZE (>= 16 BLOCKS):",
            ),
            new_level_y_size_text_texture: create_text_texture(
                &mut context.canvas,
                &context.texture_creator,
                &context.font,
                "Y-SIZE (>= 12 BLOCKS):",
            ),
            spotlight_place_text_texture: create_text_texture(
                &mut context.canvas,
                &context.texture_creator,
                &context.font,
                "PLACE SPOTLIGHT (ESC TO CANCEL)",
            ),
            spotlight_delete_text_texture: create_text_texture(
                &mut context.canvas,
                &context.texture_creator,
                &context.font,
                "DELETE SPOTLIGHT (ESC TO CANCEL)",
            ),
            spotlight_instructions_text_texture: create_text_texture(
                &mut context.canvas,
                &context.texture_creator,
                &context.font,
                "USE UP AND DOWN KEYS TO ADJUST SIZE, ENTER TO ACCEPT",
            ),
            steam_place_text_texture: create_text_texture(
                &mut context.canvas,
                &context.texture_creator,
                &context.font,
                "PLACE STEAM (ESC TO CANCEL)",
            ),
            steam_delete_text_texture: create_text_texture(
                &mut context.canvas,
                &context.texture_creator,
                &context.font,
                "DELETE STEAM (ESC TO CANCEL)",
            ),
            steam_instructions_text_texture: create_text_texture(
                &mut context.canvas,
                &context.texture_creator,
                &context.font,
                "UP/DOWN: RANGE, LEFT/RIGHT: DIR, ENTER TO ACCEPT",
            ),
            create_shadows_enabled_instructions_text_texture: create_text_texture(
                &mut context.canvas,
                &context.texture_creator,
                &context.font,
                "DISABLE AUTO SHADOW?",
            ),
            create_shadows_disabled_instructions_text_texture: create_text_texture(
                &mut context.canvas,
                &context.texture_creator,
                &context.font,
                "ENABLE AUTO SHADOW?",
            ),
            place_normal_crate_text_texture: create_text_texture(
                &mut context.canvas,
                &context.texture_creator,
                &context.font,
                "PLACE NORMAL GAME CRATE",
            ),
            place_deathmatch_create_text_texture: create_text_texture(
                &mut context.canvas,
                &context.texture_creator,
                &context.font,
                "PLACE DEATHMATCH GAME CRATE",
            ),
            insert_crate_text_texture: create_text_texture(
                &mut context.canvas,
                &context.texture_creator,
                &context.font,
                "UP/DOWN/LEFT/RIGHT: SELECT CRATE, ENTER TO ACCEPT",
            ),
            delete_crate_text_texture: create_text_texture(
                &mut context.canvas,
                &context.texture_creator,
                &context.font,
                "DELETE CRATE",
            ),
        }
    }
}
