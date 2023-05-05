use std::sync::Arc;
use macroquad::color::Color;
use crate::game::object::{Object, ObjectWrapper};
use crate::client::Message;

pub mod object;
pub mod balloon_game;


pub trait Game {
    fn on_time(&mut self, time: u32);
    fn on_message(&mut self, client: u32, message: Message, time: u32);
    fn objects(&mut self) -> Vec<ObjectWrapper>;
    fn add_objects(&mut self, object: Arc<Box<dyn Object + Send + Sync>>);
    fn was_objects_updated(&mut self) -> bool;
    fn background_color(&self, time: u32) -> Color;
}
