//! Evolution will be a 2D genetic algorithm simulation.
//!
//! A `Vehicle` will first spawn at a random location in the window.
//! The `Vehicle` will then go on to `seek` the cursor's location.

use ggez::{
    conf,
    event,
    graphics,
    input::mouse,
    nalgebra::Point2,
    Context,
    ContextBuilder,
    GameResult,
};
use rand::Rng;

use evolution::vehicle::Vehicle;

/// The dimensions of the window (width, height)
const SCREEN_SIZE: (f32, f32) = (640.0, 480.0);

/// The only application state at the moment
struct MainState {
    /// A collection of vehicles that exist in the current state of the simulation
    vehicles: Vec<Vehicle>,
}

impl MainState {
    /// Creates a new instance of the application state
    fn new() -> GameResult<MainState> {
        // Random number generator is used for the location of the vehicle and its angle
        let mut rng = rand::thread_rng();

        let mut vehicles = Vec::new();

        // The non-default attributes of the vehicle that are to be specified before-hand
        let size = rng.gen_range(10.0, 40.0);
        let max_speed = 5.0;
        let max_steering_force = 0.1;
        let angle = rng.gen_range(0.0, 2.0 * std::f32::consts::PI);
        let pos = Point2::new(
            rng.gen_range(0.0, SCREEN_SIZE.0),
            rng.gen_range(0.0, SCREEN_SIZE.1),
        );

        vehicles.push(Vehicle::new(
            size,
            max_speed,
            max_steering_force,
            pos,
            angle
        ));

        Ok(MainState { vehicles })
    }
}

impl event::EventHandler for MainState {
    /// Updates all elements of the current application state
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        // Have to convert mint Point2 to nalgebra Point2
        let mint_mouse_position = mouse::position(ctx);
        let na_mouse_position = Point2::new(mint_mouse_position.x, mint_mouse_position.y);

        // All vehicles seek the mouse and have their state's updated
        for vehicle in self.vehicles.iter_mut() {
            vehicle.seek(&na_mouse_position);
            vehicle.update();
        }

        Ok(())
    }

    /// Draws all elements of the current application state
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        for vehicle in self.vehicles.iter() {
            if let Err(error) = vehicle.draw(ctx) {
                return Err(error);
            }
        }

        graphics::present(ctx)?;
        Ok(())
    }
}

/// The main function :D
pub fn main() -> GameResult {
    let (ctx, event_loop) = &mut ContextBuilder::new("evolution", "Austin Baugh")
        .window_setup(conf::WindowSetup::default().title("Evolution!"))
        .window_mode(conf::WindowMode::default().dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1))
        .build()?;

    let state = &mut MainState::new()?;

    event::run(ctx, event_loop, state)
}
