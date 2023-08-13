use std::sync::{Arc, mpsc};
use macroquad::prelude::*;
use crate::{draw_text_center_align, player_to_color};
use crate::game::object::{Coord, Depth, Object};
use crate::sound::SoundType;
use crate::texture::TextureStore;

pub struct Scoreboard {
    scores: Vec<i32>,
    multiplications: Vec<(i32, u32, u32)>,
}

impl Scoreboard {
    pub fn new(client_count: u32) -> Self {
        Self {
            scores: vec![0; client_count as usize],
            multiplications: vec![]
        }
    }

    pub fn update(&mut self, client: u32, diff: i32, time: u32) -> i32 {
        let mut actual_diff = diff;
        for (by, target, until) in &self.multiplications {
            if client == *target && time < *until {
                actual_diff *= *by;
                break;
            }
        }
        self.scores[client as usize] += actual_diff;
        if self.scores[client as usize] < 0 {
            self.scores[client as usize] = 0;
        }
        actual_diff
    }

    pub fn add_multiplication(&mut self, by: i32, client: u32, until: u32) {
        self.multiplications.push((by, client, until));
    }

    pub fn scores(&self) -> Vec<i32> {
        self.scores.clone()
    }
}

/*
    State saves the rendering portion itself
 */
#[derive(Clone)]
pub struct ScoreboardObject {
    birth_time: u32,
    start_state: Vec<f32>,
    target_state: Vec<f32>,
    animation_duration: u32,
    scores: Vec<i32>,
}

impl ScoreboardObject {
    pub fn new(time: u32, window_size: (f32, f32), client_count: u32) -> Self {
        let portion = window_size.0 / client_count as f32;
        Self {
            birth_time: time,
            start_state: vec![portion; client_count as usize],
            target_state: vec![portion; client_count as usize],
            animation_duration: 0,
            scores: vec![0; client_count as usize]
        }
    }

    pub fn from(scoreboard: &Scoreboard, previous: &ScoreboardObject, time: u32, animation_duration: u32, window_size: (f32, f32)) -> Self {
        let scores = scoreboard.scores();
        let mut sum = 0;
        for i in &scores {
            sum += i;
        }

        Self {
            birth_time: time,
            start_state: previous.current_state(time).unwrap(),
            target_state: scores.iter().map(|x| if sum == 0 {1. / scores.len() as f32} else {*x as f32 / sum as f32} * window_size.0).collect(),
            animation_duration,
            scores,
        }
    }

    pub fn current_state(&self, time: u32) -> Option<Vec<f32>> {
        if time < self.birth_time {
            return None
        }

        if time > self.birth_time + self.animation_duration {
            return Some(self.target_state.clone())
        }

        let ratio = (time - self.birth_time) as f32 / self.animation_duration as f32;
        let iter = self.start_state.iter().zip(self.target_state.iter());
        Some(iter.map(|(x, y)| x + (y-x) * ratio).collect())
    }
}

impl Object for ScoreboardObject {
    fn draw(&self, _center: Coord, age: u32, window_size: (f32, f32), _texture_store: Arc<TextureStore>) {
        let mut sum = 0.;
        for (i, val) in self.current_state(age).unwrap().iter().enumerate() {
            let color = player_to_color(i);
            draw_rectangle(sum, 0., *val, window_size.1 / 24.0, color);
            draw_text_center_align(self.scores[i].to_string().as_str(), sum + *val / 2., window_size.1 / 48.0, window_size.1 / 18.0, WHITE);
            sum += *val;
        }
    }

    fn pos(&self, _age: u32, _window_size: (f32, f32)) -> Coord {
        (100.0, 100.0)
    }

    fn depth(&self) -> Depth { Depth::Foreground(0) }

    fn max_age(&self) -> Option<u32> {
        None
    }

    fn born_time(&self) -> u32 {
        0
    }

    fn shoot_check(&self, _coord: Coord, _time: u32, _window_size: (f32, f32)) -> Option<Coord> {
        None
    }

    fn shoot(&mut self, _coord: Coord, _time: u32, _client: u32, _scoreboard: &mut Scoreboard, _sound_tx: &mut mpsc::Sender<SoundType>) {}

    fn can_be_cleaned(&self, _time: u32) -> bool {
        false
    }
}