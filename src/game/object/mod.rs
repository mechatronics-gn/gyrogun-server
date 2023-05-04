pub mod balloon;
pub mod scoreboard;

type Coord = (f32, f32);

pub trait Object {
    fn draw(&self, center: Coord, age: u32, window_size: (f32, f32));
    fn pos(&self, age: u32, window_size: (f32, f32)) -> Coord;
    fn max_age(&self) -> Option<u32>;
    fn born_time(&self) -> u32;
    fn shoot_check(&self, coord: Coord, time: u32, window_size: (f32, f32)) -> bool;
    fn shoot(&mut self, time: u32);
}