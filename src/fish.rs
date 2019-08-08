//! A module for creating animated fish in a `ggez` window.

use ggez::{
    graphics,
    graphics::{DrawParam, Rect},
    nalgebra::{distance, Point2, Vector2},
    Context, GameResult,
};
use serde::Deserialize;

use super::food::Food;

/// The indicies of each animation frame for the fish.
///
/// The order of the animation frames will be from the beginning of the array to the end. Ultimately specifying which
/// animation frame to switch to next and looping back to the beginning of the array.
const ANIMATION_FRAMES: [u8; 4] = [0, 1, 2, 1];

/// The configuration structure specifically for fish that is read and deserialized from
/// `config.ron`
#[derive(Debug, Deserialize)]
pub struct FishConfig {
    /// The number of fish in the simulation
    pub quantity: u32,
    /// The range of scales of the fish.
    /// E.g. A scale of 2 would result in a fish twice as large as the original image.
    pub scale_range: (f32, f32),
    /// The range of maximum speeds for the fish
    pub max_speed_range: (f32, f32),
    /// The range of maximum turning forces for the fish
    pub max_steering_force_range: (f32, f32),
    /// The number of links in the food chain, excluding food
    ///
    /// TODO: still need to add predetors
    pub total_food_chain_links: u8,
    /// The number of frames in the simulation that will go by before going to the next
    /// animation frame at the fish's maximum speed.
    pub frames_per_animation_frame: f32,
}

/// An entity that has the behavior of eating food and poison, along with basic physics.
pub struct Fish {
    /// The index of the current animation frame index stored in
    /// `ANIMATION_FRAMES`
    animation_index: usize,
    /// The current frame number of the window to determine when to update the animation frame
    /// specified from `FishConfig.frames_per_animation_frame`
    frame_index: u8,
    //// The DNA currently holds values for the weights against steering
    //// towards food and poison respectively.
    dna: [f32; 2],
    /// The health of the fish starts at 1 (full) and will decline by 0.001 per frame.
    /// A health of 0 or lower will result in an invisible fish.
    /// The opacity of a fish is dependant on its health.
    health: f32,
    /// The scale of the fish.
    /// E.g. A scale of 2 would result in a fish twice as large as the original image.
    scale: f32,
    /// The maximum velocity magnitude that the fish is able to reach
    max_speed: f32,
    /// The maximum steering/turning force that is able to be applied to the fish.
    max_steering_force: f32,
    /// The 2D position of the fish (the fish's location is in relation to its center)
    pos: Point2<f32>,
    /// A radian angle that determines where the fish is pointed towards.
    /// An angle of zero would point the fish towards the right.
    /// This value is set to wherever the velocity vector is pointed towards.
    angle: f32,
    /// The 2D velocity vector.
    vel: Vector2<f32>,
    /// The 2D acceleration vector.
    acc: Vector2<f32>,
}

impl Fish {
    /// Creates a new fish based on the provided and default attributes.
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

    /// Draw the image that represents the fish, and animates it.
    pub fn draw(
        &mut self,
        ctx: &mut Context,
        image: &graphics::Image,
        frames_per_animation_frame: f32,
    ) -> GameResult {
        // Specifies what animation frame to display, and how to display it.
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

        // Determines if it's time to update the animation frame, based on
        // `FishConfig.frames_per_animation_frame`.
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

    /// Update the state of the fish in the simulation
    pub fn update(&mut self) {
        // Only update fish with a positive health value.
        if self.health > 0.0 {
            // Limit the velocity magnitude to the maximum speed.
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

    /// Applies the seeking behavior to the fish to consume food and poison
    pub fn behaviors(&mut self, food: &mut Vec<Food>, poison: &mut Vec<Food>) {
        // Obtains the steering forces based on the nearest peice of food and poison.
        let mut food_steer = self.eat(food);
        let mut poison_steer = self.eat(poison);

        // Multiplies the weights in the DNA of the fish by the previously-obtained steering forces
        food_steer *= self.dna[0];
        poison_steer *= self.dna[1];

        // Applying the steering forces
        self.acc += food_steer + poison_steer;
    }

    /// Determine the closest piece of `Food`, and what the steering force should be applied to the
    /// `Fish` to head towards that piece `Food`.
    pub fn eat(&mut self, food: &mut Vec<Food>) -> Vector2<f32> {
        // The record distance of closest peice of food.
        // The intial value of this variable is not considered.
        let mut record = 0.0;
        // An optional value that can hold the nearest piece of food's index in the vector holding
        // it.
        let mut closest = None;
        // Find the nearest peice of food
        for (i, item) in food.iter().enumerate() {
            let distance = distance(&item.pos, &self.pos);
            if closest.is_none() || distance < record {
                record = distance;
                closest = Some(i);
            }
        }

        if let Some(closest_index) = closest {
            let closest_item = &food[closest_index];

            // Determines if the piece of `Food` is close enough to be eaten, or has the fish seek it
            // otherwise.
            if record <= closest_item.radius + self.scale * 12.0 {
                food.remove(closest_index);
            } else {
                return self.seek(&closest_item.pos);
            }
        }

        // If there was no `Food` in the provided vector, the resulting steering force will be nothing.
        Vector2::new(0.0, 0.0)
    }

    /// Returns a force that will point the fish towards its target.
    pub fn seek(&mut self, target: &Point2<f32>) -> Vector2<f32> {
        // Get the desired velocity vector.
        let mut desired = target - self.pos;
        // Set the magnitude of the desired vector to the maximum speed.
        desired = desired.normalize() * self.max_speed;

        let mut steering_force = desired - self.vel;
        // Limit the steering force to the maximum value.
        if steering_force.magnitude() > self.max_steering_force {
            steering_force = steering_force.normalize() * self.max_steering_force;
        }

        steering_force
    }
}
