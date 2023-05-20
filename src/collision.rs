use ggez::Context;
use rand::rngs::ThreadRng;
use crate::alien::{Alien, ALIEN_NEGATIVE_Y_BOUND, ALIEN_POSITIVE_Y_BOUND, ALIEN_X_BOUND};
use crate::asteroid::Asteroid;
use crate::particle::Particle;
use crate::projectile::Projectile;
use crate::score::Score;
use crate::ship::Ship;
use crate::sounds::Sounds;

pub fn handle_collisions(ctx: &Context,
                         rng: &mut ThreadRng,
                         _ship: &mut Ship,
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
                    new_particles.append(&mut Particle::create_particle_effect(rng, &alien.position, 6, 10));

                    player_projectile.expired = true;
                    alien.health -= 1;

                    if alien.health <= 0 {
                        alien.expired = true;
                        score.update_score_alien();
                        sounds.play_alien_explosion(ctx);
                    }
                    continue; // Skip this projectile as it has collided with an Alien.
                }
            }
            // Check Asteroid collisions.
            for j in 0..asteroids.len() {
                if let Some(asteroid) = asteroids.get_mut(j) {
                    if projectile_asteroid_hit(player_projectile, asteroid) {
                        // Add to particle effect
                        new_particles.append(&mut Particle::create_particle_effect(rng, &asteroid.position, 3, 5));
                        // Asteroid breaks into smaller pieces.
                        new_asteroids.append(&mut asteroid.destroy_asteroid(ctx, rng));

                        player_projectile.expired = true;

                        score.update_score_asteroid(&asteroid.size);

                        sounds.play_asteroid_break_sound(ctx, &asteroid.size);
                    }
                }
            }
        }
    }
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