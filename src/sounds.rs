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

        return Sounds {
            sounds
        }
    }

    pub fn play_shoot_sound(&mut self, ctx: &Context) -> () {
        if let Some(sound) = self.sounds.get_mut(0) {
            sound.play_detached(ctx).unwrap();
        }
    }

    pub fn play_thrust_sound(&mut self, ctx: &Context) -> () {
        if let Some(sound) = self.sounds.get_mut(1) {
            if !sound.playing() {
                sound.play(ctx).unwrap();
                sound.set_repeat(true);
            }
        }
    }

    pub fn stop_thrust_sound(&mut self, ctx: &Context) -> () {
        if let Some(sound) = self.sounds.get_mut(1) {
            if sound.playing() {
                sound.stop(ctx).unwrap();
            }
        }
    }

    pub fn play_asteroid_break_sound(&mut self, ctx: &Context, size: &AsteroidSize) -> () {
        match size {
            AsteroidSize::BIG => {
                if let Some(sound) = self.sounds.get_mut(2) {
                    sound.play_detached(ctx).unwrap();
                }
            },
            AsteroidSize::MEDIUM => {
                if let Some(sound) = self.sounds.get_mut(3) {
                    sound.play_detached(ctx).unwrap();
                }
            },
            _ => {
                if let Some(sound) = self.sounds.get_mut(4) {
                    sound.play_detached(ctx).unwrap();
                }
            }
        }
    }
}

