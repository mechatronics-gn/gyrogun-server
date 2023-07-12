use std::sync::Arc;
use std::sync::mpsc::Sender;
use macroquad::prelude::*;
use crate::game::object::{Coord, Depth, Object};
use crate::game::object::scoreboard::Scoreboard;
use crate::sound::SoundType;
use crate::texture::TextureStore;

pub struct CorrectionCircle {
    is_left: bool,
    depth: i32,
}

impl CorrectionCircle {
    pub fn new(is_left: bool, depth: i32) -> Self {
        Self {
            is_left,
            depth,
        }
    }
}

impl Object for CorrectionCircle {
    fn draw(&self, center: Coord, age: u32, window_size: (f32, f32), texture_store: Arc<TextureStore>) {
        draw_circle(center.0, center.1, 20.0, GREEN);
    }

    fn pos(&self, age: u32, window_size: (f32, f32)) -> Coord {
        let (w, h) = window_size;
        if self.is_left {
            (w / 2. -  h / 2., h / 2.)
        } else {
            (w / 2. + h / 2., h / 2.)
        }
    }

    fn depth(&self) -> Depth {
        Depth::Main(self.depth)
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

    fn shoot(&mut self, coord: Coord, time: u32, client: u32, scoreboard: &mut Scoreboard, sound_tx: &mut Sender<SoundType>) {
    }

    fn can_be_cleaned(&self, time: u32) -> bool {
        false
    }
}