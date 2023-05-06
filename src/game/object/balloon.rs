use std::sync::{Arc, mpsc};
use crate::game::object::{Coord, Depth};
use macroquad::prelude::*;
use crate::game::object::scoreboard::Scoreboard;
use crate::sound::SoundType;
use crate::texture::TextureStore;
use super::Object;

pub struct Balloon {
    start_x: f32,
    pub(super) radius: f32,
    pub(super) color: BalloonColor,
    pub(super) lifetime: u32,
    shoot_points: i32,
    pub(super) born_time: u32,
    pub(super) shot_data: Option<(u32, Coord)>,
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

    pub(super) fn draw_balloon(&self, x: f32, y: f32, texture: Texture2D) {
        draw_texture_ex(texture, x - self.radius * 2.61125 / 2.0, y - self.radius * 1.17557 , WHITE, DrawTextureParams {
            dest_size: Some(Vec2 { x: self.radius * 2.61125, y: self.radius * 2.61125 }),
            source: None, rotation: 0.0, flip_x: false, flip_y: false, pivot: None,
        });
    }

    pub(super) fn draw_explosion(&self, age: u32, shot_age: u32, x: f32, y: f32, texture_store: Arc<TextureStore>) {
        if shot_age + 50 > age && age > shot_age {
            let variant = (age - shot_age) / 10 + 2;
            let texture = texture_store.balloon(&self.color, variant as i32);
            self.draw_balloon(x, y, texture);
        }
    }

    pub(super) fn draw_point_text(&self, age: u32, shot_age: u32, x: f32, y: f32, font_size: f32, text: &str, (r, g, b): (u8, u8, u8)) {
        let alpha = if age < shot_age + 33 {
            ((age - shot_age) * 255 / 34) as u8
        } else if age < shot_age + 66 {
            255
        } else if age < shot_age + 100 {
            255 - ((age - shot_age - 66) * 255 / 35) as u8
        } else {
            0
        };
        draw_text(text, x, y, font_size, Color::from_rgba(r, g, b, alpha));
    }

    pub(super) fn draw_string(&self, x: f32, y: f32, variant: i32, flip: bool, texture_store: Arc<TextureStore>) {
        if variant == 1 {
            let string = texture_store.balloon_string(1);
            draw_texture_ex(string, x - self.radius * 0.95914 / 2.0, y + self.radius * 1.1882, WHITE, DrawTextureParams {
                dest_size: Some(Vec2 { x: self.radius * 0.95914, y: self.radius * 1.6834}),
                source: None, rotation: 0.0, flip_x: flip, flip_y: false, pivot: None,
            })
        } else if variant == 2 {
            let string = texture_store.balloon_string(2);
            draw_texture_ex(string, x - self.radius * 0.08881 / 2.0, y + self.radius * 1.1882, WHITE, DrawTextureParams {
                dest_size: Some(Vec2 { x: self.radius * 0.08881, y: self.radius * 2.2076}),
                source: None, rotation: 0.0, flip_x: false, flip_y: false,  pivot: None,
            } )
        }
    }
}

impl Object for Balloon {
    fn draw(&self, center: Coord, age: u32, window_size: (f32, f32), texture_store: Arc<TextureStore>) {
        if let Some((shot_time, (x, y))) = self.shot_data {
            let shot_age = shot_time - self.born_time;
            self.draw_explosion(age, shot_age, x, y, texture_store.clone());
            let text = if self.shoot_points > 0 { format!("+{}", self.shoot_points) } else { format!("{}", self.shoot_points) };
            let rgb = if self.shoot_points > 0 { (0, 255, 0) } else { (255, 0, 0) };
            self.draw_point_text(age, shot_age, x, y, window_size.0 / 18.0, text.as_str(), rgb);

            return;
        }
        let (x, y) = center;
        let texture = texture_store.balloon(&self.color, 1);
        self.draw_balloon(x, y, texture);
        self.draw_string(x, y, 2, false, texture_store.clone());
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

        let long_side = self.radius;
        let short_side = self.radius * 0.8054;

        let long_side_square = long_side * long_side;
        let short_side_square = short_side * short_side;

        if (x2 - x1) * (x2 - x1) * long_side_square + (y2 - y1) * (y2 - y1) * short_side_square < long_side_square * short_side_square {
            Some((x1, y1))
        } else {
            None
        }
    }

    fn shoot(&mut self, coord: Coord, time: u32, client: u32, scoreboard: &mut Scoreboard, sound_tx: &mut mpsc::Sender<SoundType>) {
        if let None = self.shot_data {
            self.shot_data = Some((time, coord));
            self.shoot_points = scoreboard.update(client, self.shoot_points, time);
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