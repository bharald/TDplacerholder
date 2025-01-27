use crate::{ Player, BLOCK_SIZE, };

use ggez::{
    graphics,
    Context,
    GameResult,
};

pub const WINDOW_HEIGHT: f32 = 600.0;
pub const WINDOW_WIDTH: f32 = 800.0;
pub const UI_HEIGHT: f32 = 180.0;

const GOLD_X: f32 = 30.0;
const GOLD_Y: f32 = 30.0;

const HP_X: f32 = 30.0;
const HP_Y: f32 = 50.0;

pub struct TowerIcon {}

pub struct UI {
    pub build_bar: Vec<TowerIcon>,
    pub selected_tile: Option<(f32, f32)>,
}

impl UI {
    pub fn draw(&mut self, ctx: &mut Context, player: &Player) -> GameResult {
        self.draw_background(ctx)?;
        self.draw_gold(ctx, player)?;
        self.draw_hp(ctx, player)?;
        // self.draw_tower_icons(ctx)?;
        self.draw_selected_tile(ctx)?;
        Ok(())
    }

    fn draw_background(&mut self, ctx: &mut Context) -> GameResult {
        let rectangle = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            [0.0, 0.0, WINDOW_WIDTH, UI_HEIGHT].into(),
            ggez::graphics::Color::new(0.2, 0.3, 0.4, 1.0),
        )?;

        let location = (ggez::mint::Point2 {
            x: 0.0,
            y: WINDOW_HEIGHT - UI_HEIGHT,
        },);
        graphics::draw(ctx, &rectangle, location)?;
        Ok(())
    }

    fn draw_gold(&mut self, ctx: &mut Context, player: &Player) -> GameResult {
        let text = graphics::Text::new(format!("GOLD: {}", player.gold));
        let location_x = GOLD_X;
        let location_y = WINDOW_HEIGHT - UI_HEIGHT + GOLD_Y;
        let location = (ggez::mint::Point2 {
            x: location_x,
            y: location_y,
        },);
        graphics::draw(ctx, &text, location)?;
        Ok(())
    }

    fn draw_hp(&mut self, ctx: &mut Context, player: &Player) -> GameResult {
        let text = graphics::Text::new(format!("HP: {}", player.health));
        let location_x = HP_X;
        let location_y = WINDOW_HEIGHT - UI_HEIGHT + HP_Y;
        let location = (ggez::mint::Point2 {
            x: location_x,
            y: location_y,
        },);
        graphics::draw(ctx, &text, location)?;
        Ok(())
    }

    fn draw_selected_tile(&mut self, ctx: &mut Context) -> GameResult {
        if let Some(tile) = self.selected_tile {
            let rectangle = graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::stroke(3.0),
                [0.0, 0.0, BLOCK_SIZE, BLOCK_SIZE].into(),
                ggez::graphics::Color::new(0.5, 0.0, 0.0, 1.0),
            )?;

            let location = (ggez::mint::Point2 {
                x: tile.0,
                y: tile.1,
            },);
            graphics::draw(ctx, &rectangle, location)?;
        }
        Ok(())
    }
}
