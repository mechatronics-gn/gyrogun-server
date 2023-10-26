use std::collections::HashMap;
use std::f32::consts::PI;
use std::sync::Arc;
use std::sync::mpsc::Sender;
use macroquad::prelude::*;
use crate::game::object::{Coord, Depth, Object};
use crate::game::object::scoreboard::Scoreboard;
use crate::sound::SoundType;
use crate::texture::TextureStore;

#[derive(Clone)]
pub struct InitIndicator {
    state: HashMap<i32, bool>,
}

impl InitIndicator {
    pub fn new(state: HashMap<i32, bool>) -> Self {
        InitIndicator {
            state,
        }
    }
}

impl Object for InitIndicator {
    fn draw(&self, center: Coord, age: u32, window_size: (f32, f32), texture_store: Arc<TextureStore>) {
        let (x, y) = center;
        for (i, val) in &self.state {
            let x = x - 0.225 * window_size.0 + 0.15 * *i as f32 * window_size.0;

            let x = x - window_size.0 * 0.05;
            let y  = y - window_size.0 * 0.05;

            if *val {
                draw_texture_ex(&texture_store.checkmark(*i), x, y, WHITE, DrawTextureParams {
                    dest_size: Some(Vec2 { x: window_size.0 * 0.1, y: window_size.0 * 0.1 }),
                    source: None, rotation: 0.0, flip_x: false, flip_y: false, pivot: None,
                })
            } else {
                draw_texture_ex(&texture_store.loader(), x, y, WHITE, DrawTextureParams {
                    dest_size: Some(Vec2 { x: window_size.0 * 0.1, y: window_size.0 * 0.1}),
                    source: None, rotation: (age % 60) as f32 * PI / 30., flip_x: false, flip_y: false, pivot: None,
                })
            }
        }
    }

    fn pos(&self, _age: u32, window_size: (f32, f32)) -> Coord {
        (window_size.0 * 0.5, window_size.1 * 0.15)
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

    fn shoot(&mut self, _coord: Coord, _time: u32, _client: u32, _scoreboard: &mut Scoreboard, _sound_tx: &mut Sender<SoundType>) {
    }

    fn can_be_cleaned(&self, _time: u32) -> bool {
        false
    }
}

