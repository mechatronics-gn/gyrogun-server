use std::sync::{Arc, mpsc};
use macroquad::color::Color;
use crate::game::object::{Object, ObjectWrapper};
use crate::client::Message;
use crate::sound::SoundType;

pub mod object;
pub mod balloon_game;


pub trait Game {
    fn on_time(&mut self, time: u32);
    fn on_message(&mut self, client: u32, message: Message, time: u32, sound_tx: &mut mpsc::Sender<SoundType>);
    fn objects(&mut self, time: u32) -> Vec<ObjectWrapper>;
    fn add_objects(&mut self, object: Arc<Box<dyn Object + Send + Sync>>);
    fn was_objects_updated(&mut self) -> bool;
    fn was_scoreboard_updated(&mut self) -> bool;
    fn background_color(&self, time: u32) -> Color;
}
