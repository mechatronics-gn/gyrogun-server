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
                draw_texture_ex(texture, x - self.radius * 2.61125 / 2.0, y - self.radius * 1.17557 , WHITE, DrawTextureParams {
                    dest_size: Some(Vec2 { x: self.radius * 2.61125, y: self.radius * 2.61125 }),
                    source: None, rotation: 0.0, flip_x: false, flip_y: false, pivot: None,
                });
            }

            let (r, g, b) = if self.shoot_points > 0 { (0, 255, 0) } else { (255, 0, 0) };
            let text = if self.shoot_points > 0 { format!("+{}", self.shoot_points) } else { format!("{}", self.shoot_points) };
            let alpha = if age < shot_age + 33 {
                ((age - shot_age) * 255 / 34) as u8
            } else if age < shot_age + 66 {
                255
            } else if age < shot_age + 100 {
                255 - ((age - shot_age - 66) * 255 / 35) as u8
            } else {
                0
            };
            draw_text(text.as_str(), x, y, window_size.0 / 24.0, Color::from_rgba(r, g, b, alpha));

            return;
        }
        let (x, y) = center;
        draw_circle(x, y, self.radius, RED);
        let texture = texture_store.balloon(&self.color, 1);
        draw_texture_ex(texture, x - self.radius * 2.61125 / 2.0, y - self.radius * 1.17557 , WHITE, DrawTextureParams {
            dest_size: Some(Vec2 { x: self.radius * 2.61125, y: self.radius * 2.61125 }),
            source: None, rotation: 0.0, flip_x: false, flip_y: false, pivot: None,
        });

        let string = texture_store.balloon_string(2);
        draw_texture_ex(string, x - self.radius * 0.08881 / 2.0, y + self.radius * 1.1882, WHITE, DrawTextureParams {
            dest_size: Some(Vec2 { x: self.radius * 0.08881, y: self.radius * 2.2076}),
            source: None, rotation: 0.0, flip_x: false, flip_y: false,  pivot: None,
        } )
    }

    fn pos(&self, age: u32, window_size: (f32, f32)) -> Coord {
        (self.start_x, window_size.1 - (age * 1080 / (self.lifetime-60)) as f32 + self.radius)
    }

    fn depth(&self) -> Depth {
        Depth::Main(self.radius as i32)
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
        } else if self.born_time + self.lifetime + 150 < time {
            return true;
        }
        false
    }
}