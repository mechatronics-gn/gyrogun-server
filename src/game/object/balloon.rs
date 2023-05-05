use std::sync::{Arc, mpsc};
use crate::game::object::{Coord, Depth};
use macroquad::prelude::*;
use crate::game::object::scoreboard::Scoreboard;
use crate::sound::SoundType;
use crate::texture::TextureStore;
use super::Object;

pub struct Balloon {
    start_x: f32,
    radius: f32,
    color: BalloonColor,
    lifetime: u32,
    shoot_points: i32,
    born_time: u32,
    shot_data: Option<(u32, Coord)>,
}

pub enum BalloonColor {
    Blue, Green, Orange, Pink, Purple, Red, Yellow
}

impl Balloon {
    pub fn new(start_x: f32, radius: f32, born_time: u32, color: BalloonColor, lifetime: u32, shoot_points: i32) -> Self {
        Self {
            start_x,
            radius,
            color,
            lifetime,
            shoot_points,
            born_time,
            shot_data: None,
        }
    }
}

impl Object for Balloon {
    fn draw(&self, center: Coord, age: u32, window_size: (f32, f32), texture_store: Arc<TextureStore>) {
        if let Some((shot_time, (x, y))) = self.shot_data {
            let shot_age = shot_time - self.born_time;
            if shot_age + 50 > age && age > shot_age {
                let variant = (age - shot_age) / 10 + 2;
                let texture = texture_store.balloon(&self.color, variant as i32);
                draw_texture_ex(texture, x - window_size.0 * 0.108 / 2.0, y - window_size.1 * 0.086 , WHITE, DrawTextureParams {
                    dest_size: Some(Vec2 { x: window_size.0 * 0.108, y: window_size.1 * 0.193 }),
                    source: None, rotation: 0.0, flip_x: false, flip_y: false, pivot: None,
                });
            }
            return;
        }
        let (x, y) = center;
        draw_circle(x, y, self.radius, RED);
        let texture = texture_store.balloon(&self.color, 1);
        draw_texture_ex(texture, x - window_size.0 * 0.108 / 2.0, y - window_size.1 * 0.086 , WHITE, DrawTextureParams {
            dest_size: Some(Vec2 { x: window_size.0 * 0.108, y: window_size.1 * 0.193 }),
            source: None, rotation: 0.0, flip_x: false, flip_y: false, pivot: None,
        });

        let string = texture_store.balloon_string(2);
        draw_texture_ex(string, x - window_size.0 * 0.003 / 2.0, y + window_size.1 * 0.088, WHITE, DrawTextureParams {
            dest_size: Some(Vec2 { x: window_size.0 * 0.003, y: window_size.1 * 0.083}),
            source: None, rotation: 0.0, flip_x: false, flip_y: false,  pivot: None,
        } )
    }

    fn pos(&self, age: u32, window_size: (f32, f32)) -> Coord {
        (self.start_x, window_size.1 - (age * 1080 / (self.lifetime-60)) as f32 + self.radius)
    }

    fn depth(&self) -> Depth {
        Depth::Main(self.shoot_points)
    }

    fn max_age(&self) -> Option<u32> {
        Some(self.lifetime)
    }

    fn born_time(&self) -> u32 {
        self.born_time
    }

    fn shoot_check(&self, coord: Coord, time: u32, window_size: (f32, f32)) -> Option<Coord> {
        let (x1, y1) = self.pos(time - self.born_time(), window_size);
        let (x2, y2) = coord;

        if (x2 - x1) * (x2 - x1) + (y2 - y1) * (y2 - y1) < self.radius * self.radius {
            Some((x1, y1))
        } else {
            None
        }
    }

    fn shoot(&mut self, coord: Coord, time: u32, client: u32, scoreboard: &mut Scoreboard, sound_tx: &mut mpsc::Sender<SoundType>) {
        if let None = self.shot_data {
            self.shot_data = Some((time, coord));
            scoreboard.update(client, self.shoot_points);
            sound_tx.send(SoundType::BalloonExplosion).ok();
        }
    }

    fn can_be_cleaned(&self, time: u32) -> bool {
        if let Some((shot_time, _)) = self.shot_data {
            if time > shot_time + 100 {
                return true;
            }
        } else if self.born_time + self.lifetime < time {
            return true;
        }
        false
    }
}