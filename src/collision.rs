use crate::alien::{Alien, ALIEN_NEGATIVE_Y_BOUND, ALIEN_POSITIVE_Y_BOUND, ALIEN_X_BOUND};
use crate::asteroid::Asteroid;
use crate::projectile::Projectile;

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