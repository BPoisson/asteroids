use ggez::{Context, GameError, graphics, mint};
use ggez::graphics::{Canvas, Color, Mesh, Rect};
use ggez::mint::{Point2, Vector2};
use crate::{Vec2};
use crate::constants::SCREEN_SIZE;

pub const PROJECTILE_RADIUS: f32 = 5.0;

pub struct Projectile {
    pub position: Vec2,
    pub circle_mesh: Mesh,
    pub forward: Vec2,
    pub speed: f32,
    pub to_remove: bool
}

impl Projectile {
    pub fn new(ctx: &Context, x_pos: f32, y_pos: f32, forward_x: f32, forward_y: f32) -> Self {
        let position:Vec2 = Vec2::new(x_pos, y_pos);

        let circle_mesh: Mesh = Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            position,
            PROJECTILE_RADIUS,
            2.0,
            Color::WHITE
        ).unwrap();

        return Projectile {
            position,
            circle_mesh,
            forward: Vec2::new(forward_x, forward_y),
            speed: 15.0,
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

    pub fn move_forward(&mut self, ctx: &Context) -> Result<(), GameError> {
        self.position.x = self.position.x + self.forward.x * self.speed;
        self.position.y = self.position.y + self.forward.y * self.speed;

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