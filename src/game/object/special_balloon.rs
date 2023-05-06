use std::sync::Arc;
use std::sync::mpsc::Sender;
use crate::game::object::balloon::{Balloon, BalloonColor};
use crate::game::object::{Coord, Depth, Object};
use crate::game::object::scoreboard::Scoreboard;
use crate::sound::SoundType;
use crate::texture::TextureStore;

pub struct SpecialBalloon {
    base: Balloon,
    effect: SpecialBalloonEffect,
    direction_is_right: bool,
}

impl SpecialBalloon {
    pub fn new(start_x: f32, radius: f32, born_time: u32, color: BalloonColor, lifetime: u32, effect: SpecialBalloonEffect) -> Self {
        SpecialBalloon {
            base: Balloon::new(
                start_x, radius, born_time, color, lifetime, 0,
            ),
            effect,
            direction_is_right: rand::random(),
        }
    }
}

impl Object for SpecialBalloon {
    fn draw(&self, center: Coord, age: u32, window_size: (f32, f32), texture_store: Arc<TextureStore>) {
        if let Some((shot_time, (x, y))) = self.base.shot_data {
            let shot_age = shot_time - self.base.born_time;
            self.base.draw_explosion(age, shot_age, x, y, texture_store.clone());
            let (text, rgb) = match &self.effect {
                SpecialBalloonEffect::MultiplyScore(times, duration) => {
                    let text = format!("x{} {}s", times, duration / 100);
                    let rgb = if *times > 0 { (0, 255, 0) } else { (255, 0, 0) };
                    (text, rgb)
                }
            };
            self.base.draw_point_text(age, shot_age, x, y, window_size.0 / 24.0, text.as_str(), rgb);

            return;
        }
        let (x, y) = center;
        let texture = texture_store.balloon(&self.base.color, 1);
        self.base.draw_balloon(x, y, texture);

        let relative_x = (age as f32 / self.base.lifetime as f32) * window_size.0 / 12.0;
        let derivative = 1.0 + 9.0 * (relative_x/9.0).cos();
        let mut flip = !self.direction_is_right;
        if derivative < 0.0 {
            flip = !flip;
        }
        self.base.draw_string(x, y, 1, flip, texture_store.clone());
    }

    fn pos(&self, age: u32, window_size: (f32, f32)) -> Coord {
        let (x, y) = self.base.pos(age, window_size);

        let relative_x = (age as f32 / self.base.lifetime as f32) * window_size.0 / 12.0;
        let x = if self.direction_is_right {
            x + relative_x + 81.0 * (relative_x/9.0).sin()
        } else {
            x - relative_x - 81.0 * (relative_x/9.0).sin()
        } ;
        (x, y)
    }

    fn depth(&self) -> Depth {
        self.base.depth()
    }

    fn max_age(&self) -> Option<u32> {
        self.base.max_age()
    }

    fn born_time(&self) -> u32 {
        self.base.born_time()
    }

    fn shoot_check(&self, coord: Coord, time: u32, window_size: (f32, f32)) -> Option<Coord> {
        let (x1, y1) = self.pos(time - self.born_time(), window_size);
        let (x2, y2) = coord;

        let long_side = self.base.radius;
        let short_side = self.base.radius * 0.8054;

        let long_side_square = long_side * long_side;
        let short_side_square = short_side * short_side;

        if (x2 - x1) * (x2 - x1) * long_side_square + (y2 - y1) * (y2 - y1) * short_side_square < long_side_square * short_side_square {
            Some((x1, y1))
        } else {
            None
        }
    }

    fn shoot(&mut self, coord: Coord, time: u32, client: u32, scoreboard: &mut Scoreboard, sound_tx: &mut Sender<SoundType>) {
        self.base.shoot(coord, time, client, scoreboard, sound_tx);
        if let SpecialBalloonEffect::MultiplyScore(by, duration) = self.effect {
            scoreboard.add_multiplication(by, client, time + duration);
        }
    }

    fn can_be_cleaned(&self, time: u32) -> bool {
        self.base.can_be_cleaned(time)
    }
}

pub enum SpecialBalloonEffect {
    MultiplyScore(i32, u32),
}