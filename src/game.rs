use std::collections::HashSet;
use std::time::Instant;
use ggez::{Context, event, GameError, GameResult, graphics};
use ggez::glam::Vec2;
use ggez::graphics::{Canvas, Color, PxScale, Text, TextLayout};
use ggez::input::keyboard::{KeyCode, KeyInput};
use rand::Rng;
use rand::rngs::ThreadRng;
use crate::alien::Alien;
use crate::asteroid::Asteroid;
use crate::{collision, save};
use crate::constants::SCREEN_SIZE;
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
    sounds: Sounds,
    paused: bool,
    game_over: bool
}

impl Game {
    pub fn new(ctx: &Context) -> Self {
        let mut rng: ThreadRng = rand::thread_rng();
        let now: Instant = Instant::now();

        Game {
            ship: Ship::new(ctx),
            asteroids: Game::initialize_asteroids(ctx, &mut rng),
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
            sounds: Sounds::new(ctx),
            paused: false,
            game_over: false
        }
    }

    fn initialize_asteroids(ctx: &Context, rng: &mut ThreadRng) -> Vec<Asteroid> {
        let mut asteroids: Vec<Asteroid> = Vec::new();

        for _ in 0..4 {
            asteroids.push(Asteroid::new(ctx, rng));
        }

        return asteroids;
    }

