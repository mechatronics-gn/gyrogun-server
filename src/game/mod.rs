use std::collections::BinaryHeap;
use std::sync::{Arc, Weak};
use crate::game::object::{Object};
use crate::client::Message;
use crate::game::object::balloon::Balloon;

pub mod object;


pub trait Game {
    fn on_time(&mut self, time: u32);
    fn on_message(&mut self, client: u32, message: Message, time: u32);
    fn objects(&self) -> Vec<Weak<Box<dyn Object + Send + Sync>>>;
    fn add_objects(&mut self, object: Arc<Box<dyn Object + Send + Sync>>);
    fn was_objects_updated(&mut self) -> bool;
}

pub struct BalloonGame {
    window_size: (f32, f32),
    objects: Vec<Arc<Box<dyn Object + Send + Sync>>>,
    objects_was_updated: bool,
}

impl BalloonGame {
    pub fn from(window_size: (f32, f32)) -> Self {
        Self {
            window_size,
            objects: vec![],
            objects_was_updated: false,
        }
    }
}

impl Game for BalloonGame {
    fn on_time(&mut self, time: u32) {
        if time % 100 == 0 {
            let balloon = Balloon::from(
                rand::random::<f32>() * self.window_size.0,
                80.0,
                time,
                macroquad::color::PINK,
            360
            );
            let balloon: Arc<Box<dyn Object + Send + Sync>> = Arc::new(Box::new(balloon));
            self.add_objects(balloon.clone());
        }
        if time % 200 == 50 {
            let balloon = Balloon::from(
                rand::random::<f32>() * self.window_size.0,
                80.0,
                time,
                macroquad::color::ORANGE,
                240
            );
            let balloon: Arc<Box<dyn Object + Send+ Sync>> = Arc::new(Box::new(balloon));
            self.add_objects(balloon.clone());
        }
    }

    fn on_message(&mut self, client: u32, message: Message, time: u32) {
        match message {
            Message::Click(pos) => {
                let mut shooteds = vec![];
                let mut i = 0;
                while i < self.objects.len() {
                    if let Some(object_pos) = self.objects[i].shoot_check(pos, time, self.window_size) {
                        let x = self.objects.remove(i);
                        let x = Arc::try_unwrap(x);
                        if let Ok(mut x) = x {
                            x.shoot(object_pos, time);
                            shooteds.push(Arc::new(x));
                        }
                    } else {
                        i += 1;
                    }
                }
                for i in shooteds {
                    self.add_objects(i);
                }
            },
            _ => {

            }
        }

    }

    fn objects(&self) -> Vec<Weak<Box<dyn Object + Send + Sync>>> {
        self.objects.iter().map(|x| Arc::downgrade(x)).collect()
    }

    fn add_objects(&mut self, object: Arc<Box<dyn Object + Send + Sync>>) {
        self.objects.push(object);
        self.objects_was_updated = true;
    }

    fn was_objects_updated(&mut self) -> bool {
        if self.objects_was_updated {
            self.objects_was_updated = false;
            return true;
        }
        false
    }
}