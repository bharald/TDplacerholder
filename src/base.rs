use crate::{
    BLOCK_SIZE,
    asset_manager::AssetManager,
};

use ggez::{
    graphics,
    GameResult,
    Context,
};

pub const BASE_SIZE: f32 = 60.0;
pub const BASE_PADDING: f32 = 5.0;

pub struct Base {
    pub pos: [f32; 2],
}

impl Base {
    pub fn draw(&mut self, ctx: &mut Context, asset_manager: &AssetManager) -> GameResult {
        let location = (
            ggez::mint::Point2 {
                x: self.pos[0] * BLOCK_SIZE + BASE_PADDING,
                y: self.pos[1] * BLOCK_SIZE + BASE_SIZE / 2.0 - BASE_PADDING,
            },
        );

        graphics::draw(ctx, &asset_manager.base_sprite, location)?;

        Ok(())
    }
}
