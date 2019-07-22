//! 2d genetic algorithm simulation

use ggez;
use ggez::event;
use ggez::input::mouse;
use ggez::graphics;
use ggez::nalgebra::Point2;
use ggez::{Context, GameResult};

use evolution::vehicle::Vehicle;

const SCREEN_SIZE: (f32, f32) = (640.0, 480.0);

struct MainState {
    vehicles: Vec<Vehicle>,
}

impl MainState {
    fn new() -> GameResult<MainState> {
        let vehicles = vec![Vehicle::new(25.0, &SCREEN_SIZE)];
        Ok(MainState { vehicles })
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let mint_mouse_position = mouse::position(ctx);
        let na_mouse_position = Point2::new(mint_mouse_position.x, mint_mouse_position.y);
        for vehicle in self.vehicles.iter_mut() {
            //vehicle.seek(&na_mouse_position);
            vehicle.update();
        }

        Ok(())
    }

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

pub fn main() -> GameResult {
    let (ctx, event_loop) = &mut ggez::ContextBuilder::new("evolution", "Austin Baugh")
        .window_setup(ggez::conf::WindowSetup::default().title("Evolution!"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1))
        .build()?;

    let state = &mut MainState::new()?;

    event::run(ctx, event_loop, state)
}
