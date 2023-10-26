use std::sync::Arc;
use std::sync::mpsc::Sender;
use macroquad::prelude::*;
use crate::game::object::{Coord, Depth, Object};
use crate::game::object::scoreboard::Scoreboard;
use crate::sound::SoundType;
use crate::texture::TextureStore;

pub struct FullScreenImage {
    image_idx: i32,
    depth: i32,
}

impl FullScreenImage {
    pub fn new(image_idx: i32, depth: i32) -> Self {
        Self {
            image_idx,
            depth
        }
    }
}

impl Object for FullScreenImage {
    fn draw(&self, _center: Coord, _age: u32, window_size: (f32, f32), texture_store: Arc<TextureStore>) {
        draw_texture_ex(&texture_store.full_screen_image(self.image_idx), 0., 0., WHITE, DrawTextureParams {
            dest_size: Some(Vec2 { x: window_size.0, y: window_size.1 }),
            source: None, rotation: 0.0, flip_x: false, flip_y: false, pivot: None,
        })
    }

    fn pos(&self, _age: u32, _window_size: (f32, f32)) -> Coord {
        (0., 0.)
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

    fn shoot_check(&self, _coord: Coord, _time: u32, _window_size: (f32, f32)) -> Option<Coord> {
        None
    }

    fn shoot(&mut self, _coord: Coord, _time: u32, _client: u32, _scoreboard: &mut Scoreboard, _sound_tx: &mut Sender<SoundType>) {
    }

    fn can_be_cleaned(&self, _time: u32) -> bool {
        false
    }
}