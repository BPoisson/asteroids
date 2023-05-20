use ggez::{Context};
use ggez::glam::{Vec2};
use ggez::graphics::{Canvas, Color, DrawMode, DrawParam, Mesh};
use rand::Rng;
use rand::rngs::ThreadRng;
use crate::{SCREEN_SIZE};
use crate::projectile::{Projectile};

pub const FRICTION: f32 = 0.35;
pub const SPEED: f32 = 400.0;

pub struct Ship {
    pub triangle_mesh: Mesh,
    pub exhaust_mesh: Mesh,
    pub position: Vec2,
    pub rotation: f32,
    pub forward: Vec2,
    pub thrust: Vec2,
    pub thrusting: bool,
    pub health: i32
}

impl Ship {
    pub fn new(ctx: &Context) -> Self {
        let position: Vec2 = Vec2::new(SCREEN_SIZE.x / 2.0, SCREEN_SIZE.y / 2.0);
        let rotation: f32 = 270.0_f32.to_radians();
        let triangle_mesh: Mesh = Ship::create_ship_triangle(ctx, &position, &rotation);
        let exhaust_mesh: Mesh = Ship::create_exhaust(ctx, &position, &rotation);

        return Ship {
            triangle_mesh,
            exhaust_mesh,
            position,
            rotation,
            forward: Vec2::new(0.0, -1.0),
            thrust: Vec2::new(0.0, 0.0),
            thrusting: false,
            health: 5
        }
    }

    pub fn draw(&mut self, ctx: &Context, canvas: &mut Canvas, rng: &mut ThreadRng) -> () {
        let triangle_points: [Vec2; 3] = Ship::get_triangle_points(&self.position, &self.rotation);

        self.triangle_mesh = Mesh::new_polygon(
            ctx,
            DrawMode::stroke(2.0),
            &triangle_points,
            Color::WHITE
        ).unwrap();

        canvas.draw(
            &self.triangle_mesh,
            DrawParam::default()
        );

        // Only draw exhaust when thrusting and only 50% of frames.
        if self.thrusting && rng.gen_range(0..=1) == 0 {
            let exhaust_points: [Vec2; 7] = Ship::get_exhaust_points(&self.position, &self.rotation);

            self.exhaust_mesh = Mesh::new_polygon(
                ctx,
                DrawMode::stroke(2.0),
                &exhaust_points,
                Color::WHITE
            ).unwrap();

            canvas.draw(
                &self.exhaust_mesh,
                DrawParam::default()
            );
        }
    }

    pub fn apply_thrust(&mut self, dt: &f32) -> () {
        self.thrust.x += self.forward.x * dt * 2.0;
        self.thrust.y += self.forward.y * dt * 2.0;
        self.clamp_thrust();
    }

    pub fn apply_friction(&mut self, dt: &f32) -> () {
        if self.thrust.x > 0.0 {
            self.thrust.x -= FRICTION * dt;
        } else if self.thrust.x < 0.0 {
            self.thrust.x += FRICTION * dt;
        }

        if self.thrust.y > 0.0 {
            self.thrust.y -= FRICTION * dt;
        } else if self.thrust.y < 0.0 {
            self.thrust.y += FRICTION * dt;
        }
    }

    pub fn move_forward(&mut self, dt: &f32) -> () {
        self.position.x += self.thrust.x * SPEED * dt;
        self.position.y += self.thrust.y * SPEED * dt;
        self.clamp_position();
    }

    pub fn rotate(&mut self, radians: f32, dt: &f32) -> () {
        self.rotation += radians * dt;
        self.forward.x = self.rotation.cos();
        self.forward.y = self.rotation.sin();
    }

    pub fn shoot(&self, ctx: &Context) -> Projectile {
        return Projectile::new(
            ctx,
            &self.position,
            &self.forward
        );
    }

    pub fn clamp_thrust(&mut self) -> () {
        if self.thrust.x > 1.0 {
            self.thrust.x = 1.0;
        } else if self.thrust.x < -1.0 {
            self.thrust.x = -1.0;
        }
        if self.thrust.y > 1.0 {
            self.thrust.y = 1.0;
        } else if self.thrust.y < -1.0 {
            self.thrust.y = -1.0;
        }
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

    fn create_ship_triangle(ctx: &Context, position: &Vec2, rotation: &f32) -> Mesh {
        let triangle_points: [Vec2; 3] = Ship::get_triangle_points(position, rotation);

        return Mesh::new_polygon(
            ctx,
            DrawMode::stroke(2.0),
            &triangle_points,
            Color::WHITE
        ).unwrap();
    }

    fn create_exhaust(ctx: &Context, position: &Vec2, rotation: &f32) -> Mesh {
        let exhaust_points: [Vec2; 7] = Ship::get_exhaust_points(position, rotation);

        return Mesh::new_polygon(
            ctx,
            DrawMode::stroke(2.0),
            &exhaust_points,
            Color::WHITE
        ).unwrap();
    }

    fn get_triangle_points(position: &Vec2, rotation: &f32) -> [Vec2; 3] {
        return [
            *position + Ship::rotate_point(Vec2::new(-20.0, -25.0), rotation - 90.0_f32.to_radians()),
            *position + Ship::rotate_point(Vec2::new(20.0, -25.0), rotation - 90.0_f32.to_radians()),
            *position + Ship::rotate_point(Vec2::new(0.0, 25.0), rotation - 90.0_f32.to_radians()),
        ];
    }

    fn get_exhaust_points(position: &Vec2, rotation: &f32) -> [Vec2; 7] {
        return [
            *position + Ship::rotate_point(Vec2::new(-10.0, -25.0), rotation - 90.0_f32.to_radians()),
            *position + Ship::rotate_point(Vec2::new(-10.0, -35.0), rotation - 90.0_f32.to_radians()),
            *position + Ship::rotate_point(Vec2::new(-5.0, -30.0), rotation - 90.0_f32.to_radians()),
            *position + Ship::rotate_point(Vec2::new(0.0, -50.0), rotation - 90.0_f32.to_radians()),
            *position + Ship::rotate_point(Vec2::new(5.0, -30.0), rotation - 90.0_f32.to_radians()),
            *position + Ship::rotate_point(Vec2::new(10.0, -35.0), rotation - 90.0_f32.to_radians()),
            *position + Ship::rotate_point(Vec2::new(10.0, -25.0), rotation - 90.0_f32.to_radians()),
        ];
    }

    fn rotate_point(triangle_point: Vec2, rotation: f32) -> Vec2 {
        let rotation_sin: f32 = rotation.sin();
        let rotation_cos: f32 = rotation.cos();
        let x_rotation: f32 = triangle_point.x * rotation_cos - triangle_point.y * rotation_sin;
        let y_rotation: f32 = triangle_point.x * rotation_sin + triangle_point.y * rotation_cos;

        return Vec2::new(x_rotation, y_rotation)
    }
}
