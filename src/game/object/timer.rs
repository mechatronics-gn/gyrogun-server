use std::sync::Arc;
use std::sync::mpsc::Sender;
use macroquad::prelude::*;
use crate::draw_text_center_align;
use crate::game::object::{Coord, Depth, Object};
use crate::game::object::scoreboard::Scoreboard;
use crate::sound::SoundType;
use crate::texture::TextureStore;

pub struct Timer {
    end_at: u32,
}

impl Timer {
    pub fn new(game_duration: u32) -> Self {
        Self {
            end_at: game_duration
        }
    }
}

impl Object for Timer {
    fn draw(&self, _center: Coord, age: u32, window_size: (f32, f32), _texture_store: Arc<TextureStore>) {
        let (w, h) = window_size;
        let timer = self.end_at - age;
        let color = if timer < 1000 && timer % 100 >= 50 {
            RED
        } else if 1000 <= timer &&  timer < 3000 && timer % 200 >= 100 {
            RED
        } else {
            WHITE
        };
        draw_text_center_align((timer / 100).to_string().as_str(), w / 2. + 5., h * 0.925 + 5., h * 0.15, BLACK);
        draw_text_center_align((timer / 100).to_string().as_str(), w / 2., h * 0.925, h * 0.15, color);
    }

    fn pos(&self, _age: u32, window_size: (f32, f32)) -> Coord {
        let (w, h) = window_size;
        (w / 2., h * 0.9)
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