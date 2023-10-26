use std::sync::{Arc, mpsc};
use macroquad::prelude::*;
use crate::game::object::{Coord, Depth, Object};
use crate::game::object::scoreboard::Scoreboard;
use crate::sound::SoundType;
use crate::texture::TextureStore;

pub struct Cloud {
    y: f32,
    height: f32,
    lifetime: u32,
    born_time: u32,
    variant: i32,
}

impl Cloud {
    pub fn new(y: f32, height: f32, lifetime: u32, born_time: u32) -> Self {
        Self {
            y, height, lifetime, born_time, variant: ((rand::rand() % 3) as i32 + 1)
        }
    }

    fn width(&self) -> f32 {
        self.height * 1.5
    }
}

impl Object for Cloud {
    fn draw(&self, center: Coord, _age: u32, _window_size: (f32, f32), texture_store: Arc<TextureStore>) {
        let (x, y) = center;
        draw_texture_ex(&texture_store.cloud(self.variant), x-self.width()/2.0, y-self.height/2.0, WHITE, DrawTextureParams {
            dest_size: Some(Vec2 { x: self.width(), y: self.height }),
            source: None, rotation: 0.0, flip_x: false, flip_y: false, pivot: None,
        });
    }

    fn pos(&self, age: u32, window_size: (f32, f32)) -> Coord {
        ((window_size.0 + self.width()) * age as f32 / self.lifetime as f32 - self.width() / 2.0, self.y)
    }

    fn depth(&self) -> Depth {
        Depth::Background(self.lifetime as i32 * -1)
    }

    fn max_age(&self) -> Option<u32> {
        Some(self.lifetime)
    }

    fn born_time(&self) -> u32 {
        self.born_time
    }

    fn shoot_check(&self, _coord: Coord, _time: u32, _window_size: (f32, f32)) -> Option<Coord> {
        None
    }

    fn shoot(&mut self, _coord: Coord, _time: u32, _client: u32, _scoreboard: &mut Scoreboard, _sound_tx: &mut mpsc::Sender<SoundType>) {
    }

    fn can_be_cleaned(&self, time: u32) -> bool {
        time > self.born_time + self.lifetime
    }
}