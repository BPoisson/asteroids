use std::time::{Instant};
use ggez::{graphics};
use ggez::glam::Vec2;
use ggez::graphics::{Canvas, Color, Rect};
use rand::Rng;
use rand::rngs::ThreadRng;

const PARTICLE_SPEED: f32 = 100.0;
const PARTICLE_DURATION_SECS: f32 = 1.5;

pub struct Particle {
    pub rect: Rect,
    pub creation_time: Instant,
    pub forward: Vec2,
    pub expired: bool
}

impl Particle {
    pub fn new(rng: &mut ThreadRng, position: &Vec2) -> Self {
        let particle_size: f32 = rng.gen_range(3.0..=5.0);

        let rect = Rect::new(
            position.x,
            position.y,
            particle_size,
            particle_size);
        let x_dir: f32 = rng.gen_range(-1.0..=1.0);
        let y_dir: f32 = rng.gen_range(-1.0..=1.0);

        return Particle {
            rect,
            creation_time: Instant::now(),
            forward: Vec2::new(x_dir, y_dir),
            expired: false
        }
    }

    pub fn draw(&self, canvas: &mut Canvas) -> () {
        canvas.draw(
            &graphics::Quad,
            graphics::DrawParam::new()
                .dest_rect(self.rect)
                .color(Color::WHITE));
    }

    pub fn move_forward(&mut self, dt: &f32) -> () {
        self.rect.x += self.forward.x * PARTICLE_SPEED * dt;
        self.rect.y += self.forward.y * PARTICLE_SPEED * dt;
    }

    pub fn create_particle_effect(rng: &mut ThreadRng, position: &Vec2, min_particles: u32, max_particles: u32) -> Vec<Self> {
        let mut particles: Vec<Particle> = Vec::new();

        for _ in 0..rng.gen_range(min_particles..=max_particles) {
            particles.push(
                Particle::new(rng, &position)
            )
        }
        return particles;
    }

    pub fn check_expiration(&mut self, now_time: &Instant) -> () {
        self.expired = now_time.duration_since(self.creation_time).as_secs_f32() > PARTICLE_DURATION_SECS;
    }
}