use std::sync::Arc;
use macroquad::color::Color;
use macroquad::prelude::draw_text;
use macroquad::text::measure_text;
use crate::game::object::balloon::BalloonColor;

pub mod client;
pub mod display;
pub mod game;
pub mod sound;
pub mod texture;

/* This function is not rational */
pub fn wait_unwrap_and_map<T, V, F>(arc: Arc<T>, call: F) -> V
    where F: FnOnce(T) -> V
{
    let x = Arc::try_unwrap(arc);
    match x {
        Ok(x) => {
            call(x)
        }
        Err(x) => {
            while Arc::strong_count(&x) > 1 {};
            wait_unwrap_and_map(x, call)
        }
    }
}

pub fn draw_text_center_align(text: &str, x: f32, y: f32, font_size: f32, color: Color) {
    let dimensions = measure_text(text, None, font_size as u16, 1.);
    draw_text(text, x - dimensions.width / 2., y - dimensions.height / 2. + dimensions.offset_y, font_size, color);
}

pub fn player_to_color(id: usize) -> Color {
    match id {
        0 => Color::from_rgba(226, 0, 1, 255),
        1 => Color::from_rgba(0, 189, 1, 255),
        2 => Color::from_rgba(248, 213, 60, 255),
        3 => Color::from_rgba(57, 32, 214, 255),
        _ => macroquad::color::WHITE,
    }
}

pub fn player_to_balloon_color(id: usize) -> BalloonColor {
    match id {
        0 => BalloonColor::Red,
        1 => BalloonColor::Green,
        2 => BalloonColor::Yellow,
        3 => BalloonColor::Blue,
        _ => BalloonColor::Orange,
    }
}