    fn alien_spawn_check(&mut self, now: &Instant) -> () {
        if self.alien.is_none() && now.duration_since(self.last_alien_spawn_check_instant).as_secs_f32() >= 10.0 {
            self.spawn_alien = self.rng.gen_bool(0.1);
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

    fn handle_game_updates(&mut self, ctx: &Context, dt: &f32, now: &Instant) -> () {
        // Ship updates.
        self.ship.move_forward(&dt);
        if !self.input_set.contains(&KeyCode::Up) {
            self.ship.apply_friction(&dt);
        }
        self.ship.handle_immune_timeout(&now);
        self.ship.update_collision_rect(&ctx);

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
    }

    fn get_pause_text(&mut self) -> Text {
        let high_score: u64 = save::get_high_score();
        let pause_string: String = format!("Game Paused!\n\nYour Score: {}\n\nHigh Score: {}\n\nPress Q To Quit", self.score.score, high_score);
        let mut pause_text: Text = Text::new(pause_string);
        pause_text.set_scale(PxScale::from(50.0));
        pause_text.set_layout(TextLayout::center());

        return pause_text;
    }

    fn get_game_over_text(&mut self) -> Text {
        let high_score: u64 = save::get_high_score();
        let pause_string: String = format!("Game Over!\n\nYour Score: {}\n\nHigh Score: {}\n\nPress R To Restart\n\nPress Q To Quit", self.score.score, high_score);
        let mut pause_text: Text = Text::new(pause_string);
        pause_text.set_scale(PxScale::from(50.0));
        pause_text.set_layout(TextLayout::center());

        return pause_text;
    }

    fn draw_text(&mut self, canvas: &mut Canvas, text: Text) -> () {
        canvas.draw(
            &text,
            graphics::DrawParam::default()
                .dest(Vec2::new(SCREEN_SIZE.x / 2.0, SCREEN_SIZE.y / 2.0))
        );
    }

    fn check_game_over(&mut self) -> () {
        if self.ship.health <= 0 {
            self.game_over = true;
        }
    }

    fn handle_reset(&mut self, ctx: &Context) -> () {
        let now: Instant = Instant::now();

        self.ship = Ship::new(ctx);
        self.asteroids = Game::initialize_asteroids(ctx, &mut self.rng);
        self.player_projectiles = Vec::new();
        self.alien_projectiles = Vec::new();
        self.particles = Vec::new();
        self.alien = None;
        self.score = Score::new();
        self.input_set = HashSet::new();
        self.last_update = now;
        self.last_asteroid_instant = now;
        self.last_alien_spawn_check_instant = now;
        self.spawn_alien = false;
        self.sounds = Sounds::new(ctx);
        self.paused = false;
        self.game_over = false;
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
        self.last_update = now;

        if self.paused || self.game_over {
            return Ok(());
        }

        self.alien_spawn_check(&now);

        self.handle_input(&dt);

        self.handle_game_updates(ctx, &dt, &now);

        // Handle player_projectile collision with asteroids.
        let mut player_projectile_new_asteroids_particles_tuple: (Vec<Asteroid>, Vec<Particle>) =
            collision::handle_player_projectile_collisions(
                ctx,
                &mut self.rng,
                &mut self.alien,
                &mut self.player_projectiles,
                &mut self.asteroids,
                &mut self.score,
                &mut self.sounds);

        let mut alien_projectile_new_asteroids_particles_tuple: (Vec<Asteroid>, Vec<Particle>) =
            collision::handle_alien_projectile_collisions(
                ctx,
                &mut self.rng,
                &mut self.ship,
                &mut self.alien_projectiles,
                &mut self.asteroids,
                &mut self.score,
                &mut self.sounds);

        if let Some(particles) = &mut collision::handle_ship_asteroid_collisions(ctx, &mut self.rng, &mut self.ship, &self.asteroids, &mut self.sounds) {
            player_projectile_new_asteroids_particles_tuple.1.append(particles);
        }

        // Spawn another asteroid
        if self.asteroids.len() < 4 || (self.asteroids.len() < 10 && now.duration_since(self.last_asteroid_instant).as_secs_f32() > 8.0) {
            player_projectile_new_asteroids_particles_tuple.0.push(Asteroid::new(ctx, &mut self.rng));
            self.last_asteroid_instant = now;
        }

        // Free destroyed and expired assets.
        self.clean_up(ctx, &now);

        self.asteroids.append(&mut player_projectile_new_asteroids_particles_tuple.0);
        self.asteroids.append(&mut alien_projectile_new_asteroids_particles_tuple.0);
        self.particles.append(&mut player_projectile_new_asteroids_particles_tuple.1);
        self.particles.append(&mut alien_projectile_new_asteroids_particles_tuple.1);

        self.check_game_over();

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas: Canvas = Canvas::from_frame(ctx, Color::BLACK);

        if !self.game_over {
            self.ship.draw(ctx, &mut canvas, &mut self.rng);
        }

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

        if self.paused {
            let pause_text: Text = self.get_pause_text();
            self.draw_text(&mut canvas, pause_text);
        }

        if self.game_over {
            let game_over_text: Text = self.get_game_over_text();
            self.draw_text(&mut canvas, game_over_text);
        }

        canvas.finish(ctx)?;
        Ok(())
    }

    fn key_down_event(&mut self, ctx: &mut Context, input: KeyInput, _repeated: bool) -> Result<(), GameError> {
        let register_actions: bool = !self.paused && !self.game_over;

        if let Some(key) = input.keycode {
            if key == KeyCode::Up && register_actions {
                self.ship.thrusting = true;
                self.sounds.play_thrust_sound(ctx);
            } else if key == KeyCode::Space && !self.input_set.contains(&key) && register_actions {
                let player_projectile: Projectile = self.ship.shoot(ctx);

                self.player_projectiles.push(player_projectile);

                self.sounds.play_player_shoot_sound(ctx);
            } else if !self.game_over && key == KeyCode::Escape {
                self.paused = !self.paused;
            } else if key == KeyCode::Q && (self.paused || self.game_over) {
                save::save_high_score(self.score.score);
                ctx.request_quit();
            } else if key == KeyCode::R && self.game_over {
                save::save_high_score(self.score.score);
                self.handle_reset(ctx);
            }

            if key != KeyCode::Escape && key != KeyCode::Q && key != KeyCode::R {
                self.input_set.insert(key);
            }
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