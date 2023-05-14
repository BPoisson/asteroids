use ggez::{Context, GameError, graphics};
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
    pub to_remove: bool
}

impl Projectile {
    pub fn new(ctx: &Context, x_pos: f32, y_pos: f32, forward_x: f32, forward_y: f32) -> Self {
        let position: Vec2 = Vec2::new(x_pos, y_pos);

        let circle_mesh: Mesh = Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            position,
            PROJECTILE_RADIUS,
            2.0,
            Color::WHITE
        ).unwrap();

        return Projectile {
            circle_mesh,
            position,
            forward: Vec2::new(forward_x, forward_y),
            speed: PROJECTILE_SPEED,
            to_remove: false
        }
    }

    pub fn draw(&self, canvas: &mut Canvas) -> Result<(), GameError> {
        canvas.draw(
            &self.circle_mesh,
            graphics::DrawParam::default()
        );

        Ok(())
    }

    pub fn move_forward(&mut self, ctx: &Context, dt: f32) -> Result<(), GameError> {
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

        Ok(())
    }

    pub fn set_out_of_bounds(&mut self) -> Result<(), GameError> {
        if self.position.x < 0.0
            || self.position.y <0.0
            || self.position.x > SCREEN_SIZE.x
            || self.position.y > SCREEN_SIZE.y {

            self.to_remove = true;
        }
        Ok(())
    }
}