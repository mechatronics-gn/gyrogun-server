use std::sync::Arc;
use std::sync::mpsc::Sender;
use macroquad::prelude::*;
use crate::{draw_text_center_align, player_to_color};
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
    fn draw(&self, center: Coord, _age: u32, window_size: (f32, f32), _texture_store: Arc<TextureStore>) {
        let (w, h) = window_size;
        draw_text_center_align("Good game!", center.0 + w * 0.2 + 5., center.1 + 5., h * 0.1, BLACK);
        draw_text_center_align("Good game!", center.0 + w * 0.2, center.1, h * 0.1, WHITE);
        let mut x: Vec<(usize, &i32)> = self.scores.iter().enumerate().collect();
        x.sort_by(|(_, a), (_, b)| { (**b).partial_cmp(*a).unwrap() });

        let mut cnt = 1;
        let max_score = *x[0].1 as f32;
        for (i, value) in x {
            draw_rectangle(center.0 + 5., center.1 + cnt as f32 * h * 0.12 + 5., w * 0.4 * (*value) as f32 / max_score, h * 0.08, BLACK);
            draw_rectangle(center.0, center.1 + cnt as f32 * h * 0.12, w * 0.4 * (*value) as f32 / max_score, h * 0.08, player_to_color(i));

            draw_text_center_align(format!("{}", value).as_str(), center.0 + w * 0.02, center.1 + cnt as f32 * h * 0.12 + h * 0.04, h * 0.1, WHITE);
            cnt += 1;
        }
    }

    fn pos(&self, _age: u32, window_size: (f32, f32)) -> Coord {
        let (w, h) = window_size;
        (w * 0.08, h * 0.25)
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

    fn shoot_check(&self, _coord: Coord, _time: u32, _window_size: (f32, f32)) -> Option<Coord> {
        None
    }

    fn shoot(&mut self, _coord: Coord, _time: u32, _client: u32, _scoreboard: &mut Scoreboard, _sound_tx: &mut Sender<SoundType>) {}

    fn can_be_cleaned(&self, _time: u32) -> bool {
        false
    }
}