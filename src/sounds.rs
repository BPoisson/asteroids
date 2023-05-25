use ggez::audio::{SoundSource, Source};
use ggez::Context;
use crate::asteroid::{AsteroidSize};

pub struct Sounds {
    sounds: Vec<Source>
}

impl Sounds {
    pub fn new(ctx: &Context) -> Self {
        let mut sounds: Vec<Source> = Vec::new();

        sounds.push(Source::new(ctx, "\\sounds\\player_shoot.wav").unwrap());
        sounds.push(Source::new(ctx, "\\sounds\\ship_thrust.wav").unwrap());
        sounds.push(Source::new(ctx, "\\sounds\\big_explosion.wav").unwrap());
        sounds.push(Source::new(ctx, "\\sounds\\medium_explosion.wav").unwrap());
        sounds.push(Source::new(ctx, "\\sounds\\small_explosion.wav").unwrap());
        sounds.push(Source::new(ctx, "\\sounds\\alien_music.wav").unwrap());
        sounds.push(Source::new(ctx, "\\sounds\\alien_warning.wav").unwrap());
        sounds.push(Source::new(ctx, "\\sounds\\alien_shoot.wav").unwrap());
        sounds.push(Source::new(ctx, "\\sounds\\alien_hit.wav").unwrap());
        sounds.push(Source::new(ctx, "\\sounds\\alien_explosion.wav").unwrap());
        sounds.push(Source::new(ctx, "\\sounds\\ship_collision.wav").unwrap());
        sounds.push(Source::new(ctx, "\\sounds\\ship_hit.wav").unwrap());

        return Sounds {
            sounds
        }
    }

    pub fn play_player_shoot_sound(&mut self, ctx: &Context) -> () {
        if let Some(player_shoot_sound) = self.sounds.get_mut(0) {
            player_shoot_sound.play_detached(ctx).unwrap();
        }
    }

    pub fn play_thrust_sound(&mut self, ctx: &Context) -> () {
        if let Some(thrust_sound) = self.sounds.get_mut(1) {
            if !thrust_sound.playing() {
                thrust_sound.play(ctx).unwrap();
                thrust_sound.set_repeat(true);
            }
        }
    }

    pub fn stop_thrust_sound(&mut self, ctx: &Context) -> () {
        if let Some(thrust_sound) = self.sounds.get_mut(1) {
            if thrust_sound.playing() {
                thrust_sound.stop(ctx).unwrap();
            }
        }
    }

    pub fn play_asteroid_break_sound(&mut self, ctx: &Context, size: &AsteroidSize) -> () {
        match size {
            AsteroidSize::BIG => {
                if let Some(big_asteroid_break_sound) = self.sounds.get_mut(2) {
                    big_asteroid_break_sound.play_detached(ctx).unwrap();
                }
            },
            AsteroidSize::MEDIUM => {
                if let Some(medium_asteroid_break_sound) = self.sounds.get_mut(3) {
                    medium_asteroid_break_sound.play_detached(ctx).unwrap();
                }
            },
            _ => {
                if let Some(small_asteroid_break_sound) = self.sounds.get_mut(4) {
                    small_asteroid_break_sound.play_detached(ctx).unwrap();
                }
            }
        }
    }

    pub fn play_alien_music(&mut self, ctx: &Context) -> () {
        if let Some(alien_music) = self.sounds.get_mut(5) {
            alien_music.play(ctx).unwrap();
        }
    }

    pub fn stop_alien_music(&mut self, ctx: &Context) -> () {
        if let Some(alien_music) = self.sounds.get_mut(5) {
            if alien_music.playing() {
                alien_music.stop(ctx).unwrap();
            }
        }
    }

    pub fn play_alien_warning_sound(&mut self, ctx: &Context) -> () {
        if let Some(alien_warning_sound) = self.sounds.get_mut(6) {
            alien_warning_sound.play(ctx).unwrap();
        }
    }

    pub fn stop_alien_warning_sound(&mut self, ctx: &Context) -> () {
        if let Some(alien_warning_sound) = self.sounds.get_mut(6) {
            if alien_warning_sound.playing() {
                alien_warning_sound.stop(ctx).unwrap();
            }
        }
    }

    pub fn play_alien_shoot_sound(&mut self, ctx: &Context) -> () {
        if let Some(alien_shoot_sound) = self.sounds.get_mut(7) {
            alien_shoot_sound.play_detached(ctx).unwrap();
        }
    }

    pub fn play_alien_hit_sound(&mut self, ctx: &Context) -> () {
        if let Some(alien_hit_sound) = self.sounds.get_mut(8) {
            alien_hit_sound.play_detached(ctx).unwrap();
        }
    }

    pub fn play_alien_explosion_sound(&mut self, ctx: &Context) -> () {
        if let Some(alien_explosion_sound) = self.sounds.get_mut(9) {
            alien_explosion_sound.play_detached(ctx).unwrap();
        }
    }

    pub fn play_ship_collision_sound(&mut self, ctx: &Context) -> () {
        if let Some(ship_collision_sound) = self.sounds.get_mut(10) {
            ship_collision_sound.play_detached(ctx).unwrap();
        }
    }

    pub fn play_ship_hit_sound(&mut self, ctx: &Context) -> () {
        if let Some(ship_hit_sound) = self.sounds.get_mut(11) {
            ship_hit_sound.play_detached(ctx).unwrap();
        }
    }
}

