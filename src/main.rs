mod ship;
mod projectile;
mod asteroid;
mod constants;
mod collision;
mod particle;

use std::collections::HashSet;
use std::error::Error;
use std::time::{Instant};
use ggez::{Context, ContextBuilder, event, GameError, GameResult};
use ggez::conf::{WindowMode, WindowSetup};
use ggez::glam::Vec2;
use ggez::graphics::{Canvas, Color};
use ggez::input::keyboard::{KeyCode, KeyInput};
use rand::rngs::ThreadRng;
use crate::asteroid::Asteroid;
use crate::constants::{SCREEN_SIZE};
use crate::particle::Particle;
use crate::projectile::Projectile;
use crate::ship::Ship;

const GAME_ID: &str = "Asteroids";
const AUTHOR: &str = "BPoisson";

struct GameState {
    ship: Ship,
    asteroids: Vec<Asteroid>,
    projectiles: Vec<Projectile>,
    particles: Vec<Particle>,
    input_set: HashSet<KeyCode>,
    last_update: Instant,
    rng: ThreadRng,
    last_asteroid_instant: Instant
}

impl GameState {
    fn new(ctx: &Context) -> Self {
        let mut rng: ThreadRng = rand::thread_rng();
        let now: Instant = Instant::now();

        let mut asteroids: Vec<Asteroid> = Vec::new();

        for _ in 0..4 {
            asteroids.push(Asteroid::new(ctx, &mut rng));
        }

        GameState {
            ship: Ship::new(ctx),
            asteroids,
            projectiles: Vec::new(),
            particles: Vec::new(),
            input_set: HashSet::new(),
            last_update: now,
            rng,
            last_asteroid_instant: now

        }
    }
}

impl event::EventHandler<GameError> for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let now: Instant = Instant::now();
        let dt: f32 = (now - self.last_update).as_secs_f32();

        for key in &self.input_set {
            match key {
                KeyCode::Up => {
                    self.ship.apply_thrust(dt);
                }
                KeyCode::Left => {
                    self.ship.rotate(-270_f32.to_radians(), dt);
                }
                KeyCode::Right => {
                    self.ship.rotate(270_f32.to_radians(), dt);
                }
                _ => ()
            }
        }

        // Ship updates.
        self.ship.move_forward(dt);
        if !self.input_set.contains(&KeyCode::Up) {
            self.ship.apply_friction(dt);
        }

        // Projectile updates.
        for i in 0..self.projectiles.len() {
            if let Some(projectile) = self.projectiles.get_mut(i) {
                projectile.move_forward(ctx, dt);
                projectile.set_out_of_bounds();
            }
        }

        // Asteroid updates
        for i in 0..self.asteroids.len() {
            if let Some(asteroid) = self.asteroids.get_mut(i) {
                asteroid.move_forward(ctx, dt);
            }
        }

        // Particle updates
        for i in 0..self.asteroids.len() {
            if let Some(particle) = self.particles.get_mut(i) {
                particle.move_forward(dt);
                particle.check_expiration(Instant::now());
            }
        }

        let mut new_asteroids: Vec<Asteroid> = Vec::new();
        let mut new_particles: Vec<Particle> = Vec::new();
        // Handle projectile collision with asteroids.
        for i in 0..self.projectiles.len() {
            if let Some(projectile) = self.projectiles.get_mut(i) {
                for j in 0..self.asteroids.len() {
                    if let Some(asteroid) = self.asteroids.get_mut(j) {
                        if collision::projectile_hit(projectile, asteroid) {
                            // Add to particle effect
                            new_particles.append(&mut Particle::create_particle_effect(&mut self.rng, asteroid.position));
                            // Asteroid breaks into smaller pieces.
                            new_asteroids.append(&mut asteroid.destroy_asteroid(ctx, &mut self.rng));

                            projectile.to_remove = true;
                        }
                    }
                }
            }
        }

        // Spawn another asteroid
        if self.asteroids.len() < 3 || (now - self.last_asteroid_instant).as_secs_f32() > 8.0 {
            new_asteroids.push(Asteroid::new(ctx, &mut self.rng));
            self.last_asteroid_instant = now;
        }

        self.projectiles.retain(|p| !p.to_remove);
        self.asteroids.retain(|a| !a.destroyed);
        self.particles.retain(|p| !p.expired);

        self.asteroids.append(&mut new_asteroids);
        self.particles.append(&mut new_particles);

        self.last_update = Instant::now();
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas: Canvas = Canvas::from_frame(ctx, Color::BLACK);

        self.ship.draw(ctx, &mut canvas);

        for projectile in &mut self.projectiles {
            projectile.draw(&mut canvas);
        }

        for asteroid in &mut self.asteroids {
            asteroid.draw(&mut canvas);
        }

        for particle in &mut self.particles {
            particle.draw(&mut canvas);
        }

        canvas.finish(ctx)?;
        Ok(())
    }

    fn key_down_event(&mut self, ctx: &mut Context, input: KeyInput, _repeated: bool) -> Result<(), GameError> {
        if let Some(key) = input.keycode {
            if key == KeyCode::Space && !self.input_set.contains(&key) {
                let projectile: Projectile = self.ship.shoot(ctx);

                self.projectiles.push(projectile);
            }
            self.input_set.insert(key);
        }

        Ok(())
    }

    fn key_up_event(&mut self, _ctx: &mut Context, input: KeyInput) -> Result<(), GameError> {
        if let Some(key) = input.keycode {
            self.input_set.remove(&key);
        }

        Ok(())
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let (ctx, event_loop) = ContextBuilder::new(GAME_ID, AUTHOR)
        .window_setup(WindowSetup::default().title(GAME_ID))
        .window_mode(WindowMode::default().dimensions(SCREEN_SIZE.x, SCREEN_SIZE.y))
        .build()?;

    let game_state: GameState = GameState::new(&ctx);

    event::run(ctx, event_loop, game_state);
}

