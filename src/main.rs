mod ship;
mod projectile;
mod asteroid;
mod constants;
mod collision;
mod particle;
mod sounds;
mod score;
mod alien;

use std::collections::HashSet;
use std::error::Error;
use std::time::{Instant};
use ggez::{Context, ContextBuilder, event, GameError, GameResult};
use ggez::conf::{WindowMode, WindowSetup};
use ggez::glam::Vec2;
use ggez::graphics::{Canvas, Color};
use ggez::input::keyboard::{KeyCode, KeyInput};
use rand::Rng;
use rand::rngs::ThreadRng;
use crate::asteroid::{Asteroid};
use crate::constants::{SCREEN_SIZE};
use crate::alien::{Alien};
use crate::particle::Particle;
use crate::projectile::Projectile;
use crate::score::Score;
use crate::ship::Ship;
use crate::sounds::{Sounds};

const GAME_ID: &str = "Asteroids";
const AUTHOR: &str = "BPoisson";

struct GameState {
    ship: Ship,
    asteroids: Vec<Asteroid>,
    player_projectiles: Vec<Projectile>,
    alien_projectiles: Vec<Projectile>,
    particles: Vec<Particle>,
    alien: Option<Alien>,
    score: Score,
    input_set: HashSet<KeyCode>,
    last_update: Instant,
    rng: ThreadRng,
    last_asteroid_instant: Instant,
    sounds: Sounds
}

impl GameState {
    fn new(ctx: &Context) -> Self {
        let mut rng: ThreadRng = rand::thread_rng();
        let now: Instant = Instant::now();

        let mut asteroids: Vec<Asteroid> = Vec::new();

        for _ in 0..3 {
            asteroids.push(Asteroid::new(ctx, &mut rng));
        }

        GameState {
            ship: Ship::new(ctx),
            asteroids,
            player_projectiles: Vec::new(),
            alien_projectiles: Vec::new(),
            particles: Vec::new(),
            alien: None,
            score: Score::new(),
            input_set: HashSet::new(),
            last_update: now,
            rng,
            last_asteroid_instant: now,
            sounds: Sounds::new(ctx)
        }
    }

    fn clean_up(&mut self) -> () {
        self.player_projectiles.retain(|p| !p.expired);
        self.alien_projectiles.retain(|p| !p.expired);
        self.asteroids.retain(|a| !a.destroyed);
        self.particles.retain(|p| !p.expired);
    }
}

