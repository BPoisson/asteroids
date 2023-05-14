use crate::asteroid::Asteroid;
use crate::projectile::Projectile;

pub fn projectile_hit(projectile: &mut Projectile, asteroid: &mut Asteroid) -> bool {
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