use ggez::{Context, GameError, graphics};
use ggez::glam::Vec2;
use ggez::graphics::{Canvas, Color, Mesh};
use rand::Rng;
use rand::rngs::ThreadRng;
use crate::constants::SCREEN_SIZE;

pub const ASTEROID_BIG_RADIUS: f32 = 50.0;
pub const ASTEROID_MED_RADIUS: f32 = 35.0;
pub const ASTEROID_SMALL_RADIUS: f32 = 20.0;

pub struct Asteroid {
    pub circle_mesh: Mesh,
    pub position: Vec2,
    pub forward: Vec2,
    pub radius: f32,
    pub tolerance: f32,
    pub speed: f32,
    pub destroyed: bool
}

impl Asteroid {
    pub fn new(ctx: &Context) -> Self {
        let range_start: f32 = 5.0;
        let range_end: (f32, f32) = (SCREEN_SIZE.x - 5.0, SCREEN_SIZE.y - 5.0);
        let mut rng: ThreadRng = rand::thread_rng();

        let x_pos: f32 = rng.gen_range(range_start..range_end.0);
        let y_pos: f32 = rng.gen_range(range_start..range_end.1);
        let x_dir: f32 = rng.gen_range(-1.0..=1.0);
        let y_dir: f32 = rng.gen_range(-1.0..=1.0);
        let position:Vec2 = Vec2::new(x_pos, y_pos);
        let forward:Vec2 = Vec2::new(x_dir, y_dir);
        let tolerance: f32 = rng.gen_range(0.0..5.0);

        let circle_mesh: Mesh = Mesh::new_circle(
            ctx,
            graphics::DrawMode::stroke(2.0),
            position,
            ASTEROID_BIG_RADIUS,
            tolerance,
            Color::WHITE
        ).unwrap();

        return Asteroid {
            circle_mesh,
            position,
            forward,
            radius: ASTEROID_BIG_RADIUS,
            tolerance,
            speed: 1.0,
            destroyed: false
        }
    }

    pub fn new_smaller(ctx: &Context, parent_radius: f32, parent_x: f32, parent_y: f32) -> Self {
        let mut rng: ThreadRng = rand::thread_rng();

        let x_pos: f32 = rng.gen_range(parent_x - 20.0..parent_x + 20.0);
        let y_pos: f32 = rng.gen_range(parent_y - 20.0..parent_y + 20.0);
        let x_dir: f32 = rng.gen_range(-1.0..=1.0);
        let y_dir: f32 = rng.gen_range(-1.0..=1.0);
        let position:Vec2 = Vec2::new(x_pos, y_pos);
        let forward:Vec2 = Vec2::new(x_dir, y_dir);
        let tolerance: f32 = rng.gen_range(0.0..5.0);
        let radius = Asteroid::next_radius(parent_radius);

        let circle_mesh: Mesh = Mesh::new_circle(
            ctx,
            graphics::DrawMode::stroke(2.0),
            position,
            radius,
            tolerance,
            Color::WHITE
        ).unwrap();

        return Asteroid {
            circle_mesh,
            position,
            forward,
            radius,
            tolerance,
            speed: 2.0,
            destroyed: false
        }
    }

    pub fn draw(&self, canvas: &mut Canvas) -> Result<(), GameError> {
        canvas.draw(
            &self.circle_mesh,
            graphics::DrawParam::default()
        );

        Ok(())
    }

    pub fn move_forward(&mut self, ctx: &Context) -> Result<(), GameError> {
        self.position.x = self.position.x + self.forward.x * self.speed;
        self.position.y = self.position.y + self.forward.y * self.speed;

        self.clamp();

        self.circle_mesh = Mesh::new_circle(
            ctx,
            graphics::DrawMode::stroke(2.0),
            self.position,
            self.radius,
            self.tolerance,
            Color::WHITE
        ).unwrap();

        Ok(())
    }

    pub fn clamp(&mut self) -> () {
        if self.position.x < -self.radius {
            self.position.x = SCREEN_SIZE.x + self.radius;
        } else if self.position.x - self.radius > SCREEN_SIZE.x {
            self.position.x = -self.radius;
        }
        if self.position.y < -self.radius {
            self.position.y = SCREEN_SIZE.y + self.radius;
        } else if self.position.y - self.radius > SCREEN_SIZE.y {
            self.position.y = -self.radius;
        }
    }

    pub fn destroy_asteroid(&mut self, ctx: &Context) -> Vec<Asteroid> {
        let mut new_asteroids: Vec<Asteroid> = Vec::new();
        let mut rng: ThreadRng = rand::thread_rng();
        let asteroid_pieces: i32 = rng.gen_range(2..4); // 2 or 3 pieces.

        if self.radius != ASTEROID_SMALL_RADIUS {
            for _ in 0..asteroid_pieces {
                new_asteroids.push(Asteroid::new_smaller(ctx, self.radius, self.position.x, self.position.y));
            }
        }
        self.destroyed = true;

        return new_asteroids;
    }

    fn next_radius(radius: f32) -> f32 {
        if radius == ASTEROID_BIG_RADIUS {
            return ASTEROID_MED_RADIUS;
        } else if radius == ASTEROID_MED_RADIUS {
            return ASTEROID_SMALL_RADIUS;
        } else {
            return 0.0;
        }
    }
}