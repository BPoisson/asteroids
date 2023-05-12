use ggez::{Context, GameError};
use ggez::glam::{Vec2};
use ggez::graphics::{Canvas, Color, DrawMode, DrawParam, Mesh};
use crate::{SCREEN_SIZE};
use crate::projectile::{Projectile};

pub struct Ship {
    pub triangle_mesh: Mesh,
    pub position: Vec2,
    pub rotation: f32,
    pub forward: Vec2,
    pub speed: f32,
    pub lives: i32
}

impl Ship {
    pub fn new(ctx: &Context) -> Self {
        let position: Vec2 = Vec2::new(SCREEN_SIZE.x / 2.0, SCREEN_SIZE.y / 2.0);
        let rotation: f32 = 270.0_f32.to_radians();
        let triangle_mesh: Mesh = Ship::create_ship_triangle(ctx, position, rotation);

        return Ship {
            triangle_mesh,
            position,
            rotation,
            forward: Vec2::new(0.0, -1.0),
            speed: 8.0,
            lives: 4
        }
    }

    pub fn draw(&mut self, ctx: &Context, canvas: &mut Canvas) -> Result<(), GameError> {
        let triangle_points: [Vec2; 3] = Ship::get_triangle_points(self.position, self.rotation);

        self.triangle_mesh = Mesh::new_polygon(
            ctx,
            DrawMode::fill(),
            &triangle_points,
            Color::WHITE
        ).unwrap();

        canvas.draw(
            &self.triangle_mesh,
            DrawParam::default()
        );

        Ok(())
    }

    pub fn shoot(&self, ctx: &Context) -> Projectile {
        return Projectile::new(
            ctx,
            self.position.x,
            self.position.y,
            self.forward.x,
            self.forward.y
        );
    }

    pub fn clamp(&mut self) -> () {
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

    fn create_ship_triangle(ctx: &Context, position: Vec2, rotation: f32) -> Mesh {
        let triangle_points: [Vec2; 3] = Ship::get_triangle_points(position, rotation);

        return Mesh::new_polygon(
            ctx,
            DrawMode::fill(),
            &triangle_points,
            Color::WHITE
        ).unwrap();
    }

    fn get_triangle_points(position: Vec2, rotation: f32) -> [Vec2; 3] {
        return [
            position + Ship::rotate_triangle_point(Vec2::new(-20.0, -25.0), rotation - 90.0_f32.to_radians()),
            position + Ship::rotate_triangle_point(Vec2::new(20.0, -25.0), rotation - 90.0_f32.to_radians()),
            position + Ship::rotate_triangle_point(Vec2::new(0.0, 25.0), rotation - 90.0_f32.to_radians()),
        ];
    }

    fn rotate_triangle_point(triangle_point: Vec2, rotation: f32) -> Vec2 {
        let rotation_sin: f32 = rotation.sin();
        let rotation_cos: f32 = rotation.cos();
        let x_rotation: f32 = triangle_point.x * rotation_cos - triangle_point.y * rotation_sin;
        let y_rotation: f32 = triangle_point.x * rotation_sin + triangle_point.y * rotation_cos;

        return Vec2::new(x_rotation, y_rotation)
    }
}
