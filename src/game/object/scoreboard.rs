use std::ops::Add;
use std::sync::mpsc;
use macroquad::prelude::*;
use crate::game::object::{Coord, Depth, Object};
use crate::sound::SoundType;

pub struct Scoreboard {
    scores: Vec<i32>
}

impl Scoreboard {
    pub fn new(client_count: u32) -> Self {
        Self {
           scores: vec![0; client_count as usize]
        }
    }

    pub fn update(&mut self, client: u32, diff: i32) {
        self.scores[client as usize] += diff;
    }

    fn scores(&self) -> Vec<i32> {
        self.scores.clone()
    }
}

pub struct ScoreboardObject {
    scores: Vec<i32>
}

impl ScoreboardObject {
    pub fn from(scoreboard: &Scoreboard) -> Self {
        Self {
            scores: scoreboard.scores()
        }
    }
}

impl Object for ScoreboardObject {
    fn draw(&self, _center: Coord, age: u32, window_size: (f32, f32)) {
        let mut s = String::new();
        for (i, val) in self.scores.iter().enumerate() {
            s = s.add(format!("{}: {} points\n", i, val).as_str());
        }
        let (x, y) = self.pos(age, window_size);
        draw_text(s.as_str(), x, y, 30.0, BLACK);
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

    fn shoot(&mut self, coord: Coord, time: u32, client: u32, scoreboard: &mut Scoreboard, sound_tx: &mut mpsc::Sender<SoundType>) {
    }

    fn can_be_cleaned(&self, _time: u32) -> bool {
        false
    }
}