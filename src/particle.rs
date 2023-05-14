use std::time::{Instant};
use ggez::{GameError, graphics};
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
    pub fn new(position: Vec2) -> Particle {
        let mut rng: ThreadRng = rand::thread_rng();
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

    pub fn draw(&self, canvas: &mut Canvas) -> Result<(), GameError> {
        canvas.draw(
            &graphics::Quad,
            graphics::DrawParam::new()
                .dest_rect(self.rect)
                .color(Color::WHITE));

        Ok(())
    }

    pub fn move_forward(&mut self, dt: f32) -> Result<(), GameError> {
        self.rect.x += self.forward.x * PARTICLE_SPEED * dt;
        self.rect.y += self.forward.y * PARTICLE_SPEED * dt;

        Ok(())
    }

    pub fn create_particle_effect(position: Vec2) -> Vec<Particle> {
        let mut particles: Vec<Particle> = Vec::new();

        for _ in 0..4 {
            particles.push(
                Particle::new(position)
            )
        }
        return particles;
    }

    pub fn check_expiration(&mut self, now_time: Instant) -> Result<(), GameError> {
        self.expired = (now_time - self.creation_time).as_secs_f32() > PARTICLE_DURATION_SECS;

        Ok(())
    }
}