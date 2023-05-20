use ggez::{Context, graphics};
use ggez::graphics::{Canvas, Color, Mesh};
use crate::{Vec2};
use crate::constants::SCREEN_SIZE;

pub const PROJECTILE_RADIUS: f32 = 5.0;
pub const PROJECTILE_SPEED: f32 = 1000.0;

pub struct Projectile {
    pub circle_mesh: Mesh,
    pub position: Vec2,
    pub forward: Vec2,
    pub speed: f32,
    pub expired: bool
}

impl Projectile {
    pub fn new(ctx: &Context, origin: &Vec2, forward: &Vec2) -> Self {
        let circle_mesh: Mesh = Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            *origin,
            PROJECTILE_RADIUS,
            2.0,
            Color::WHITE
        ).unwrap();

        return Projectile {
            circle_mesh,
            position: *origin,
            forward: *forward,
            speed: PROJECTILE_SPEED,
            expired: false
        }
    }

    pub fn draw(&self, canvas: &mut Canvas) -> () {
        canvas.draw(
            &self.circle_mesh,
            graphics::DrawParam::default()
        );
    }

    pub fn move_forward(&mut self, ctx: &Context, dt: &f32) -> () {
        self.position.x = self.position.x + self.forward.x * self.speed * dt;
        self.position.y = self.position.y + self.forward.y * self.speed * dt;

        self.circle_mesh = Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            self.position,
            PROJECTILE_RADIUS,
            2.0,
            Color::WHITE
        ).unwrap();
    }

    pub fn set_out_of_bounds(&mut self) -> () {
        if self.position.x < 0.0
            || self.position.y <0.0
            || self.position.x > SCREEN_SIZE.x
            || self.position.y > SCREEN_SIZE.y {

            self.expired = true;
        }
    }
}