use std::rc::Rc;
use crate::game::object::{Coord};
use macroquad::prelude::*;
use super::Object;

pub struct Balloon {
    start_x: f32,
    radius: f32,
    color: Color,
    lifetime: u32,
    born_time: u32,
    shot_data: Option<(u32, Coord)>,
}

impl Balloon {
    pub fn from(start_x: f32, radius: f32, born_time: u32, color: Color, lifetime: u32) -> Self {
        Self {
            start_x,
            radius,
            color,
            lifetime,
            born_time,
            shot_data: None,
        }
    }
}

impl Object for Balloon {
    fn draw(&self, center: Coord, age: u32, window_size: (f32, f32)) {
        if let Some((shot_time, (x, y))) = self.shot_data {
            let shot_age = shot_time - self.born_time;
            if shot_age + 100 > age {
                draw_circle(x, y, self.radius * (shot_age + 100 - age) as f32 / 100.0 as f32, self.color);
            }
            return;
        }
        let (x, y) = center;
        draw_circle(x, y, self.radius, self.color);
    }

    fn pos(&self, age: u32, window_size: (f32, f32)) -> Coord {
        (self.start_x, window_size.1 - (age * 1080 / (self.lifetime-60)) as f32 + self.radius / 2.0)
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

    fn shoot(&mut self, coord: Coord, time: u32) {
        if let None = self.shot_data {
            self.shot_data = Some((time, coord));
        }
    }

    fn can_be_cleaned(&self, time: u32) -> bool {
        if let Some((shot_time, _)) = self.shot_data {
            if time > shot_time + 100 {
                return true;
            }
        }
        false
    }
}