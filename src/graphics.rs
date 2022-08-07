pub struct Graphics {
    pub tile_size: u32,
    pub render_multiplier: u32,
    pub resolution_x: u32,
    pub resolution_y: u32,
}

impl Graphics {
    const TILE_SIZE: u32 = 20;
    const RENDER_MULTIPLIER: u32 = 2;
    const RESOLUTION_X: u32 = 1280;
    const RESOLUTION_Y: u32 = 720;

    pub fn new() -> Graphics {
        Graphics {
            tile_size: Graphics::TILE_SIZE,
            render_multiplier: Graphics::RENDER_MULTIPLIER,
            resolution_x: Graphics::RESOLUTION_X,
            resolution_y: Graphics::RESOLUTION_Y,
        }
    }

    pub fn get_render_size(&self) -> u32 {
        self.tile_size * self.render_multiplier
    }

    pub fn get_x_tiles_per_screen(&self) -> u32 {
        self.resolution_x / self.get_render_size()
    }

    pub fn get_y_tiles_per_screen(&self) -> u32 {
        self.resolution_y / self.get_render_size()
    }
}