impl event::EventHandler<GameError> for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let now: Instant = Instant::now();
        let dt: f32 = (now - self.last_update).as_secs_f32();

        for key in &self.input_set {
            match key {
                KeyCode::Up => {
                    self.ship.apply_thrust(&dt);
                }
                KeyCode::Left => {
                    self.ship.rotate(-360_f32.to_radians(), &dt);
                }
                KeyCode::Right => {
                    self.ship.rotate(360_f32.to_radians(), &dt);
                }
                _ => ()
            }
        }

        // Ship updates.
        self.ship.move_forward(&dt);
        if !self.input_set.contains(&KeyCode::Up) {
            self.ship.apply_friction(&dt);
        }

        // Alien updates.
        if let Some(alien) = &mut self.alien {
            alien.move_forward(&mut self.rng, &dt);
            alien.update_aim(&self.ship.position);

            if self.rng.gen_range(0..500) == 0 {
                let alien_projectile: Projectile = alien.shoot(&ctx);

                self.alien_projectiles.push(alien_projectile);

                self.sounds.play_alien_shot(&ctx);
            }

            alien.check_expiration(&Instant::now());

            if alien.expired {
                self.alien = None;
                self.sounds.stop_alien_music(&ctx);
                self.sounds.stop_alien_warning(&ctx);
            }
        } else if self.rng.gen_range(0..=1) == 0 {
            // Random chance to spawn the alien if it does not exist.
            self.alien = Some(Alien::new(&ctx, &mut self.rng));
            self.sounds.play_alien_music(&ctx);
            self.sounds.play_alien_warning(&ctx);
        }

        // Player projectile updates.
        for i in 0..self.player_projectiles.len() {
            if let Some(player_projectile) = self.player_projectiles.get_mut(i) {
                player_projectile.move_forward(ctx, &dt);
                player_projectile.set_out_of_bounds();
            }
        }

        // Alien projectile updates.
        for i in 0..self.alien_projectiles.len() {
            if let Some(alien_projectile) = self.alien_projectiles.get_mut(i) {
                alien_projectile.move_forward(ctx, &dt);
                alien_projectile.set_out_of_bounds();
            }
        }

        // Asteroid updates.
        for i in 0..self.asteroids.len() {
            if let Some(asteroid) = self.asteroids.get_mut(i) {
                asteroid.move_forward(ctx, &dt);
            }
        }

        // Particle updates.
        for i in 0..self.asteroids.len() {
            if let Some(particle) = self.particles.get_mut(i) {
                particle.move_forward(&dt);
                particle.check_expiration(&Instant::now());
            }
        }

        let mut new_asteroids: Vec<Asteroid> = Vec::new();
        let mut new_particles: Vec<Particle> = Vec::new();

        // Handle player_projectile collision with asteroids.
        for i in 0..self.player_projectiles.len() {
            if let Some(player_projectile) = self.player_projectiles.get_mut(i) {
                // Check Alien collisions.
                if let Some(alien) = &mut self.alien {
                    // Destroy alien and projectile when hit.
                    if collision::projectile_alien_hit(player_projectile, alien) {
                        new_particles.append(&mut Particle::create_particle_effect(&mut self.rng, &alien.position, 6, 10));

                        player_projectile.expired = true;
                        alien.health -= 1;

                        if alien.health <= 0 {
                            alien.expired = true;

                            self.score.update_score_alien();
                            self.sounds.play_alien_explosion(ctx);
                        }
                        continue; // Skip this projectile as it has collided with an Alien.
                    }
                }
                // Check Asteroid collisions.
                for j in 0..self.asteroids.len() {
                    if let Some(asteroid) = self.asteroids.get_mut(j) {
                        if collision::projectile_asteroid_hit(player_projectile, asteroid) {
                            // Add to particle effect
                            new_particles.append(&mut Particle::create_particle_effect(&mut self.rng, &asteroid.position, 3, 5));
                            // Asteroid breaks into smaller pieces.
                            new_asteroids.append(&mut asteroid.destroy_asteroid(ctx, &mut self.rng));

                            player_projectile.expired = true;

                            self.score.update_score_asteroid(&asteroid.size);

                            self.sounds.play_asteroid_break_sound(ctx, &asteroid.size);
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

        // Free destroyed and expired assets.
        self.clean_up();

        self.asteroids.append(&mut new_asteroids);
        self.particles.append(&mut new_particles);

        self.last_update = Instant::now();
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas: Canvas = Canvas::from_frame(ctx, Color::BLACK);

        self.ship.draw(ctx, &mut canvas, &mut self.rng);

        for player_projectile in &mut self.player_projectiles {
            player_projectile.draw(&mut canvas);
        }

        for alien_projectile in &mut self.alien_projectiles {
            alien_projectile.draw(&mut canvas);
        }

        for asteroid in &mut self.asteroids {
            asteroid.draw(&mut canvas);
        }

        for particle in &mut self.particles {
            particle.draw(&mut canvas);
        }

        if let Some(alien) = &mut self.alien {
            alien.draw(ctx, &mut canvas);
        }

        self.score.draw(&mut canvas);

        canvas.finish(ctx)?;
        Ok(())
    }

    fn key_down_event(&mut self, ctx: &mut Context, input: KeyInput, _repeated: bool) -> Result<(), GameError> {
        if let Some(key) = input.keycode {
            if key == KeyCode::Up {
                self.ship.thrusting = true;
                self.sounds.play_thrust_sound(ctx);
            } else if key == KeyCode::Space && !self.input_set.contains(&key) {
                let player_projectile: Projectile = self.ship.shoot(ctx);

                self.player_projectiles.push(player_projectile);

                self.sounds.play_shoot_sound(ctx);
            }
            self.input_set.insert(key);
        }

        Ok(())
    }

    fn key_up_event(&mut self, ctx: &mut Context, input: KeyInput) -> Result<(), GameError> {
        if let Some(key) = input.keycode {
            self.input_set.remove(&key);

            if key == KeyCode::Up {
                self.ship.thrusting = false;
                self.sounds.stop_thrust_sound(ctx);
            }
        }

        Ok(())
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let (ctx, event_loop) = ContextBuilder::new(GAME_ID, AUTHOR)
        .window_setup(WindowSetup::default().title(GAME_ID))
        .window_mode(WindowMode::default().dimensions(SCREEN_SIZE.x, SCREEN_SIZE.y))
        .add_resource_path("resources")
        .build()?;

    let game_state: GameState = GameState::new(&ctx);

    event::run(ctx, event_loop, game_state);
}

