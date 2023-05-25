use std::time::Instant;
use ggez::Context;
use ggez::graphics::Color;
use rand::rngs::ThreadRng;
use crate::alien::{Alien, ALIEN_NEGATIVE_Y_BOUND, ALIEN_POSITIVE_Y_BOUND, ALIEN_X_BOUND};
use crate::asteroid::Asteroid;
use crate::particle::Particle;
use crate::projectile::Projectile;
use crate::score::Score;
use crate::ship::Ship;
use crate::sounds::Sounds;

pub fn handle_player_projectile_collisions(ctx: &Context,
                                           rng: &mut ThreadRng,
                                           alien: &mut Option<Alien>,
                                           player_projectiles: &mut Vec<Projectile>,
                                           asteroids: &mut Vec<Asteroid>,
                                           score: &mut Score,
                                           sounds: &mut Sounds) -> (Vec<Asteroid>, Vec<Particle>) {
    let mut new_asteroids: Vec<Asteroid> = Vec::new();
    let mut new_particles: Vec<Particle> = Vec::new();

    for i in 0..player_projectiles.len() {
        if let Some(player_projectile) = player_projectiles.get_mut(i) {
            // Check Alien collisions.
            if let Some(alien) = alien {
                // Destroy alien and projectile when hit.
                if projectile_alien_hit(player_projectile, alien) {
                    new_particles.append(&mut handle_projectile_alien_hit(ctx, rng, player_projectile, alien, score, sounds));

                    continue; // Stop processing collisions for this projectile since it has collided with an Alien.
                }
            }
            // Check Asteroid collisions.
            for j in 0..asteroids.len() {
                if let Some(asteroid) = asteroids.get_mut(j) {
                    if projectile_asteroid_hit(player_projectile, asteroid) {
                        let mut new_asteroids_and_particles: (Vec<Asteroid>, Vec<Particle>) = handle_projectile_asteroid_hit(ctx, rng, player_projectile, asteroid, score, sounds);

                        new_asteroids.append(&mut new_asteroids_and_particles.0);
                        new_particles.append(&mut new_asteroids_and_particles.1);
                    }
                }
            }
        }
    }
    return (new_asteroids, new_particles);
}

pub fn handle_ship_asteroid_collisions(ctx: &Context,
                                       rng: &mut ThreadRng,
                                       ship: &mut Ship,
                                       asteroids: &Vec<Asteroid>,
                                       sounds: &mut Sounds) -> Option<Vec<Particle>> {
    if ship.immune {
        return None;
    }

    let mut new_particles: Vec<Particle> = Vec::new();

    for asteroid in asteroids {
        if ship_asteroid_collision(ship, asteroid) {
            new_particles = handle_ship_collision(ctx, rng, ship, sounds);
        }
    }
    return Some(new_particles);
}

pub fn handle_ship_alien_collisions(ctx: &Context,
                                       rng: &mut ThreadRng,
                                       ship: &mut Ship,
                                       alien: &mut Option<Alien>,
                                       score: &mut Score,
                                       sounds: &mut Sounds) -> Option<Vec<Particle>> {
    if ship.immune{
        return None;
    }

    if let Some(alien) = alien {
        let mut new_particles: Vec<Particle> = Vec::new();

        if ship_alien_collision(ship, alien) {
            alien.health -= 1;

            if alien.health <= 0 {
                alien.expired = true;
                score.update_score_alien();
                sounds.play_alien_explosion_sound(ctx);
            } else {
                sounds.play_alien_hit_sound(ctx);
            }
            new_particles = handle_ship_collision(ctx, rng, ship, sounds);
        }
        return Some(new_particles)
    } else {
        return None
    }
}

