use std::error::Error;
use ggez::conf::{WindowMode, WindowSetup};
use ggez::{ContextBuilder, event};
use crate::constants::SCREEN_SIZE;
use crate::game::Game;

mod ship;
mod projectile;
mod asteroid;
mod constants;
mod collision;
mod particle;
mod sounds;
mod score;
mod alien;
mod game;
mod save;


const GAME_ID: &str = "Asteroids";
const AUTHOR: &str = "BPoisson";

fn main() -> Result<(), Box<dyn Error>> {
    let (ctx, event_loop) = ContextBuilder::new(GAME_ID, AUTHOR)
        .window_setup(WindowSetup::default().title(GAME_ID))
        .window_mode(WindowMode::default().dimensions(SCREEN_SIZE.x, SCREEN_SIZE.y))
        .add_resource_path("resources")
        .build()?;

    let game: Game = Game::new(&ctx);

    event::run(ctx, event_loop, game);
}

