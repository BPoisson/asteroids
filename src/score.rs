use ggez::{graphics};
use ggez::graphics::{Canvas, PxScale, Text};
use crate::asteroid::AsteroidSize;

const SCORE_SCALE: f32 = 30.0;

pub struct Score {
    score: u64,
    text: Text
}

impl Score {
    pub fn new() -> Self {
        let mut text: Text = Text::new("0");
        text.set_scale(PxScale::from(SCORE_SCALE));

        return Score {
            score: 0,
            text
        };
    }

    pub fn draw(&mut self, canvas: &mut Canvas) -> () {
        canvas.draw(
            &self.text,
            graphics::DrawParam::default()
        );
    }

    pub fn update_score(&mut self, destroyed_asteroid_size: AsteroidSize) -> () {
        match destroyed_asteroid_size {
            AsteroidSize::BIG => self.score = self.score + 250,
            AsteroidSize::MEDIUM => self.score = self.score + 100,
            _ => self.score = self.score + 25,
        }

        let mut text: Text = Text::new(self.score.to_string());
        text.set_scale(PxScale::from(SCORE_SCALE));
        self.text = text;
    }
}