pub fn handle_alien_projectile_collisions(ctx: &Context,
                                          rng: &mut ThreadRng,
                                          ship: &mut Ship,
                                          alien_projectiles: &mut Vec<Projectile>,
                                          asteroids: &mut Vec<Asteroid>,
                                          score: &mut Score,
                                          sounds: &mut Sounds) -> (Vec<Asteroid>, Vec<Particle>) {
    let mut new_asteroids: Vec<Asteroid> = Vec::new();
    let mut new_particles: Vec<Particle> = Vec::new();

    for i in 0..alien_projectiles.len() {
        if let Some(alien_projectile) = alien_projectiles.get_mut(i) {
            // Check Player Ship collisions.
            if !ship.immune && alien_projectile_ship_hit(alien_projectile, ship) {
                new_particles.append(&mut handle_alien_projectile_ship_hit(ctx, rng, alien_projectile, ship, sounds));

                continue; // Stop processing collisions for this projectile since it has collided with the Player Ship.
            }
        
            // Check Asteroid collisions.
            for j in 0..asteroids.len() {
                if let Some(asteroid) = asteroids.get_mut(j) {
                    if projectile_asteroid_hit(alien_projectile, asteroid) {
                        let mut new_asteroids_and_particles: (Vec<Asteroid>, Vec<Particle>) = handle_projectile_asteroid_hit(ctx, rng, alien_projectile, asteroid, score, sounds);

                        new_asteroids.append(&mut new_asteroids_and_particles.0);
                        new_particles.append(&mut new_asteroids_and_particles.1);
                    }
                }
            }
        }
    }
    return (new_asteroids, new_particles);
}

fn handle_ship_collision(ctx: &Context,
                                  rng: &mut ThreadRng,
                                  ship: &mut Ship,
                                  sounds: &mut Sounds) -> Vec<Particle> {
    ship.health -= 1;
    ship.immune = true;
    ship.immune_instant = Instant::now();
    sounds.play_ship_collision_sound(ctx);

    return Particle::create_particle_effect(
        rng,
        &ship.position,
        5,
        8,
        Color::WHITE
    )
}

fn handle_alien_projectile_ship_hit(ctx: &Context,
                              rng: &mut ThreadRng,
                              projectile: &mut Projectile,
                              ship: &mut Ship,
                              sounds: &mut Sounds) -> Vec<Particle> {
    projectile.expired = true;
    ship.health -= 1;
    ship.immune = true;
    ship.immune_instant = Instant::now();

    sounds.play_ship_hit_sound(ctx);

    return Particle::create_particle_effect(
        rng,
        &ship.position,
        5,
        8,
    Color::GREEN);
}

fn handle_projectile_alien_hit(ctx: &Context,
                               rng: &mut ThreadRng,
                               projectile: &mut Projectile,
                               alien: &mut Alien,
                               score: &mut Score,
                               sounds: &mut Sounds) -> Vec<Particle> {
    projectile.expired = true;
    alien.health -= 1;

    if alien.health <= 0 {
        alien.expired = true;
        score.update_score_alien();
        sounds.play_alien_explosion_sound(ctx);
    } else {
        sounds.play_alien_hit_sound(ctx);
    }

    return Particle::create_particle_effect(
        rng,
        &alien.position,
        5,
        8,
        Color::WHITE);
}

fn handle_projectile_asteroid_hit(ctx: &Context,
                                  rng: &mut ThreadRng,
                                  projectile: &mut Projectile,
                                  asteroid: &mut Asteroid,
                                  score: &mut Score,
                                  sounds: &mut Sounds) -> (Vec<Asteroid>, Vec<Particle>) {
    let new_particles: Vec<Particle> = Particle::create_particle_effect(rng, &asteroid.position, 3, 5, Color::WHITE);
    let new_asteroids: Vec<Asteroid> = asteroid.destroy_asteroid(ctx, rng);

    projectile.expired = true;

    score.update_score_asteroid(&asteroid.size);

    sounds.play_asteroid_break_sound(ctx, &asteroid.size);

    return (new_asteroids, new_particles);
}

pub fn projectile_asteroid_hit(projectile: &mut Projectile, asteroid: &mut Asteroid) -> bool {
    let asteroid_x: f32 = asteroid.position.x;
    let asteroid_y: f32 = asteroid.position.y;
    let asteroid_radius: f32 = asteroid.radius;
    let projectile_x: f32 = projectile.position.x;
    let projectile_y: f32 = projectile.position.y;

    let asteroid_x_range: (f32, f32) = (asteroid_x - asteroid_radius, asteroid_x + asteroid_radius);
    let asteroid_y_range: (f32, f32) = (asteroid_y - asteroid_radius, asteroid_y + asteroid_radius);

    let x_overlap: bool = projectile_x > asteroid_x_range.0 && projectile_x < asteroid_x_range.1;
    let y_overlap: bool = projectile_y > asteroid_y_range.0 && projectile_y < asteroid_y_range.1;

    return x_overlap && y_overlap;
}

