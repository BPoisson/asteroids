use ggez::{Context, graphics};
use ggez::glam::Vec2;
use ggez::graphics::{Canvas, Color, Mesh};
use rand::Rng;
use rand::rngs::ThreadRng;
use crate::constants::SCREEN_SIZE;

pub const ASTEROID_BIG_RADIUS: f32 = 80.0;
pub const ASTEROID_MEDIUM_RADIUS: f32 = 50.0;
pub const ASTEROID_SMALL_RADIUS: f32 = 30.0;

pub const ASTEROID_BIG_SPEED: f32 = 100.0;
pub const ASTEROID_MEDIUM_SPEED: f32 = 200.0;
pub const ASTEROID_SMALL_SPEED: f32 = 300.0;

pub enum AsteroidSize {
    BIG,
    MEDIUM,
    SMALL
}

pub struct Asteroid {
    circle_mesh: Mesh,
    pub position: Vec2,
    pub radius: f32,
    forward: Vec2,
    pub size: AsteroidSize,
    tolerance: f32,
    speed: f32,
    pub destroyed: bool
}

impl Asteroid {
    pub fn new(ctx: &Context, rng: &mut ThreadRng) -> Self {
        let position: Vec2 =  Asteroid::get_spawn_position(rng, ASTEROID_BIG_RADIUS);
        let x_dir: f32 = rng.gen_range(-1.0..=1.0);
        let y_dir: f32 = rng.gen_range(-1.0..=1.0);
        let forward: Vec2 = Vec2::new(x_dir, y_dir);
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
            radius: ASTEROID_BIG_RADIUS,
            forward,
            size: AsteroidSize::BIG,
            tolerance,
            speed: ASTEROID_BIG_SPEED,
            destroyed: false
        }
    }

    pub fn new_smaller(self: &mut Self, ctx: &Context, rng: &mut ThreadRng) -> Self {
        let parent_x: f32 = self.position.x;
        let parent_y: f32 = self.position.y;
        let x_pos: f32 = rng.gen_range(parent_x - 20.0..parent_x + 20.0);
        let y_pos: f32 = rng.gen_range(parent_y - 20.0..parent_y + 20.0);
        let x_dir: f32 = rng.gen_range(-1.0..=1.0);
        let y_dir: f32 = rng.gen_range(-1.0..=1.0);
        let position:Vec2 = Vec2::new(x_pos, y_pos);
        let forward:Vec2 = Vec2::new(x_dir, y_dir);
        let tolerance: f32 = rng.gen_range(0.0..5.0);
        let size: AsteroidSize = Asteroid::next_size(&self.size);
        let radius: f32 = Asteroid::radius_for_size(&size);
        let speed: f32 = Asteroid::speed_for_size(&size);

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
            radius,
            forward,
            size,
            tolerance,
            speed,
            destroyed: false
        }
    }

    pub fn draw(&self, canvas: &mut Canvas) -> () {
        canvas.draw(
            &self.circle_mesh,
            graphics::DrawParam::default()
        );
    }

    pub fn get_spawn_position(rng: &mut ThreadRng, radius: f32) -> Vec2 {
        let position: Vec2;

        if rng.gen_bool(0.5) {          // Spawn to the left or right.
            if rng.gen_bool(0.5) {      // Spawn left
                position = Vec2::new(-radius, rng.gen_range(0.0..=SCREEN_SIZE.y));
            } else {                       // Spawn right
                position = Vec2::new(SCREEN_SIZE.x + radius, rng.gen_range(0.0..=SCREEN_SIZE.y));
            }
        } else {                           // Spawn top or bottom.
            if rng.gen_bool(0.5) {      // Spawn top
                position = Vec2::new(rng.gen_range(0.0..=SCREEN_SIZE.x), -SCREEN_SIZE.y - radius);
            } else {                       // Spawn bottom
                position = Vec2::new(rng.gen_range(0.0..=SCREEN_SIZE.x), SCREEN_SIZE.y + radius);
            }
        }
        return position;
    }

    pub fn move_forward(&mut self, ctx: &Context, dt: f32) -> () {
        self.position.x = self.position.x + self.forward.x * self.speed * dt;
        self.position.y = self.position.y + self.forward.y * self.speed * dt;

        self.clamp();

        self.circle_mesh = Mesh::new_circle(
            ctx,
            graphics::DrawMode::stroke(2.0),
            self.position,
            self.radius,
            self.tolerance,
            Color::WHITE
        ).unwrap();
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

    pub fn destroy_asteroid(&mut self, ctx: &Context, rng: &mut ThreadRng) -> Vec<Asteroid> {
        let mut new_asteroids: Vec<Asteroid> = Vec::new();
        let asteroid_pieces: i32 = rng.gen_range(2..=3);

        if self.radius != ASTEROID_SMALL_RADIUS {
            for _ in 0..asteroid_pieces {
                new_asteroids.push(Asteroid::new_smaller(self, ctx, rng));
            }
        }
        self.destroyed = true;

        return new_asteroids;
    }

    fn next_size(size: &AsteroidSize) -> AsteroidSize {
        match size {
            AsteroidSize::BIG => AsteroidSize::MEDIUM,
            _ => AsteroidSize::SMALL
        }
    }

    fn radius_for_size(size: &AsteroidSize) -> f32 {
        match size {
            AsteroidSize::BIG => ASTEROID_BIG_RADIUS,
            AsteroidSize::MEDIUM => ASTEROID_MEDIUM_RADIUS,
            _ => ASTEROID_SMALL_RADIUS
        }
    }

    fn speed_for_size(size: &AsteroidSize) -> f32 {
        match size {
            AsteroidSize::BIG => ASTEROID_BIG_SPEED,
            AsteroidSize::MEDIUM => ASTEROID_MEDIUM_SPEED,
            _ => ASTEROID_SMALL_SPEED
        }
    }
}