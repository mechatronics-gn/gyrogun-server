use std::sync::Arc;
use std::sync::mpsc::Sender;
use std::sync::mpsc::TrySendError::Full;
use macroquad::color::Color;
use crate::client::init::InitPhase;
use crate::client::Message;
use crate::game::Game;
use crate::game::object::{Object, ObjectWrapper};
use crate::game::object::correction_circle::CorrectionCircle;
use crate::game::object::full_screen_image::FullScreenImage;
use crate::sound::SoundType;

pub struct Tutorial {
    new_init_phase: Option<InitPhase>,
    last_change_time: u32,
    last_change_delay: u32,
    objects: Vec<Arc<Box<dyn Object + Send + Sync>>>,
    objects_was_updated: bool,
}

impl Tutorial {
    pub fn new(init_phase: InitPhase) -> Self {
        Self {
            new_init_phase: Some(init_phase),
            last_change_time: 0,
            last_change_delay: 0,
            objects: vec![],
            objects_was_updated: true,
        }
    }

    pub fn update_init_phase(&mut self, new_init_phase: InitPhase, time: u32, delay: u32) {
        self.new_init_phase = Some(new_init_phase);
        self.last_change_time = time;
        self.last_change_delay = delay;
    }

}

impl Game for Tutorial {
    fn on_time(&mut self, time: u32) {
        if time > self.last_change_delay + self.last_change_time {
            if let Some(x) = self.new_init_phase {
                self.new_init_phase = None;
                match x {
                    InitPhase::WaitMonitor => {
                        println!("Waitmonitor");
                        self.add_objects(Arc::new(Box::new(FullScreenImage::new(1, 0))));
                    }
                    InitPhase::WaitFirstPoint => {
                        println!("Waitfp");
                        self.add_objects(Arc::new(Box::new(FullScreenImage::new(2, 1))));
                        self.add_objects(Arc::new(Box::new(CorrectionCircle::new(true, 2))));
                    }
                    InitPhase::WaitSecondPoint => {
                        println!("Waitsp");
                        self.add_objects(Arc::new(Box::new(FullScreenImage::new(3, 3))));
                        self.add_objects(Arc::new(Box::new(CorrectionCircle::new(false, 4))));
                    }
                    InitPhase::Finalize => {
                        println!("finalize");
                        self.add_objects(Arc::new(Box::new(FullScreenImage::new(4, 5))));
                    }
                }
            }
        }
    }

    fn on_message(&mut self, client: u32, message: Message, time: u32, sound_tx: &mut Sender<SoundType>) {
    }

    fn objects(&mut self, _time: u32) -> Vec<ObjectWrapper> {
        self.objects.iter().map(|x| ObjectWrapper::Weak(Arc::downgrade(x))).collect()
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

    fn background_color(&self, _time: u32) -> Color {
        Color::from_rgba(147, 169, 209, 0)
    }
}