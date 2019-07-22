use ggez::{
    graphics,
    graphics::DrawParam,
    Context,
    GameResult,
    nalgebra::{Point2, Vector2},
};
use rand::Rng;

const DIMENSION_RATIO: (u8, u8) = (2, 1);

pub struct Vehicle {
    pub size: f32,
    pub max_speed: f32,
    pub max_steering_force: f32,

    pub pos: Point2<f32>,
    pub angle: f32,
    pub vel: Vector2<f32>,
    pub acc: Vector2<f32>,
}

impl Vehicle {
    pub fn new(size: f32, screen_size: &(f32, f32)) -> Self {
        let mut rng = rand::thread_rng();
        let max_speed = 125.0 / size;

        let pos = Point2::new(
            rng.gen_range(0.0, screen_size.0),
            rng.gen_range(0.0, screen_size.1)
        );

        let angle = rng.gen_range(0.0, 2.0 * std::f32::consts::PI);

        let speed = rng.gen_range(0.0, max_speed);
        let vel = Vector2::new(angle.cos(), angle.sin()) * speed;

        Self {
            size,
            max_speed,
            max_steering_force: 3.0,

            pos,
            angle,
            vel,
            acc: Vector2::new(0.0, 0.0),
        }
    }

    pub fn seek(&mut self, _mouse_position: &Point2<f32>) {
        unimplemented!();
    }

    pub fn update(&mut self) {
        self.vel += self.acc;
        self.pos += self.vel;
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult {
        let dimensions = (self.width(), self.height());

        let triangle = graphics::Mesh::new_polygon(
            ctx,
            graphics::DrawMode::fill(),
            &[
                Point2::new( dimensions.0 * 0.5,  0.0),
                Point2::new(-dimensions.0 * 0.5,  dimensions.1 * 0.5),
                Point2::new(-dimensions.0 * 0.5, -dimensions.1 * 0.5)
            ],
            [0.0, 1.0, 0.0, 1.0].into(),
        )?;

        let mut parameters = DrawParam::new();
        parameters = parameters.dest(self.pos);
        parameters = parameters.rotation(self.angle);

        graphics::draw(ctx, &triangle, parameters)?;
        Ok(())
    }

    fn width(&self) -> f32 {
        DIMENSION_RATIO.0 as f32 * self.size
    }

    fn height(&self) -> f32 {
        DIMENSION_RATIO.1 as f32 * self.size
    }
}
