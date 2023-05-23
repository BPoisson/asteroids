use std::ops::Neg;
use std::time::Instant;
use ggez::{Context};
use ggez::glam::{Vec2};
use ggez::graphics::{Canvas, Color, DrawMode, DrawParam, Mesh, Rect};
use rand::Rng;
use rand::rngs::ThreadRng;
use crate::{SCREEN_SIZE};
use crate::projectile::{Projectile};

pub const FRICTION: f32 = 0.30;
pub const SPEED: f32 = 350.0;
pub const ROTATION_RADIANS: f32 = 270_f32;

pub enum RotationDirection {
    LEFT,
    RIGHT
}

pub struct Ship {
    pub triangle_mesh: Mesh,
    pub exhaust_mesh: Mesh,
    pub collision_rect_ranges: [[f32; 2]; 2],
    pub collision_rect_mesh: Mesh,
    pub position: Vec2,
    pub rotation: f32,
    pub forward: Vec2,
    pub thrust: Vec2,
    pub thrusting: bool,
    pub health: i32,
    pub immune: bool,
    pub immune_instant: Instant
}

impl Ship {
    pub fn new(ctx: &Context) -> Self {
        let position: Vec2 = Vec2::new(SCREEN_SIZE.x / 2.0, SCREEN_SIZE.y / 2.0);
        let rotation: f32 = 270.0_f32.to_radians();
        let triangle_points: [Vec2; 3] = Ship::get_triangle_points(&position, &rotation);
        let exhaust_points: [Vec2; 7] = Ship::get_exhaust_points(&position, &rotation);
        let triangle_mesh: Mesh = Ship::create_ship_triangle(ctx, &triangle_points);
        let exhaust_mesh: Mesh = Ship::create_exhaust(ctx, &exhaust_points);
        let collision_rect_ranges: [[f32; 2]; 2] = Ship::get_collision_rect_ranges(&triangle_points);
        let collision_rect_mesh: Mesh = Ship::create_collision_rect(ctx, &collision_rect_ranges);

        return Ship {
            triangle_mesh,
            exhaust_mesh,
            collision_rect_ranges,
            collision_rect_mesh,
            position,
            rotation,
            forward: Vec2::new(0.0, -1.0),
            thrust: Vec2::new(0.0, 0.0),
            thrusting: false,
            health: 5,
            immune: false,
            immune_instant: Instant::now()
        }
    }

    pub fn draw(&mut self, ctx: &Context, canvas: &mut Canvas, rng: &mut ThreadRng) -> () {
        let triangle_points: [Vec2; 3] = Ship::get_triangle_points(&self.position, &self.rotation);
        let render_range_max: u32 = if self.immune {2} else {1};

        self.triangle_mesh = Ship::create_ship_triangle(ctx, &triangle_points);

        // Flicker Ship when immune.
        if !self.immune || rng.gen_range(0..=render_range_max) == 0 {
            canvas.draw(
                &self.triangle_mesh,
                DrawParam::default()
            );
        }

        // Draw flickering exhaust when thrusting.
        if self.thrusting && rng.gen_range(0..=render_range_max) == 0 {
            let exhaust_points: [Vec2; 7] = Ship::get_exhaust_points(&self.position, &self.rotation);

            self.exhaust_mesh = Ship::create_exhaust(ctx, &exhaust_points);

            canvas.draw(
                &self.exhaust_mesh,
                DrawParam::default()
            );
        }

        canvas.draw(
            &self.collision_rect_mesh,
            DrawParam::default()
        );
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

    pub fn rotate(&mut self, rotation_direction: RotationDirection, dt: &f32) -> () {
        let mut rotation: f32 = ROTATION_RADIANS.to_radians();

        match rotation_direction {
            RotationDirection::LEFT => rotation = rotation.neg(),
            _ => ()
        }

        self.rotation += rotation * dt;
        self.forward.x = self.rotation.cos();
        self.forward.y = self.rotation.sin();
    }

    pub fn shoot(&self, ctx: &Context) -> Projectile {
        return Projectile::new(
            ctx,
            &self.position,
            &self.forward,
            Color::WHITE
        );
    }

    pub fn update_collision_rect(&mut self, ctx: &Context) -> () {
        let triangle_points: [Vec2; 3] = Ship::get_triangle_points(&self.position, &self.rotation);

        self.collision_rect_ranges = Ship::get_collision_rect_ranges(&triangle_points);
        self.collision_rect_mesh = Ship::create_collision_rect(ctx, &self.collision_rect_ranges);
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

    fn create_ship_triangle(ctx: &Context, triangle_points: &[Vec2; 3]) -> Mesh {
        return Mesh::new_polygon(
            ctx,
            DrawMode::stroke(2.0),
            triangle_points,
            Color::WHITE
        ).unwrap();
    }

    fn create_exhaust(ctx: &Context, exhaust_points: &[Vec2; 7]) -> Mesh {
        return Mesh::new_polygon(
            ctx,
            DrawMode::stroke(2.0),
            exhaust_points,
            Color::WHITE
        ).unwrap();
    }

    fn get_collision_rect_ranges(triangle_points: &[Vec2; 3]) -> [[f32; 2]; 2] {
        let mut min_x: f32 = f32::MAX;
        let mut max_x: f32 = f32::MIN;
        let mut min_y: f32 = f32::MAX;
        let mut max_y: f32 = f32::MIN;

        for point in triangle_points {
            min_x = f32::min(min_x, point.x);
            max_x = f32::max(max_x, point.x);
            min_y = f32::min(min_y, point.y);
            max_y = f32::max(max_y, point.y);
        }

        return [
            [min_x, min_y],
            [max_x, max_y]
        ];
    }

    fn create_collision_rect(ctx: &Context, collision_rect_ranges: &[[f32; 2]; 2]) -> Mesh {
        let rect: Rect = Rect::new(
            collision_rect_ranges[0][0],
            collision_rect_ranges[0][1],
            collision_rect_ranges[1][0] - collision_rect_ranges[0][0],
            collision_rect_ranges[1][1] - collision_rect_ranges[0][1]);

        return Mesh::new_rectangle(
            ctx,
            DrawMode::stroke(1.0),
            rect,
            Color::BLUE
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

    pub fn handle_immune_timeout(&mut self, now: &Instant) -> () {
        if self.immune && now.duration_since(self.immune_instant).as_secs_f32() > 5.0 {
            self.immune = false;
        }
    }
}
