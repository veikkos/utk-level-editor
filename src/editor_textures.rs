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
                "place PL1 start point",
            ),
            p2_set_text_texture: create_text_texture(
                &mut context.canvas,
                &context.texture_creator,
                &context.font,
                "place PL2 start point",
            ),
            help_text_texture: create_text_texture(
                &mut context.canvas,
                &context.texture_creator,
                &context.font,
                "F1 for help",
            ),
            create_new_level_text_texture: create_text_texture(
                &mut context.canvas,
                &context.texture_creator,
                &context.font,
                "create new level?",
            ),
            wanna_quit_text_texture: create_text_texture(
                &mut context.canvas,
                &context.texture_creator,
                &context.font,
                "really wanna quit?",
            ),
            save_level_text_texture: create_text_texture(
                &mut context.canvas,
                &context.texture_creator,
                &context.font,
                "save level?",
            ),
            filename_text_texture: create_text_texture(
                &mut context.canvas,
                &context.texture_creator,
                &context.font,
                "filename:",
            ),
            press_y_text_texture: create_text_texture(
                &mut context.canvas,
                &context.texture_creator,
                &context.font,
                "press Y to confirm",
            ),
            new_level_x_size_text_texture: create_text_texture(
                &mut context.canvas,
                &context.texture_creator,
                &context.font,
                "x-size (>= 16 blocks):",
            ),
            new_level_y_size_text_texture: create_text_texture(
                &mut context.canvas,
                &context.texture_creator,
                &context.font,
                "y-size (>= 12 blocks):",
            ),
            spotlight_place_text_texture: create_text_texture(
                &mut context.canvas,
                &context.texture_creator,
                &context.font,
                "place spotlight (ESC to cancel)",
            ),
            spotlight_delete_text_texture: create_text_texture(
                &mut context.canvas,
                &context.texture_creator,
                &context.font,
                "delete spotlight (ESC to cancel)",
            ),
            spotlight_instructions_text_texture: create_text_texture(
                &mut context.canvas,
                &context.texture_creator,
                &context.font,
                "use UP and DOWN keys to adjust size, ENTER to accept",
            ),
            steam_place_text_texture: create_text_texture(
                &mut context.canvas,
                &context.texture_creator,
                &context.font,
                "place steam (ESC to cancel)",
            ),
            steam_delete_text_texture: create_text_texture(
                &mut context.canvas,
                &context.texture_creator,
                &context.font,
                "delete steam (ESC to cancel)",
            ),
            steam_instructions_text_texture: create_text_texture(
                &mut context.canvas,
                &context.texture_creator,
                &context.font,
                "UP/DOWN: range, LEFT/RIGHT: dir, ENTER to accept",
            ),
            create_shadows_enabled_instructions_text_texture: create_text_texture(
                &mut context.canvas,
                &context.texture_creator,
                &context.font,
                "disable auto shadow?",
            ),
            create_shadows_disabled_instructions_text_texture: create_text_texture(
                &mut context.canvas,
                &context.texture_creator,
                &context.font,
                "enable auto shadow?",
            ),
            place_normal_crate_text_texture: create_text_texture(
                &mut context.canvas,
                &context.texture_creator,
                &context.font,
                "place normal game crate",
            ),
            place_deathmatch_create_text_texture: create_text_texture(
                &mut context.canvas,
                &context.texture_creator,
                &context.font,
                "place deathmatch game crate",
            ),
            insert_crate_text_texture: create_text_texture(
                &mut context.canvas,
                &context.texture_creator,
                &context.font,
                "UP/DOWN/LEFT/RIGHT: select CRATE, ENTER to accept",
            ),
            delete_crate_text_texture: create_text_texture(
                &mut context.canvas,
                &context.texture_creator,
                &context.font,
                "delete crate",
            ),
        }
    }
}
