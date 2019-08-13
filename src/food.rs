//! A module for creating circles in a `ggez` window.

use ggez::{graphics, nalgebra::Point2, Context, GameResult};
use serde::Deserialize;

use super::Entity;

/// The configuration structure specifically for food that is read and deserialized from
/// `config.ron`
#[derive(Debug, Deserialize)]
pub struct FoodConfig {
    /// The amount of food in the simulation
    pub quantity: u32,
    /// The range of radii of the food.
    pub radius_range: (f32, f32),
}

/// An edible entity for fish
pub struct Food {
    /// The radius of the displayed circle, representing the piece of food.
    pub radius: f32,
    /// The 2D position of the food (the food's location is in relation to its center)
    pub pos: Point2<f32>,
    /// The RGBA color of the food.
    pub color: [f32; 4],
}

impl Food {
    pub fn new(pos: Point2<f32>) -> Self {
        Self {
            radius: 5.0,
            // The position is a random location in the window
            pos,
            // The color is a slightly transparent green
            color: /*[
                rng.gen_range(0.0, 1.0),
                rng.gen_range(0.0, 1.0),
                rng.gen_range(0.0, 1.0),
                0.8
            ]*/[0.0, 1.0, 0.0, 0.8]
        }
    }

    /// Draws the circle representing the piece of food in the `ggez` window
    pub fn draw(&self, ctx: &mut Context) -> GameResult {
        let circle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            Point2::new(0.0, 0.0),
            self.radius,
            1.0,
            self.color.into(),
        )?;

        graphics::draw(ctx, &circle, (self.pos,))?;
        Ok(())
    }
}

impl Entity for Food {
    /// Returns a reference to the piece of food's position
    fn pos(&self) -> Point2<f32> {
        self.pos
    }
    /// Returns the radius of the piece of food
    fn radius(&self) -> f32 {
        self.radius
    }
}
