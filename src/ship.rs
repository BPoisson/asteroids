use ggez::{Context, GameError, graphics};
use ggez::glam::Vec2;
use ggez::graphics::{Canvas, Color, Mesh, Rect};
use crate::{projectile, SCREEN_SIZE};
use crate::projectile::{Projectile};

pub struct Ship {
    pub health: u8,
    pub rect: Rect,
    pub rotation: f32,
    pub forward: Vec2,
    pub speed: f32
}

impl Ship {
    pub fn new(health: u8, rect: Rect) -> Self {
        return Ship {
            health,
            rect,
            rotation: 270.0_f32.to_radians(),
            forward: Vec2::new(0.0, -1.0),
            speed: 8.0
        }
    }

    pub fn draw(&self, canvas: &mut Canvas) -> Result<(), GameError> {
        canvas.draw(
            &graphics::Quad,
            graphics::DrawParam::new()
                .dest_rect(self.rect)
                .rotation(self.rotation)
                .offset([0.5, 0.5])
                .color(Color::WHITE));

        Ok(())
    }

    pub fn shoot(&self, ctx: &Context) -> Projectile {
        return Projectile::new(
            ctx,
            self.rect.x,
            self.rect.y,
            self.forward.x,
            self.forward.y
        );
    }

    pub fn clamp(&mut self) -> () {
        if self.rect.x < 0.0 {
            self.rect.x = SCREEN_SIZE.x;
        } else if self.rect.x > SCREEN_SIZE.x {
            self.rect.x = 0.0;
        }
        if self.rect.y < 0.0 {
            self.rect.y = SCREEN_SIZE.y;
        } else if self.rect.y > SCREEN_SIZE.y {
            self.rect.y = 0.0;
        }
    }
}
