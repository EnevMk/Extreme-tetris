use ggez::graphics;
use ggez::{Context, GameResult};
//use crate::assets::*;

enum FigureType {

    L, J, O, S, Z, I, T
}

struct Figure {

    kind: FigureType,
    col: u8,
    row: u8,
}

impl Figure {

    pub fn draw(&self, ctx: &mut Context, assets: &Assets) -> GameResult<()> {
        Ok(())
    }
}

pub struct Assets {

    pub orange_sq: graphics::Image,
    pub field_sq: graphics::Image,
}

impl Assets {
    pub fn new(ctx: &mut Context) -> GameResult<Assets> {

        let orange_sq = graphics::Image::new(ctx, "/l.png")?;
        let field_sq  = graphics::Image::new(ctx, "/field.png")?;

        Ok(Assets {orange_sq, field_sq})
    }
}