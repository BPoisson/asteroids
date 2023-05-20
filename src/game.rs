use std::collections::HashSet;
use std::time::Instant;
use ggez::{Context, event, GameError, GameResult};
use ggez::graphics::{Canvas, Color};
use ggez::input::keyboard::{KeyCode, KeyInput};
use rand::Rng;
use rand::rngs::ThreadRng;
use crate::alien::Alien;
use crate::asteroid::Asteroid;
use crate::collision;
use crate::particle::Particle;
use crate::projectile::Projectile;
use crate::score::Score;
use crate::ship::{RotationDirection, Ship};
use crate::sounds::Sounds;

pub struct Game {
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
    last_alien_spawn_check_instant: Instant,
    spawn_alien: bool,
    sounds: Sounds
}

impl Game {
    pub fn new(ctx: &Context) -> Self {
        let mut rng: ThreadRng = rand::thread_rng();
        let now: Instant = Instant::now();

        let mut asteroids: Vec<Asteroid> = Vec::new();

        for _ in 0..3 {
            asteroids.push(Asteroid::new(ctx, &mut rng));
        }

        Game {
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
            last_alien_spawn_check_instant: now,    // Set to now so we don't spawn an Alien right away.
            spawn_alien: false,
            sounds: Sounds::new(ctx)
        }
    }

    fn alien_spawn_check(&mut self, now: &Instant) -> () {
        if self.alien.is_none() && now.duration_since(self.last_alien_spawn_check_instant).as_secs_f32() >= 10.0 {
            self.spawn_alien = self.rng.gen_bool(0.2);
            self.last_alien_spawn_check_instant = *now;
        }
    }

    fn handle_input(&mut self, dt: &f32) -> () {
        for key in &self.input_set {
            match key {
                KeyCode::Up => {
                    self.ship.apply_thrust(dt);
                }
                KeyCode::Left => {
                    self.ship.rotate(RotationDirection::LEFT, dt);
                }
                KeyCode::Right => {
                    self.ship.rotate(RotationDirection::RIGHT, dt);
                }
                _ => ()
            }
        }
    }

    fn clean_up(&mut self, ctx: &Context, now: &Instant) -> () {
        self.player_projectiles.retain(|p| !p.expired);
        self.alien_projectiles.retain(|p| !p.expired);
        self.asteroids.retain(|a| !a.destroyed);
        self.particles.retain(|p| !p.expired);

        if let Some(alien) = &mut self.alien {
            alien.check_expiration(&Instant::now());

            if alien.expired {
                self.alien = None;
                self.last_alien_spawn_check_instant = *now;     // Avoid spawning an Alien right after the last one.

                self.sounds.stop_alien_music(ctx);
                self.sounds.stop_alien_warning_sound(ctx);
            }
        }
    }
}

impl event::EventHandler<GameError> for Game {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let now: Instant = Instant::now();
        let dt: f32 = now.duration_since(self.last_update).as_secs_f32();

        self.alien_spawn_check(&now);

        self.handle_input(&dt);

        // Ship updates.
        self.ship.move_forward(&dt);
        if !self.input_set.contains(&KeyCode::Up) {
            self.ship.apply_friction(&dt);
        }

        // Alien updates.
        if let Some(alien) = &mut self.alien {
            alien.move_forward(&mut self.rng, &dt);
            alien.update_aim(&self.ship.position);

            if let Some(alien_projectile) = alien.shoot(ctx, &mut self.rng, &now) {
                self.alien_projectiles.push(alien_projectile);

                self.sounds.play_alien_shoot_sound(ctx);
            }
        } else if self.spawn_alien {
            // Random chance to spawn the alien if it does not exist.
            self.alien = Some(Alien::new(ctx, &mut self.rng));
            self.spawn_alien = false;

            self.sounds.play_alien_music(ctx);
            self.sounds.play_alien_warning_sound(ctx);
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

        // Handle player_projectile collision with asteroids.
        let mut new_asteroids_particles_tuple: (Vec<Asteroid>, Vec<Particle>) =
            collision::handle_collisions(
                ctx,
                &mut self.rng,
                &mut self.ship,
                &mut self.alien,
                &mut self.player_projectiles,
                &mut self.asteroids,
                &mut self.score,
                &mut self.sounds);

        // Spawn another asteroid
        if self.asteroids.len() < 3 || now.duration_since(self.last_asteroid_instant).as_secs_f32() > 8.0 {
            new_asteroids_particles_tuple.0.push(Asteroid::new(ctx, &mut self.rng));
            self.last_asteroid_instant = now;
        }

        // Free destroyed and expired assets.
        self.clean_up(ctx, &now);

        self.asteroids.append(&mut new_asteroids_particles_tuple.0);
        self.particles.append(&mut new_asteroids_particles_tuple.1);

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

                self.sounds.play_player_shoot_sound(ctx);
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