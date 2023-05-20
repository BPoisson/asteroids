use ggez::audio::{SoundSource, Source};
use ggez::Context;
use crate::asteroid::{AsteroidSize};

pub struct Sounds {
    sounds: Vec<Source>
}

impl Sounds {
    pub fn new(ctx: &Context) -> Self {
        let mut sounds: Vec<Source> = Vec::new();

        sounds.push(Source::new(ctx, "\\sounds\\laser_shot.wav").unwrap());
        sounds.push(Source::new(ctx, "\\sounds\\ship_thrust.wav").unwrap());
        sounds.push(Source::new(ctx, "\\sounds\\big_explosion.wav").unwrap());
        sounds.push(Source::new(ctx, "\\sounds\\medium_explosion.wav").unwrap());
        sounds.push(Source::new(ctx, "\\sounds\\small_explosion.wav").unwrap());
        sounds.push(Source::new(ctx, "\\sounds\\alien_music.wav").unwrap());
        sounds.push(Source::new(ctx, "\\sounds\\alien_warning.wav").unwrap());
        sounds.push(Source::new(ctx, "\\sounds\\alien_shot.wav").unwrap());
        sounds.push(Source::new(ctx, "\\sounds\\alien_explosion.wav").unwrap());

        return Sounds {
            sounds
        }
    }

    pub fn play_shoot_sound(&mut self, ctx: &Context) -> () {
        if let Some(shoot_sound) = self.sounds.get_mut(0) {
            shoot_sound.play_detached(ctx).unwrap();
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
                if let Some(asteroid_big_sound) = self.sounds.get_mut(2) {
                    asteroid_big_sound.play_detached(ctx).unwrap();
                }
            },
            AsteroidSize::MEDIUM => {
                if let Some(asteroid_medium_sound) = self.sounds.get_mut(3) {
                    asteroid_medium_sound.play_detached(ctx).unwrap();
                }
            },
            _ => {
                if let Some(asteroid_small_sound) = self.sounds.get_mut(4) {
                    asteroid_small_sound.play_detached(ctx).unwrap();
                }
            }
        }
    }

    pub fn play_alien_music(&mut self, ctx: &Context) -> () {
        if let Some(alien_music) = self.sounds.get_mut(5) {
            alien_music.play(ctx).unwrap();
            alien_music.set_repeat(true);
        }
    }

    pub fn stop_alien_music(&mut self, ctx: &Context) -> () {
        if let Some(alien_music) = self.sounds.get_mut(5) {
            if alien_music.playing() {
                alien_music.stop(ctx).unwrap();
            }
        }
    }

    pub fn play_alien_warning(&mut self, ctx: &Context) -> () {
        if let Some(alien_warning) = self.sounds.get_mut(6) {
            alien_warning.play(ctx).unwrap();
        }
    }

    pub fn stop_alien_warning(&mut self, ctx: &Context) -> () {
        if let Some(alien_warning) = self.sounds.get_mut(6) {
            if alien_warning.playing() {
                alien_warning.stop(ctx).unwrap();
            }
        }
    }

    pub fn play_alien_shot(&mut self, ctx: &Context) -> () {
        if let Some(alien_shot) = self.sounds.get_mut(7) {
            alien_shot.play_detached(ctx).unwrap();
        }
    }

    pub fn play_alien_explosion(&mut self, ctx: &Context) -> () {
        if let Some(alien_explosion) = self.sounds.get_mut(8) {
            alien_explosion.play_detached(ctx).unwrap();
        }
    }


}

