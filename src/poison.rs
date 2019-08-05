//! A module for creating circles in a `ggez` window

use ggez::{
    graphics,
    nalgebra::Point2,
    Context, GameResult,
};

pub struct Poison {
    pub size: f32,
    pub pos: Point2<f32>,
    pub color: [f32; 4],
}

impl Poison {
    pub fn draw(&self, ctx: &mut Context) -> GameResult {
        let circle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            Point2::new(0.0, 0.0),
            self.size,
            1.0,
            self.color.into(),
        )?;

        graphics::draw(ctx, &circle, (self.pos,))?;
        Ok(())
    }
}