pub fn projectile_alien_hit(projectile: &mut Projectile, alien: &mut Alien) -> bool {
    let alien_x: f32 = alien.position.x;
    let alien_y: f32 = alien.position.y;
    let projectile_x: f32 = projectile.position.x;
    let projectile_y: f32 = projectile.position.y;

    let alien_x_range: (f32, f32) = (alien_x - ALIEN_X_BOUND, alien_x + ALIEN_X_BOUND);
    let alien_y_range: (f32, f32) = (alien_y - ALIEN_NEGATIVE_Y_BOUND, alien_y + ALIEN_POSITIVE_Y_BOUND);

    let x_overlap: bool = projectile_x > alien_x_range.0 && projectile_x < alien_x_range.1;
    let y_overlap: bool = projectile_y > alien_y_range.0 && projectile_y < alien_y_range.1;

    return x_overlap && y_overlap;
}

pub fn alien_projectile_ship_hit(projectile: &mut Projectile, ship: &mut Ship) -> bool {
    let projectile_x: f32 = projectile.position.x;
    let projectile_y: f32 = projectile.position.y;

    let x_overlap: bool = projectile_x > ship.collision_rect_ranges[0][0] && projectile_x < ship.collision_rect_ranges[1][0];
    let y_overlap: bool = projectile_y > ship.collision_rect_ranges[0][1] && projectile_y < ship.collision_rect_ranges[1][1];

    return x_overlap && y_overlap;
}

pub fn ship_asteroid_collision(ship: &Ship, asteroid: &Asteroid) -> bool {
    let asteroid_x: f32 = asteroid.position.x;
    let asteroid_y: f32 = asteroid.position.y;
    let asteroid_radius: f32 = asteroid.radius;

    let asteroid_x_range: (f32, f32) = (asteroid_x - asteroid_radius + 5.0, asteroid_x + asteroid_radius - 5.0);
    let asteroid_y_range: (f32, f32) = (asteroid_y - asteroid_radius + 5.0, asteroid_y + asteroid_radius - 5.0);

    let x_overlap: bool = (ship.collision_rect_ranges[1][0] > asteroid_x_range.0 && ship.collision_rect_ranges[1][0] < asteroid_x_range.1)
        || (ship.collision_rect_ranges[0][0] < asteroid_x_range.1 && asteroid_x_range.0 < ship.collision_rect_ranges[0][0]);

    let y_overlap: bool = (ship.collision_rect_ranges[1][1] > asteroid_y_range.0 && ship.collision_rect_ranges[1][1] < asteroid_y_range.1)
        || (ship.collision_rect_ranges[0][1] < asteroid_y_range.1 && asteroid_y_range.0 < ship.collision_rect_ranges[0][1]);

    return x_overlap && y_overlap;
}

pub fn ship_alien_collision(ship: &Ship, alien: &Alien) -> bool {
    let alien_x: f32 = alien.position.x;
    let alien_y: f32 = alien.position.y;

    let alien_x_range: (f32, f32) = (alien_x - ALIEN_X_BOUND + 5.0, alien_x + ALIEN_X_BOUND - 5.0);
    let alien_y_range: (f32, f32) = (alien_y - ALIEN_NEGATIVE_Y_BOUND + 5.0, alien_y + ALIEN_POSITIVE_Y_BOUND - 5.0);

    let x_overlap: bool = (ship.collision_rect_ranges[1][0] > alien_x_range.0 && ship.collision_rect_ranges[1][0] < alien_x_range.1)
        || (ship.collision_rect_ranges[0][0] < alien_x_range.1 && alien_x_range.0 < ship.collision_rect_ranges[0][0]);

    let y_overlap: bool = (ship.collision_rect_ranges[1][1] > alien_y_range.0 && ship.collision_rect_ranges[1][1] < alien_y_range.1)
        || (ship.collision_rect_ranges[0][1] < alien_y_range.1 && alien_y_range.0 < ship.collision_rect_ranges[0][1]);

    return x_overlap && y_overlap;
}