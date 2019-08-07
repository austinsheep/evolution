//! A module for creating triangles on a `ggez` window with a
//! seeking behavior

use ggez::{
    graphics,
    graphics::{DrawParam, Rect},
    nalgebra::{distance, Point2, Vector2},
    Context, GameResult,
};
use serde::Deserialize;

use super::food::Food;

const ANIMATION_FRAMES: [u8; 4] = [0, 1, 2, 1];

#[derive(Debug, Deserialize)]
pub struct FishConfig {
    pub quantity: u32,
    pub size_range: (f32, f32),
    pub max_speed_range: (f32, f32),
    pub max_steering_force_range: (f32, f32),
    pub total_food_chain_links: u8,
    pub frames_per_animation_frame: f32,
}

/// An entity that constists of attributes that determine its behavior.
pub struct Fish {
    animation_index: usize,
    frame_index: u8,
    dna: [f32; 2],
    health: f32,
    /// The scale of the fish, which ultimately be multiplied by the dimension ration to
    /// determine its actual dimensions
    pub scale: f32,
    /// The maximum velocity magnitude that the fish is able to reach
    pub max_speed: f32,
    /// The maximum steering force that is able to be applied to the fish
    pub max_steering_force: f32,
    /// The 2D position point
    pub pos: Point2<f32>,
    /// A radian angle that determines where the fish is pointed towards
    /// An angle of zero would point the fish towards the right
    /// This value is set to where ever the velocity vector is pointed towards
    pub angle: f32,
    /// The 2D velocity vector
    vel: Vector2<f32>,
    /// The 2D acceleration vector
    acc: Vector2<f32>,
}

impl Fish {
    /// Creates a new fish based on the provided optional attributes
    pub fn new(
        scale: f32,
        max_speed: f32,
        max_steering_force: f32,
        pos: Point2<f32>,
        angle: f32,
        dna: [f32; 2],
    ) -> Self {
        Self {
            animation_index: 0,
            frame_index: 0,
            scale,
            max_speed,
            max_steering_force,
            acc: Vector2::new(0.0, 0.0),
            vel: Vector2::new(0.0, 0.0),
            angle,
            pos,
            dna,
            health: 1.0,
        }
    }

    pub fn behaviors(&mut self, food: &mut Vec<Food>, poison: &mut Vec<Food>) {
        let mut food_steer = self.eat(food);
        let mut poison_steer = self.eat(poison);

        food_steer *= self.dna[0];
        poison_steer *= self.dna[1];

        // Apply the steering forces
        self.acc += food_steer + poison_steer;
    }

    pub fn eat(&mut self, food: &mut Vec<Food>) -> Vector2<f32> {
        let mut record = 0.0;
        let mut closest = None;
        for (i, item) in food.iter().enumerate() {
            let distance = distance(&item.pos, &self.pos);
            if closest.is_none() || distance < record {
                record = distance;
                closest = Some(i);
            }
        }

        if let Some(closest_index) = closest {
            let closest_item = &food[closest_index];

            if record <= closest_item.size + self.scale * 12.0 {
                food.remove(closest_index);
            } else {
                return self.seek(&closest_item.pos);
            }
        }
        Vector2::new(0.0, 0.0)
    }

    /// Applies a force that will point the fish towards its target
    pub fn seek(&mut self, target: &Point2<f32>) -> Vector2<f32> {
        // Get the desired velocity vector
        let mut desired = target - self.pos;
        // Set the magnitude of the desired vector to the maximum speed
        desired = desired.normalize() * self.max_speed;

        let mut steering_force = desired - self.vel;
        // Limit the steering force to the maximum value
        if steering_force.magnitude() > self.max_steering_force {
            steering_force = steering_force.normalize() * self.max_steering_force;
        }

        steering_force
    }

    /// Update the physics of the fish
    pub fn update(&mut self) {
        if self.health > 0.0 {
            // Limit the velocity magnitude to the maximum speed
            if self.vel.magnitude() > self.max_speed {
                self.vel = self.vel.normalize() * self.max_speed;
            }
            self.vel += self.acc;
            // Point the fish towards its velocity vector
            self.angle = self.vel.y.atan2(self.vel.x);
            self.pos += self.vel;
            self.acc *= 0.0;
            self.health -= 0.001;
        }
    }

    /// Draw the triangle that represents the fish
    pub fn draw(
        &mut self,
        ctx: &mut Context,
        image: &graphics::Image,
        frames_per_animation_frame: f32,
    ) -> GameResult {
        let parameters = DrawParam {
            src: Rect {
                x: 0.0,
                y: ANIMATION_FRAMES[self.animation_index] as f32 / 3.0,
                w: 1.0,
                h: 1.0 / 3.0,
            },
            dest: self.pos.into(),
            rotation: self.angle,
            scale: Vector2::new(self.scale, self.scale).into(),
            offset: Point2::new(0.5, 0.5).into(),
            color: [0.0, 1.0, 0.0, self.health].into(),
        };

        if self.frame_index as f32
            >= frames_per_animation_frame * self.max_speed / self.vel.magnitude()
        {
            self.frame_index = 0;
            self.animation_index += 1;
            self.animation_index %= 4;
        } else {
            self.frame_index += 1;
        }

        graphics::draw(ctx, image, parameters)?;
        Ok(())
    }
}
