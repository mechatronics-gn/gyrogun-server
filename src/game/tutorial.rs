use std::sync::Arc;
use std::sync::mpsc::Sender;
use macroquad::color::Color;
use crate::client::init::InitPhase;
use crate::client::Message;
use crate::game::Game;
use crate::game::object::{Object, ObjectWrapper};
use crate::sound::SoundType;

pub struct Tutorial {
    current_init_phase: InitPhase,
    last_change_time: u32,
    last_change_delay: u32,

}

impl Tutorial {
    pub fn new(init_phase: InitPhase) -> Self {
        Self {
            current_init_phase: init_phase,
            last_change_time: 0,
            last_change_delay: 0,
        }
    }

    pub fn update_init_phase(&mut self, new_init_phase: InitPhase, time: u32, delay: u32) {
        self.current_init_phase = new_init_phase;
        self.last_change_time = time;
        self.last_change_delay = delay;
    }

}

impl Game for Tutorial {
    fn on_time(&mut self, time: u32) {
        todo!()
    }

    fn on_message(&mut self, client: u32, message: Message, time: u32, sound_tx: &mut Sender<SoundType>) {
        todo!()
    }

    fn objects(&mut self, time: u32) -> Vec<ObjectWrapper> {
        todo!()
    }

    fn add_objects(&mut self, object: Arc<Box<dyn Object + Send + Sync>>) {
        todo!()
    }

    fn was_objects_updated(&mut self) -> bool {
        todo!()
    }

    fn background_color(&self, time: u32) -> Color {
        todo!()
    }
}