use std::time::Instant;
use ggez::Context;
use ggez::glam::Vec2;
use ggez::graphics::{Canvas, Color, DrawMode, DrawParam, Mesh};
use rand::Rng;
use rand::rngs::ThreadRng;
use crate::constants::SCREEN_SIZE;

pub const SPEED: f32 = 200.0;
const ALIEN_DURATION_SECS: f32 = 28.0;
pub const ALIEN_X_BOUND: f32 = 55.0;
pub const ALIEN_NEGATIVE_Y_BOUND: f32 = 30.0;
pub const ALIEN_POSITIVE_Y_BOUND: f32 = 15.0;

pub struct Alien {
    ship_mesh: Mesh,
    ship_window_line: Mesh,
    ship_body_line: Mesh,
    pub position: Vec2,
    forward: Vec2,
    creation_time: Instant,
    pub expired: bool
}

impl Alien {
    pub fn new(ctx: &Context, rng: &mut ThreadRng) -> Self {
        let position: Vec2;
        let forward: Vec2;

        if rng.gen_bool(0.5) {
            position = Vec2::new(-60.0, SCREEN_SIZE.y / 2.0);
            forward = Vec2::new(1.0, 0.0);
        } else {
            position = Vec2::new(SCREEN_SIZE.x + 60.0, SCREEN_SIZE.y / 2.0);
            forward = Vec2::new(-1.0, 0.0);
        }

        return Alien {
            ship_mesh: Alien::create_ship_mesh(ctx, &position),
            ship_window_line: Alien::create_ship_window_line(ctx, &position),
            ship_body_line: Alien::create_ship_body_line(ctx, &position),
            position,
            forward,
            creation_time: Instant::now(),
            expired: false
        };
    }

    pub fn draw(&mut self, ctx: &Context, canvas: &mut Canvas) -> () {
        self.ship_mesh = Alien::create_ship_mesh(&ctx, &self.position);
        self.ship_window_line = Alien::create_ship_window_line(&ctx, &self.position);
        self.ship_body_line = Alien::create_ship_body_line(&ctx, &self.position);

        canvas.draw(
            &self.ship_mesh,
            DrawParam::default()
        );

        canvas.draw(
            &self.ship_window_line,
            DrawParam::default()
        );

        canvas.draw(
            &self.ship_body_line,
            DrawParam::default()
        );
    }

    pub fn move_forward(&mut self, rng: &mut ThreadRng, dt: &f32) -> () {
        let random_x: i32 = rng.gen_range(0..500);
        let random_y: i32 = rng.gen_range(0..250);

        // Randomly move left or right.
        if self.forward.x < 1.0 && random_x == 0 {
            self.forward.x += 1.0;
        } else if self.forward.x > -1.0 && random_x == 1 {
            self.forward.x -= 1.0;
        }

        // Randomly move up or down.
        if self.forward.y < 1.0 && random_y == 0 {
            self.forward.y += 1.0;
        } else if self.forward.y > -1.0 && random_y == 1 {
            self.forward.y -= 1.0;
        }
        self.position.x += self.forward.x * SPEED * dt;
        self.position.y += self.forward.y * SPEED * dt;

        self.clamp_position();
    }

    pub fn clamp_position(&mut self) -> () {
        if self.position.x < 0.0 {
            self.position.x = SCREEN_SIZE.x;
        } else if self.position.x > SCREEN_SIZE.x {
            self.position.x = 0.0;
        }
        if self.position.y < 0.0 {
            self.position.y = SCREEN_SIZE.y;
        } else if self.position.y > SCREEN_SIZE.y {
            self.position.y = 0.0;
        }
    }

    fn create_ship_mesh(ctx: &Context, position: &Vec2) -> Mesh {
        let ship_points: [Vec2; 10] = Alien::get_ship_points(position);

        return Mesh::new_polygon(
            ctx,
            DrawMode::stroke(2.0),
            &ship_points,
            Color::WHITE
        ).unwrap();
    }

    fn create_ship_window_line(ctx: &Context, position: &Vec2) -> Mesh {
        let ship_window_line_points: [Vec2; 2] = [
            *position + Vec2::new(-25.0, -15.0),
            *position + Vec2::new(25.0, -15.0),
        ];

        return Mesh::new_line(
            ctx,
            &ship_window_line_points,
            2.0,
            Color::WHITE
        ).unwrap();
    }

    fn create_ship_body_line(ctx: &Context, position: &Vec2) -> Mesh {
        let ship_body_line_points: [Vec2; 2] = [
            *position + Vec2::new(-55.0, 0.0),
            *position + Vec2::new(55.0, 0.0),
        ];

        return Mesh::new_line(
            ctx,
            &ship_body_line_points,
            2.0,
            Color::WHITE
        ).unwrap();
    }

    fn get_ship_points(position: &Vec2) -> [Vec2; 10] {
        return [
            *position + Vec2::new(-15.0, -30.0),
            *position + Vec2::new(-25.0, -15.0),
            *position + Vec2::new(-40.0, -15.0),
            *position + Vec2::new(-55.0, 0.0),
            *position + Vec2::new(-40.0, 15.0),
            *position + Vec2::new(40.0, 15.0),
            *position + Vec2::new(55.0, 0.0),
            *position + Vec2::new(40.0, -15.0),
            *position + Vec2::new(25.0, -15.0),
            *position + Vec2::new(15.0, -30.0),
        ];
    }

    pub fn check_expiration(&mut self, now_time: &Instant) -> () {
        self.expired = self.expired || (*now_time - self.creation_time).as_secs_f32() > ALIEN_DURATION_SECS;
    }
}