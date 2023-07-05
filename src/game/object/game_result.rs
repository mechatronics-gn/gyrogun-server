use std::sync::Arc;
use std::sync::mpsc::Sender;
use macroquad::prelude::*;
use crate::game::object::{Coord, Depth, Object};
use crate::game::object::scoreboard::Scoreboard;
use crate::sound::SoundType;
use crate::texture::TextureStore;

pub struct GameResult {
    scores: Vec<i32>,
}

impl GameResult {
    pub fn new(scores: Vec<i32>) -> Self {
        Self {
            scores
        }
    }
}

impl Object for GameResult {
    fn draw(&self, center: Coord, age: u32, window_size: (f32, f32), texture_store: Arc<TextureStore>) {
        draw_text("I am too lazy", center.0, center.1, 100., WHITE);
        for (i, value) in self.scores.iter().enumerate() {
            draw_text(format!("{}: {}\n", i, value).as_str(), center.0, center.1 + (i + 1) as f32 * 100., 100., WHITE);
        }
    }

    fn pos(&self, age: u32, window_size: (f32, f32)) -> Coord {
        let (w, h) = window_size;
        (w / 2., h / 2.)
    }

    fn depth(&self) -> Depth {
        Depth::Foreground(0)
    }

    fn max_age(&self) -> Option<u32> {
        None
    }

    fn born_time(&self) -> u32 {
        0
    }

    fn shoot_check(&self, coord: Coord, time: u32, window_size: (f32, f32)) -> Option<Coord> {
        None
    }

    fn shoot(&mut self, coord: Coord, time: u32, client: u32, scoreboard: &mut Scoreboard, sound_tx: &mut Sender<SoundType>) {}

    fn can_be_cleaned(&self, time: u32) -> bool {
        false
    }
}