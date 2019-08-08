//! Evolution will be a 2D genetic algorithm simulation.
//!
//! A `Fish` will first spawn at a random location in the window.
//! The `Fish` will then go on to seek and eat `Food` and `Poison`.

use ggez::{conf, event, graphics, nalgebra::Point2, timer, Context, ContextBuilder, GameResult};
use rand::Rng;
use ron::de::from_reader;
use serde::Deserialize;
use std::{fs::File, path::PathBuf};

use evolution::fish::{Fish, FishConfig};
use evolution::food::{Food, FoodConfig};
use evolution::inverse_map_range;

/// The configuration structure that is read and deserialized from `config.ron`
#[derive(Debug, Deserialize)]
struct Config {
    /// Whether or not the window is fullscreen
    fullscreen: bool,
    /// If the window is not fullscreen, the size of the window will be specified based on the
    /// provided (width, height)
    window_size: (f32, f32),
    /// The FPS that the simulation would preferably run at
    desired_fps: u32,
    /// Whether or not the current FPS should be displayed in the simulation window
    show_fps: bool,
    /// The configuration pertaining to the fish
    fish: FishConfig,
    /// The configuration pertaining to the food
    food: FoodConfig,
    /// The configuration pertaining to the poison
    poison: FoodConfig,
}

/// The application state that keeps track of all configurations and entities of the simulation
struct State {
    /// The configuration information set from `config.ron` when the program was executed
    config: Config,
    /// A collection of fish
    fish: Vec<Fish>,
    /// A collection of food
    food: Vec<Food>,
    /// A collection of poison
    poison: Vec<Food>,
    /// The spritesheet of the fish used for its animation
    fish_image: graphics::Image,
}

impl State {
    /// Creates a new instance of the application state
    fn new(ctx: &mut Context, config: Config) -> GameResult<State> {
        // Random number generator is used for psuedo-random components of this
        // simulation
        let mut rng = rand::thread_rng();

        let mut fish = Vec::new();
        // Spawn the fish
        for _ in 1..config.fish.quantity {
            // Setting the non-default fields of the fish
            // Scale is a random field between the specified range in `FishConfig`
            let scale = rng.gen_range(config.fish.scale_range.0, config.fish.scale_range.1);
            // Max speed and max steering force are values that are inversely
            // proportional to the scale value of the fish
            let max_speed =
                inverse_map_range(scale, config.fish.scale_range, config.fish.max_speed_range);
            let max_steering_force = inverse_map_range(
                scale,
                config.fish.scale_range,
                config.fish.max_steering_force_range,
            );
            // The angle is just a random radian around the unit circle
            let angle = rng.gen_range(0.0, 2.0 * std::f32::consts::PI);
            // The position is a random location in the window
            // TODO: In fullscreen mode, the window size may change on program
            // execution resulting in the fish, food, and poison spawning in a different area than the
            // window dimensions.
            let pos = Point2::new(
                rng.gen_range(0.0, config.window_size.0),
                rng.gen_range(0.0, config.window_size.1),
            );
            // The DNA currently holds random values for the weights against steering
            // towards food and poison respectively.
            let dna = [rng.gen_range(-5.0, 5.0), rng.gen_range(-5.0, 5.0)];

            fish.push(Fish::new(
                scale,
                max_speed,
                max_steering_force,
                pos,
                angle,
                dna,
            ));
        }

        let mut food = Vec::new();
        // Spawn the food
        for _ in 1..config.food.quantity {
            food.push(Food {
                // Size is a random field between the specified range in `FoodConfig`
                radius: rng.gen_range(
                          config.food.radius_range.0,
                          config.food.radius_range.1
                      ),
                // The position is a random location in the window
                pos: Point2::new(
                    rng.gen_range(0.0, config.window_size.0),
                    rng.gen_range(0.0, config.window_size.1)
                ),
                // The color is a slightly transparent green
                color: /*[
                    rng.gen_range(0.0, 1.0),
                    rng.gen_range(0.0, 1.0),
                    rng.gen_range(0.0, 1.0),
                    0.8
                ]*/[0.0, 1.0, 0.0, 0.8]
            });
        }

        let mut poison = Vec::new();
        // Spawn the poison
        for _ in 1..config.poison.quantity {
            poison.push(Food {
                // Size is a random field between the specified range in `PoisonConfig`
                radius: rng.gen_range(config.poison.radius_range.0, config.poison.radius_range.1),
                // The position is a random location in the window
                pos: Point2::new(
                    rng.gen_range(0.0, config.window_size.0),
                    rng.gen_range(0.0, config.window_size.1),
                ),
                // The color is a slightly transparent red
                color: [1.0, 0.0, 0.0, 0.8],
            });
        }

        // Retrieve the spritesheet for the fish animation
        let mut fish_image = graphics::Image::new(ctx, "/frames.png").unwrap();
        // This makes the pixel art visibly sharp, rather than blurry
        fish_image.set_filter(graphics::FilterMode::Nearest);

        Ok(State {
            config,
            fish,
            food,
            poison,
            fish_image,
        })
    }
}

impl event::EventHandler for State {
    /// Updates all elements of the current application state
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        // We should only update the application state when we're within our preferred
        // FPS
        while timer::check_update_time(ctx, self.config.desired_fps) {
            for fish in self.fish.iter_mut() {
                // Update the behavior state of all fish
                fish.behaviors(&mut self.food, &mut self.poison);
                // Update the physical state of all fish
                fish.update();
            }
        }

        Ok(())
    }

    /// Draws all elements of the current application state
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        // Sets the background to a solid blue-ish color
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

        for fish in self.fish.iter_mut() {
            if let Err(error) = fish.draw(
                ctx,
                &self.fish_image,
                self.config.fish.frames_per_animation_frame,
            ) {
                return Err(error);
            }
        }

        if self.config.show_fps {
            let fps = timer::fps(ctx);
            let fps_text = graphics::Text::new(format!("FPS: {:.*}", 1, fps));
            graphics::draw(ctx, &fps_text, (Point2::new(5.0, 5.0), graphics::WHITE))?;
        }

        graphics::present(ctx)?;

        Ok(())
    }
}

/// The main function :D
pub fn main() -> GameResult {
    // Specifying the path for the configuration file and deserializing its data
    let input_path = format!("{}/config.ron", env!("CARGO_MANIFEST_DIR"));
    let f = File::open(&input_path)?;
    let config: Config = match from_reader(f) {
        Ok(x) => x,
        Err(e) => {
            println!("Failed to load `config.ron`: {}", e);
            std::process::exit(1);
        }
    };

    // Setting the path for the assets folder
    let assets_dir = PathBuf::from(format!("{}/assets", env!("CARGO_MANIFEST_DIR")));

    // If not fullscreen, display window based on provided dimensions
    let window_settings = if config.fullscreen {
        conf::WindowMode::default().fullscreen_type(conf::FullscreenType::True)
    } else {
        conf::WindowMode::default().dimensions(config.window_size.0, config.window_size.1)
    };

    // Setting-up the simulation and running it
    let (ctx, event_loop) = &mut ContextBuilder::new("evolution", "Austin Baugh")
        .window_setup(conf::WindowSetup::default().title("Evolution!"))
        .window_mode(window_settings)
        .add_resource_path(assets_dir)
        .build()?;

    let state = &mut State::new(ctx, config)?;

    event::run(ctx, event_loop, state)
}
