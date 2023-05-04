use std::rc::Rc;
use crate::game::object::{Coord};
use macroquad::prelude::*;
use super::Object;

pub struct Balloon {
    start_x: f32,
    radius: f32,
    born_time: u32,
    shot_time: Option<u32>,
}

impl Balloon {
    pub fn from(start_x: f32, radius: f32, born_time: u32,) -> Self {
        Self {
            start_x,
            radius,
            born_time,
            shot_time: None,
        }
    }
}

impl Object for Balloon {
    fn draw(&self, center: Coord, age: u32, window_size: (f32, f32)) {
        if let Some(_) = self.shot_time {
            return;
        }
        let (x, y) = center;
        draw_circle(x, y, self.radius, GREEN);
    }

    fn pos(&self, age: u32, window_size: (f32, f32)) -> Coord {
        (self.start_x, window_size.1 - age as f32 * 3.6 + self.radius / 2.0)
    }

    fn max_age(&self) -> Option<u32> {
        Some(360)
    }

    fn born_time(&self) -> u32 {
        self.born_time
    }

    fn shoot_check(&self, coord: Coord, time: u32, window_size: (f32, f32)) -> bool {
        let (x1, y1) = self.pos(time - self.born_time(), window_size);
        let (x2, y2) = coord;

        (x2 - x1) * (x2 - x1) + (y2 - y1) * (y2 - y1) < self.radius * self.radius
    }

    fn shoot(&mut self, time: u32) {
        if let None = self.shot_time {
            self.shot_time = Some(time);
        }
    }
}