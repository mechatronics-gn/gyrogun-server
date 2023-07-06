use std::sync::Arc;
use std::sync::mpsc::Sender;
use macroquad::color::Color;
use crate::client::Message;
use crate::game::balloon_game::BalloonGame;
use crate::game::Game;
use crate::game::object::{Object, ObjectWrapper};
use crate::game::object::balloon::{Balloon, BalloonColor};
use crate::game::object::game_result::GameResult;
use crate::sound::SoundType;

pub struct BalloonResults {
    window_size: (f32, f32),
    scores: Vec<i32>,
    objects: Vec<Arc<Box<dyn Object + Send + Sync>>>,
    objects_was_updated: bool,
}

impl BalloonResults {
    pub fn from(window_size: (f32, f32), game: &BalloonGame) -> Self {
        Self {
            window_size,
            scores: game.scores(),
            objects: vec![Arc::new(Box::new(GameResult::new(game.scores())))],
            objects_was_updated: true,
        }
    }
}

impl Game for BalloonResults {
    fn on_time(&mut self, time: u32) {
        if time % 20 == 0 && time < 300 {
            let mut x: Vec<(usize, &i32)> = self.scores.iter().enumerate().collect();
            x.sort_by(|(_, a), (_, b)| { (**b).partial_cmp(*a).unwrap() });
            let mut winners = vec![];
            let mut max = -1;
            for (i, val) in x {
                if *val >= max {
                    max = *val;
                    winners.push(i);
                } else {
                    break;
                }
            }
            let color = |x| { match x {
                0 => { BalloonColor::Red },
                1 => { BalloonColor::Green },
                _ => { BalloonColor::Blue }
            }};
            let balloon = Balloon::new(
                rand::random::<f32>() * self.window_size.0 * 0.25 + self.window_size.0 * 0.6,
                self.window_size.0 / 32.0 * (rand::random::<f32>() * 0.2 + 1.0),
                time,
                color(winners[rand::random::<usize>() % winners.len()]),
                240,
                0
            );
            let balloon: Arc<Box<dyn Object + Send+ Sync>> = Arc::new(Box::new(balloon));
            self.add_objects(balloon.clone());
        }
    }

    fn on_message(&mut self, _client: u32, _message: Message, _time: u32, _sound_tx: &mut Sender<SoundType>) {}

    fn objects(&mut self, _time: u32) -> Vec<ObjectWrapper> {
        self.objects.iter().map(|x| ObjectWrapper::Weak(Arc::downgrade(x))).collect()
    }

    fn add_objects(&mut self, object: Arc<Box<dyn Object + Send + Sync>>) {
        self.objects.push(object);
        self.objects_was_updated = true;
    }

    fn was_objects_updated(&mut self) -> bool {
        if self.objects_was_updated {
            self.objects_was_updated = false;
            return true;
        }
        false
    }

    fn background_color(&self, _time: u32) -> Color {
        Color::from_rgba(147, 169, 209, 0)
    }
}