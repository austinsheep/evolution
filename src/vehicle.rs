//! A module for creating triangles on a `ggez` window with a
//! seeking behavior

use ggez::{
    graphics,
    graphics::DrawParam,
    nalgebra::{Point2, Vector2},
    Context, GameResult,
};

/// The ratio of the dimensions of the vehicle's appearance (width, height)
const DIMENSION_RATIO: (u8, u8) = (2, 1);

/// An entity that constists of attributes that determine its behavior.
pub struct Vehicle {
    /// The size of the vehicle, which ultimately be multiplied by the dimension ration to
    /// determine its actual dimensions
    pub size: f32,
    /// The maximum velocity magnitude that the vehicle is able to reach
    pub max_speed: f32,
    /// The maximum steering force that is able to be applied to the vehicle
    pub max_steering_force: f32,
    /// The 2D position point
    pub pos: Point2<f32>,
    /// A radian angle that determines where the vehicle is pointed towards
    /// An angle of zero would point the vehicle towards the right
    /// This value is set to where ever the velocity vector is pointed towards
    pub angle: f32,
    /// The 2D velocity vector
    vel: Vector2<f32>,
    /// The 2D acceleration vector
    acc: Vector2<f32>,
}

impl Vehicle {
    /// Creates a new vehicle based on the provided optional attributes
    pub fn new(
        size: f32,
        max_speed: f32,
        max_steering_force: f32,
        pos: Point2<f32>,
        angle: f32,
    ) -> Self {
        Self {
            size,
            max_speed,
            max_steering_force,
            acc: Vector2::new(0.0, 0.0),
            vel: Vector2::new(0.0, 0.0),
            angle,
            pos,
        }
    }

    /// Applies a force that will point the vehicle towards its target
    pub fn seek(&mut self, target: &Point2<f32>) {
        // Get the desired velocity vector
        let mut desired = target - self.pos;
        // Set the magnitude of the desired vector to the maximum speed
        desired = desired.normalize() * self.max_speed;

        let mut steering_force = desired - self.vel;
        // Limit the steering force to the maximum value
        if steering_force.magnitude() > self.max_steering_force {
            steering_force = steering_force.normalize() * self.max_steering_force;
        }

        // Apply the steering force
        self.acc += steering_force;
    }

    /// Update the physics of the vehicle
    pub fn update(&mut self) {
        // Limit the velocity magnitude to the maximum speed
        if self.vel.magnitude() > self.max_speed {
            self.vel = self.vel.normalize() * self.max_speed;
        }
        self.vel += self.acc;
        // Point the vehicle towards its velocity vector
        self.angle = self.vel.y.atan2(self.vel.x);
        self.pos += self.vel;
        self.acc *= 0.0;
    }

    /// Draw the triangle that represents the vehicle
    pub fn draw(&self, ctx: &mut Context) -> GameResult {
        let dimensions = (
            DIMENSION_RATIO.0 as f32 * self.size,
            DIMENSION_RATIO.1 as f32 * self.size,
        );

        let triangle = graphics::Mesh::new_polygon(
            ctx,
            graphics::DrawMode::fill(),
            &[
                Point2::new(dimensions.0 * 0.5, 0.0),
                Point2::new(-dimensions.0 * 0.5, dimensions.1 * 0.5),
                Point2::new(-dimensions.0 * 0.5, -dimensions.1 * 0.5),
            ],
            [0.0, 1.0, 0.0, 1.0].into(),
        )?;

        let mut parameters = DrawParam::new();
        parameters = parameters.dest(self.pos);
        parameters = parameters.rotation(self.angle);

        graphics::draw(ctx, &triangle, parameters)?;
        Ok(())
    }
}
