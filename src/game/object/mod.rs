use std::sync::{Arc, Weak};
use crate::game::object::scoreboard::Scoreboard;

pub mod balloon;
pub mod scoreboard;

type Coord = (f32, f32);

pub trait Object {
    fn draw(&self, center: Coord, age: u32, window_size: (f32, f32));
    fn pos(&self, age: u32, window_size: (f32, f32)) -> Coord;
    fn depth(&self) -> Depth;
    fn max_age(&self) -> Option<u32>;
    fn born_time(&self) -> u32;
    fn shoot_check(&self, coord: Coord, time: u32, window_size: (f32, f32)) -> Option<Coord>;
    fn shoot(&mut self, coord: Coord, time: u32, client: u32, scoreboard: &mut Scoreboard);
    fn can_be_cleaned(&self, time: u32) -> bool;
}

#[derive(Clone)]
pub enum ObjectWrapper {
    Weak(Weak<Box<dyn Object + Send + Sync>>),
    Arc(Arc<Box<dyn Object + Send + Sync>>),
}

#[derive(PartialOrd, Ord, PartialEq, Eq)]
pub enum Depth {
    Background,
    Main,
    Foreground,
}