//! Evolution will be a 2D genetic algorithm simulation.
//!
//! A `Fish` will first spawn at a random location in the window.
//! The `Fish` will then go on to seek and eat `Food` and `Poison`.

use ggez::{conf, event, graphics, nalgebra::Point2, timer, Context, ContextBuilder, GameResult};
use rand::{rngs::ThreadRng, Rng};
use ron::de::from_reader;
use serde::Deserialize;
use std::{fs::File, path::PathBuf};

use evolution::{
    fish::{Fish, FishConfig},
    food::{Food, FoodConfig},
    Entity,
};

/// The configuration structure that is read and deserialized from `config.ron`
#[derive(Debug, Deserialize)]
struct Config {
    /// Whether or not the window is fullscreen
    fullscreen: bool,
    /// If the window is not fullscreen, the size of the window will be specified based on the
    /// provided (width, height)
    window_size: (f32, f32),
    /// Whether or not the current FPS should be displayed in the simulation window
    show_fps: bool,
    /// The thickness of the padding boundary for the fish around the window in pixels
    boundary_padding: f32,
    /// The configuration pertaining to the fish
    fish: FishConfig,
    /// The configuration pertaining to the food
    food: FoodConfig,
}

/// The application state that keeps track of all configurations and entities of the simulation
struct State {
    /// The configuration information set from `config.ron` when the program was executed
    config: Config,
    /// Random number generator
    rng: ThreadRng,
    /// A collection of food
    food: Vec<Food>,
    /// A collection of fish groups who are organized based on their level in the food chain
    fish_groups: Vec<Vec<Fish>>,
    /// The spritesheet of the fish used for its animation
    fish_image: graphics::Image,
}

impl State {
    /// Creates a new instance of the application state
    fn new(ctx: &mut Context, config: Config) -> GameResult<State> {
        // Random number generator is used for psuedo-random components of this
        // simulation
        let mut rng = rand::thread_rng();

        let mut food = Vec::new();
        // Spawn the food
        for _ in 1..config.food.quantity {
            Self::add_food(&mut food, &config, &mut rng);
        }

        let mut fish_groups = Vec::new();

        let fish_per_group = config.fish.quantity / config.fish.total_food_chain_links;
        // Spawn the fish
        for group_index in 0..config.fish.total_food_chain_links {
            fish_groups.push(Vec::new());
            for _ in 0..fish_per_group {
                fish_groups[group_index].push(Fish::new(
                    &config.fish,
                    &group_index,
                    &config.window_size,
                    &mut rng,
                ));
            }
        }

        // Retrieve the spritesheet for the fish animation
        let mut fish_image = graphics::Image::new(ctx, "/frames.png").unwrap();
        // This makes the pixel art visibly sharp, rather than blurry
        fish_image.set_filter(graphics::FilterMode::Nearest);

        Ok(State {
            config,
            rng,
            fish_groups,
            food,
            fish_image,
        })
    }

    /// Adds a peice of food to the collection
    fn add_food(food: &mut Vec<Food>, config: &Config, rng: &mut ThreadRng) {
        food.push(Food::new(Point2::new(
            rng.gen_range(
                config.boundary_padding,
                config.window_size.0 - config.boundary_padding,
            ),
            rng.gen_range(
                config.boundary_padding,
                config.window_size.1 - config.boundary_padding,
            ),
        )));
    }
}

impl event::EventHandler for State {
    /// Updates all elements of the current application state
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        if self.rng.gen_ratio(1, 10) {
            Self::add_food(&mut self.food, &self.config, &mut self.rng);
        }

        for group_index in 0..self.config.fish.total_food_chain_links {
            let (prey, other_fish_groups) = self.fish_groups.split_at_mut(group_index);

            let predator_positions = if group_index == self.config.fish.total_food_chain_links - 1 {
                None
            } else {
                Some(
                    other_fish_groups[1]
                        .iter()
                        .map(|predator| predator.pos())
                        .collect(),
                )
            };

            // We should remove dead fish from our collection of fish
            other_fish_groups[0].retain(|fish| fish.is_alive());

            let mut new_fish = None;

            for fish in other_fish_groups[0].iter_mut() {
                // Only update living fish
                if fish.is_alive() {
                    if new_fish.is_none() && self.rng.gen_ratio(1, 1000) {
                        new_fish = Some(fish.clone(&mut self.rng, self.config.fish.mutation_rate));
                    }
                    // Update the behavior state of all fish
                    fish.behave(
                        &mut self.food,
                        prey,
                        &predator_positions,
                        self.config.fish.eating_radius,
                    );
                    // Bound the fish to a padding in the window
                    fish.bound(&self.config.window_size, self.config.boundary_padding);
                    // Update the physical state of all fish
                    fish.update();
                }
            }

            if let Some(new_fish) = new_fish {
                other_fish_groups[0].push(new_fish)
            };
        }

        Ok(())
    }

    /// Draws all elements of the current application state
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        // Sets the background to a solid blue-ish color
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        for food in self.food.iter() {
            if let Err(error) = food.draw(ctx) {
                return Err(error);
            }
        }

        for fish_group in self.fish_groups.iter_mut() {
            for fish in fish_group.iter_mut() {
                if let Err(error) = fish.draw(
                    ctx,
                    &self.fish_image,
                    self.config.fish.frames_per_animation_frame,
                ) {
                    return Err(error);
                }
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
