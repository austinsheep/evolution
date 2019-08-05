//! Evolution will be a 2D genetic algorithm simulation.
//!
//! A `Vehicle` will first spawn at a random location in the window.
//! The `Vehicle` will then go on to `seek` the cursor's location.

use ggez::{
    conf,
    event,
    graphics,
    nalgebra::Point2,
    Context,
    ContextBuilder,
    GameResult,
    timer,
};
use rand::Rng;

use evolution::food::Food;
use evolution::vehicle::Vehicle;

/// The dimensions of the window (width, height)
const SCREEN_SIZE: (f32, f32) = (640.0, 480.0);
const DESIRED_FPS: u32 = 12;

/// The application state that keeps track of the current state of vehicles and food
struct MainState {
    /// A collection of vehicles
    vehicles: Vec<Vehicle>,
    /// A collection of food
    food: Vec<Food>,
    poison: Vec<Food>,
}

impl MainState {
    /// Creates a new instance of the application state
    fn new() -> GameResult<MainState> {
        // Random number generator is used for the location of the vehicle and its angle
        let mut rng = rand::thread_rng();

        let mut vehicles = Vec::new();

        // The non-default attributes of the vehicle that are to be specified before-hand
        for _ in 1..20 {
            let size = rng.gen_range(5.0, 15.0);
            let max_speed = 2.0;
            let max_steering_force = 0.01;
            let angle = rng.gen_range(0.0, 2.0 * std::f32::consts::PI);
            let pos = Point2::new(
                rng.gen_range(0.0, SCREEN_SIZE.0),
                rng.gen_range(0.0, SCREEN_SIZE.1),
            );
            let dna = [
                rng.gen_range(-5.0, 5.0),
                rng.gen_range(-5.0, 5.0),
            ];

            vehicles.push(Vehicle::new(
                size,
                max_speed,
                max_steering_force,
                pos,
                angle,
                dna
            ));
        }

        let mut food = Vec::new();
        for _ in 1..50 {
            food.push(Food {
                size: rng.gen_range(5.0, 20.0),
                pos: Point2::new(
                    rng.gen_range(0.0, SCREEN_SIZE.0),
                    rng.gen_range(0.0, SCREEN_SIZE.1)
                ),
                color: /*[
                    rng.gen_range(0.0, 1.0),
                    rng.gen_range(0.0, 1.0),
                    rng.gen_range(0.0, 1.0),
                    0.8
                ]*/[0.0, 1.0, 0.0, 0.8]
            });
        }

        let mut poison = Vec::new();
        for _ in 1..50 {
            poison.push(Food {
                size: rng.gen_range(5.0, 20.0),
                pos: Point2::new(
                    rng.gen_range(0.0, SCREEN_SIZE.0),
                    rng.gen_range(0.0, SCREEN_SIZE.1)
                ),
                color: [1.0, 0.0, 0.0, 0.8]
            });
        }

        Ok(MainState { vehicles, food, poison })
    }
}

impl event::EventHandler for MainState {
    /// Updates all elements of the current application state
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        while timer::check_update_time(ctx, DESIRED_FPS) {
            for vehicle in self.vehicles.iter_mut() {
                vehicle.behaviors(&mut self.food, &mut self.poison);
                vehicle.update();
            }
        }

        Ok(())
    }

    /// Draws all elements of the current application state
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        for poison in self.poison.iter() {
            if let Err(error) = poison.draw(ctx) {
                return Err(error);
            }
        }

        for food in self.food.iter() {
            if let Err(error) = food.draw(ctx) {
                return Err(error);
            }
        }

        for vehicle in self.vehicles.iter() {
            if let Err(error) = vehicle.draw(ctx) {
                return Err(error);
            }
        }
        
        let fps = timer::fps(ctx);
        let fps_text = graphics::Text::new(format!("FPS: {:.*}", 1, fps));
        graphics::draw(ctx, &fps_text, (Point2::new(0.0, 0.0), graphics::WHITE))?;

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
