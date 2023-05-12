mod ship;
mod projectile;
mod asteroid;
mod constants;

use std::collections::HashSet;
use std::time::{Duration, Instant};
use ggez::{Context, ContextBuilder, event, GameError, GameResult};
use ggez::conf::{WindowMode, WindowSetup};
use ggez::glam::Vec2;
use ggez::graphics::{Canvas, Color};
use ggez::input::keyboard::{KeyCode, KeyInput};
use crate::asteroid::Asteroid;
use crate::constants::{MILLIS_PER_FRAME, SCREEN_SIZE};
use crate::projectile::Projectile;
use crate::ship::Ship;

const GAME_ID: &str = "Asteroids";
const AUTHOR: &str = "BPoisson";

struct GameState {
    ship: Ship,
    asteroids: Vec<Asteroid>,
    projectiles: Vec<Projectile>,
    input_set: HashSet<KeyCode>,
    last_update: Instant
}

impl GameState {
    fn new(ctx: &Context) -> Self {
        let mut asteroids: Vec<Asteroid> = Vec::new();

        for _ in 0..4 {
            asteroids.push(Asteroid::new(ctx));
        }

        GameState {
            ship: Ship::new(ctx),
            asteroids,
            projectiles: Vec::new(),
            input_set: HashSet::new(),
            last_update: Instant::now()
        }
    }
}

impl event::EventHandler<GameError> for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        if Instant::now() - self.last_update >= Duration::from_millis(MILLIS_PER_FRAME) {
            for key in &self.input_set {
                match key {
                    KeyCode::Up => {
                        self.ship.position.x += self.ship.forward.x * self.ship.speed;
                        self.ship.position.y += self.ship.forward.y * self.ship.speed;
                        self.ship.clamp();
                    }
                    KeyCode::Down => {

                    }
                    KeyCode::Left => {
                        self.ship.rotation -= 5.0_f32.to_radians();
                        self.ship.forward.x = self.ship.rotation.cos();
                        self.ship.forward.y = self.ship.rotation.sin();
                    }
                    KeyCode::Right => {
                        self.ship.rotation += 5.0_f32.to_radians();
                        self.ship.forward.x = self.ship.rotation.cos();
                        self.ship.forward.y = self.ship.rotation.sin();
                    }
                    _ => ()
                }
            }
            // Projectile updates.
            for i in 0..self.projectiles.len() {
                let projectile: &mut Projectile = self.projectiles.get_mut(i).unwrap();

                projectile.move_forward(ctx).unwrap();
                projectile.set_out_of_bounds().unwrap();
            }

            // Asteroid updates
            for i in 0..self.asteroids.len() {
                let asteroid: &mut Asteroid = self.asteroids.get_mut(i).unwrap();

                asteroid.move_forward(ctx).unwrap();
            }

            // Handle projectile collision with asteroids.
            let mut new_asteroids: Vec<Asteroid> = Vec::new();
            for i in 0..self.projectiles.len() {
                let mut projectile: &mut Projectile = self.projectiles.get_mut(i).unwrap();

                for j in 0..self.asteroids.len() {
                    let asteroid: &mut Asteroid = self.asteroids.get_mut(j).unwrap();

                    if projectile_hit(projectile, asteroid) {
                        projectile.to_remove = true;
                        // Asteroid breaks into smaller pieces.
                        new_asteroids.append(&mut asteroid.destroy_asteroid(ctx));
                    }
                }
            }
            self.projectiles.retain(|p| !p.to_remove);
            self.asteroids.retain(|a| !a.destroyed);
            self.asteroids.append(&mut new_asteroids);

            self.last_update = Instant::now();
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas: Canvas = Canvas::from_frame(ctx, Color::BLACK);

        self.ship.draw(ctx, &mut canvas).unwrap();

        for projectile in &mut self.projectiles {
            projectile.draw(&mut canvas).unwrap();
        }

        for asteroid in &mut self.asteroids {
            asteroid.draw(&mut canvas).unwrap();
        }

        canvas.finish(ctx).unwrap();
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

fn projectile_hit(projectile: &mut Projectile, asteroid: &mut Asteroid) -> bool {
    let asteroid_x: f32 = asteroid.position.x;
    let asteroid_y: f32 = asteroid.position.y;
    let asteroid_radius: f32 = asteroid.radius;
    let projectile_x: f32 = projectile.position.x;
    let projectile_y: f32 = projectile.position.y;

    let asteroid_x_range: (f32, f32) = (asteroid_x - asteroid_radius, asteroid_x + asteroid_radius);
    let asteroid_y_range: (f32, f32) = (asteroid_y - asteroid_radius, asteroid_y + asteroid_radius);

    let x_overlap: bool = projectile_x >= asteroid_x_range.0 && projectile_x <= asteroid_x_range.1;
    let y_overlap: bool = projectile_y >= asteroid_y_range.0 && projectile_y <= asteroid_y_range.1;

    return x_overlap && y_overlap;
}

fn main() {
    let (ctx, event_loop) = ContextBuilder::new(GAME_ID, AUTHOR)
        .window_setup(WindowSetup::default().title(GAME_ID))
        .window_mode(WindowMode::default().dimensions(SCREEN_SIZE.x, SCREEN_SIZE.y))
        .build()
        .unwrap();

    let game_state: GameState = GameState::new(&ctx);

    event::run(ctx, event_loop, game_state);
}

