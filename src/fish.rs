//! A module for creating animated fish in a `ggez` window.
use ggez::{
    graphics,
    graphics::{DrawParam, Rect, Color},
    nalgebra::{distance, Point2, Vector2},
    Context, GameResult,
};
use rand::{Rng, rngs::ThreadRng};
use serde::Deserialize;

use super::{food::Food, inverse_map_range, Entity};

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
    pub quantity: usize,
    /// The constant radius around the fish where prey can be consumed
    pub eating_radius: f32,
    /// The frequency at which the fish's dna will mutate
    pub mutation_rate: f32,
    /// The range of scales of the fish.
    /// E.g. A scale of 2 would result in a fish twice as large as the original image.
    pub scale_range: (f32, f32),
    /// The range of maximum speeds for the fish
    pub max_speed_range: (f32, f32),
    /// The range of maximum turning forces for the fish
    pub max_steering_force_range: (f32, f32),
    /// The number of links in the food chain, excluding food
    pub total_food_chain_links: usize,
    /// The number of frames in the simulation that will go by before going to the next
    /// animation frame at the fish's maximum speed.
    pub frames_per_animation_frame: f32,
}

/// An entity that has the behavior of eating food and avoiding predators, along with basic physics.
//#[derive(Clone)]
pub struct Fish {
    /// The index of the current animation frame index stored in
    /// `ANIMATION_FRAMES`
    animation_index: usize,
    /// The current frame number of the window to determine when to update the animation frame
    /// specified from `FishConfig.frames_per_animation_frame`
    frame_index: u8,
    /// The DNA currently holds values for the weights of attraction and repulsion and the radii of perception
    /// for prey and predators respectively
    dna: [f32; 4],
    /// The rbg color of the fish
    color: (f32, f32, f32),
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
        fish_config: &FishConfig,
        group_index: &usize,
        window_size: &(f32, f32),
        rng: &mut ThreadRng,
    ) -> Self {
        // Scale is a random field between the specified range in `FishConfig`
        let scale_range = (fish_config.scale_range.1 - fish_config.scale_range.0)
            / fish_config.total_food_chain_links as f32;
        let min_scale = scale_range * *group_index as f32 + fish_config.scale_range.0;
        let max_scale = min_scale + scale_range;
        let scale = rng.gen_range(min_scale, max_scale);
        // Max speed and max steering force are values that are inversely
        // proportional to the scale value of the fish
        let max_speed =
            inverse_map_range(scale, fish_config.scale_range, fish_config.max_speed_range);
        let max_steering_force = inverse_map_range(
            scale,
            fish_config.scale_range,
            fish_config.max_steering_force_range,
        );
        // The angle is just a random radian around the unit circle
        let angle = rng.gen_range(0.0, 2.0 * std::f32::consts::PI);
        // The position is a random location in the window
        // TODO: In fullscreen mode, the window size may change on program
        // execution resulting in the fish and food spawning in a different area than the
        // window dimensions.
        let pos = Point2::new(
            rng.gen_range(0.0, window_size.0),
            rng.gen_range(0.0, window_size.1),
        );
        // The DNA currently holds random values for the weights against steering
        // towards food respectively.
        let dna = [
            // Food attraction weight
            rng.gen_range(-2.0, 2.0),
            // Predator attraction weight
            rng.gen_range(-2.0, 2.0),
            // Food perception radius
            rng.gen_range(10.0, 100.0),
            // Predator perception radius
            rng.gen_range(10.0, 100.0),
        ];
        let color = (
            rng.gen_range(0.0, 1.0),
            rng.gen_range(0.0, 1.0),
            rng.gen_range(0.0, 1.0),
        );
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
            color,
            health: 1.0,
        }
    }

    /// Creates a clone of a fish, with possible mutation(s) to the DNA
    pub fn clone(&self, rng: &mut ThreadRng, mutation_rate: f32) -> Self {
        // Possibly apply a mutation to genes in the cloned DNA, based on the `FishConfig.mutation_rate`
        let mut dna = self.dna.clone();
        for gene in dna.iter_mut() {
            if rng.gen_range(0.0, 1.0) < mutation_rate {
                *gene += rng.gen_range(-0.1, 0.1);
            }
        }

        Self {
            animation_index: 0,
            frame_index: 0,
            scale: self.scale,
            max_speed: self.max_speed,
            max_steering_force: self.max_steering_force,
            acc: Vector2::new(0.0, 0.0),
            vel: Vector2::new(0.0, 0.0),
            angle: rng.gen_range(0.0, 2.0 * std::f32::consts::PI),
            pos: self.pos,
            dna,
            color: self.color,
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
            color: Color::new(self.color.0, self.color.1, self.color.2, self.health),
        };

        // Determines if it's time to update the animation frame, based on
        // `FishConfig.frames_per_animation_frame`.
        if self.frame_index as f32
            >= frames_per_animation_frame * self.max_speed / self.vel.magnitude()
        {
            self.frame_index = 0;
            self.animation_index += 1;
            self.animation_index %= 4;
        // Doesn't increment the animation frame index if the fish isn't moving
        } else if self.vel != Vector2::new(0.0, 0.0) {
            self.frame_index += 1;
        }

        graphics::draw(ctx, image, parameters)?;

        Ok(())
    }

    /// Update the state of the fish in the simulation
    pub fn update(&mut self) {
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

    /// Applies the seeking behavior to the fish to eat prey and avoid predators.
    pub fn behave(
        &mut self,
        food: &mut Vec<Food>,
        prey: &mut [Vec<Self>],
        predator_positions: &Option<Vec<Point2<f32>>>,
        eating_radius: f32,
    ) {
        // Obtains the steering forces based on the nearest prey and predator that exist
        // within the respective perceptions (`self.dna[2]` and `self.dna[3]`)
        //
        // Then applies the weights of attraction for prey and predators respectively (`self.dna[0]` and `self.dna[1]`)
        let food_steer = self.eat(food, prey, eating_radius);
        let predator_steer = match predator_positions {
            Some(predator_positions) => self.avoid(predator_positions),
            None => Vector2::new(0.0, 0.0),
        };

        // Applying the steering forces
        self.acc += food_steer + predator_steer;
    }

    /// Determine the closest `Entity` in food and prey, and what the steering force should be applied to the
    /// `Fish` to head towards that `Entity`.
    /// Returns a the steering force of atraction for the `Entity`
    pub fn eat(
        &mut self,
        food: &mut Vec<Food>,
        prey: &mut [Vec<Self>],
        eating_radius: f32,
    ) -> Vector2<f32> {
        // The record distance of closest edible entity
        // The intial value of this variable is not considered.
        let mut record = 0.0;
        // An optional value that can hold the nearest entity's index
        let mut closest = None;
        // Find the nearest edible entity
        for (entity_index, entity) in food.iter().enumerate() {
            let distance = distance(&entity.pos(), &self.pos);
            if closest.is_none() || distance < record {
                record = distance;
                closest = Some((None, entity_index));
            }
        }
        for (group_index, prey_group) in prey.iter().enumerate() {
            for (entity_index, entity) in prey_group.iter().enumerate() {
                let distance = distance(&entity.pos(), &self.pos);
                if closest.is_none() || (distance < record && distance <= self.dna[2]) {
                    record = distance;
                    closest = Some((Some(group_index), entity_index));
                }
            }
        }

        if let Some((group_index, entity_index)) = closest {
            match group_index {
                Some(group_index) => {
                    return self.consume(
                        &mut prey[group_index],
                        entity_index,
                        record,
                        eating_radius,
                    )
                }
                None => return self.consume(food, entity_index, record, eating_radius),
            };
        }

        // If there was nothing edible nearby, the resulting steering force will be nothing.
        Vector2::new(0.0, 0.0)
    }

    /// Returns the steering force to head towards the provided entity.
    /// If this fish's radius is overlapping the provided entity's radius, then remove it
    /// from its collection.
    fn consume<E: Entity>(
        &mut self,
        entities: &mut Vec<E>,
        entity_index: usize,
        record: f32,
        eating_radius: f32,
    ) -> Vector2<f32> {
        let steer_force = self.seek(entities[entity_index].pos()) * self.dna[0];
        if record <= entities[entity_index].radius() + eating_radius {
            if self.health < 1.0 {
                self.health += 0.01;
            }
            entities.remove(entity_index);
        }
        steer_force
    }

    /// Determine the closest predator, and what the steering force should be applied to the
    /// `Fish` to avoid that predator
    pub fn avoid(&mut self, predator_positions: &Vec<Point2<f32>>) -> Vector2<f32> {
        // The record distance of closest predator
        // The intial value of this variable is not considered.
        let mut record = 0.0;
        // An optional value that can hold the nearest predator's index
        let mut closest = None;
        // Find the nearest predator
        for (i, predator_position) in predator_positions.iter().enumerate() {
            let distance = distance(predator_position, &self.pos);
            if closest.is_none() || distance < record {
                record = distance;
                closest = Some(i);
            }
        }

        if let Some(closest_index) = closest {
            let closest_predator = &predator_positions[closest_index];

            // Determines if the predator is perceived
            if record <= self.dna[3] {
                return self.seek(*closest_predator) * self.dna[1];
            }
        }

        // If there was no predator nearby, the resulting steering force will be nothing.
        Vector2::new(0.0, 0.0)
    }

    /// Bounds the fish to swim within the window based on the provided padding
    /// thickness.
    pub fn bound(&mut self, window_size: &(f32, f32), boundary_padding: f32) {
        let out_of_bounds = if self.pos.x < boundary_padding {
            true
        } else if self.pos.x > window_size.0 - boundary_padding {
            true
        } else if self.pos.y < boundary_padding {
            true
        } else if self.pos.y > window_size.1 - boundary_padding {
            true
        } else {
            false
        };

        if out_of_bounds {
            // The steering force needed to head towards the center of the window
            let center_steer = self.seek(Point2::new(window_size.0 / 2.0, window_size.1 / 2.0));

            self.acc += center_steer;
        }
    }

    /// Returns a force that will point the fish towards its target.
    pub fn seek(&mut self, target: Point2<f32>) -> Vector2<f32> {
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

    /// Returns whether or not this fish is alive
    pub fn is_alive(&self) -> bool {
        self.health >= 0.0
    }
}

impl Entity for Fish {
    /// Returns a reference to the fish's position
    fn pos(&self) -> Point2<f32> {
        self.pos
    }
    /// Returns the radius around the center of the fish at which it can interact with other entities
    fn radius(&self) -> f32 {
        self.scale * 12.0
    }
}
