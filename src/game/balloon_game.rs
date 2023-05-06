use std::sync::{Arc, mpsc};
use macroquad::color::Color;
use crate::client::Message;
use crate::game::Game;
use crate::game::object::balloon::{Balloon, BalloonColor};
use crate::game::object::{Object, ObjectWrapper};
use crate::game::object::cloud::Cloud;
use crate::game::object::scoreboard::{Scoreboard, ScoreboardObject};
use crate::game::object::special_balloon::{SpecialBalloon, SpecialBalloonEffect};
use crate::sound::SoundType;
use crate::wait_unwrap_and_map;

pub struct BalloonGame {
    window_size: (f32, f32),
    objects: Vec<Arc<Box<dyn Object + Send + Sync>>>,
    objects_was_updated: bool,
    scoreboard: Scoreboard,
}

impl BalloonGame {
    pub fn new(window_size: (f32, f32), player_count: u32) -> Self {
        Self {
            window_size,
            objects: vec![],
            objects_was_updated: false,
            scoreboard: Scoreboard::new(player_count),
        }
    }
}

impl Game for BalloonGame {
    fn on_time(&mut self, time: u32) {
        if time % if time > 6000 { 200 } else { 100 } == 0 {
            let balloon = Balloon::new(
                rand::random::<f32>() * self.window_size.0,
                self.window_size.0 / 24.0 * (rand::random::<f32>() * 0.2 + 1.0),
                time,
                BalloonColor::Pink,
                360,
                1
            );
            let balloon: Arc<Box<dyn Object + Send + Sync>> = Arc::new(Box::new(balloon));
            self.add_objects(balloon.clone());
        }
        if time % if time > 6000 { 100 } else { 200 } == 50 {
            let balloon = Balloon::new(
                rand::random::<f32>() * self.window_size.0,
                self.window_size.0 / 24.0 * (rand::random::<f32>() * 0.2 + 1.0),
                time,
                BalloonColor::Orange,
                240,
                2
            );
            let balloon: Arc<Box<dyn Object + Send+ Sync>> = Arc::new(Box::new(balloon));
            self.add_objects(balloon.clone());
        }
        if rand::random::<i32>() % 3000 == 0 {
            let balloon = Balloon::new(
                rand::random::<f32>() * self.window_size.0,
                self.window_size.0 / 24.0 * (rand::random::<f32>() * 0.2 + 1.0),
                time,
                BalloonColor::Green,
                160,
                5
            );
            let balloon: Arc<Box<dyn Object + Send + Sync>> = Arc::new(Box::new(balloon));
            self.add_objects(balloon.clone());
        }
        if rand::random::<i32>() % 1200 == 0 {
            let balloon = SpecialBalloon::new(
                rand::random::<f32>() * self.window_size.0,
                self.window_size.0 / 24.0 * (rand::random::<f32>() * 0.2 + 1.0),
                time,
                BalloonColor::Yellow,
                270,
                SpecialBalloonEffect::MultiplyScore(2, 1000),
            );
            let balloon: Arc<Box<dyn Object + Send + Sync>> = Arc::new(Box::new(balloon));
            self.add_objects(balloon.clone());
        }
        if time % 150 == 20 {
            let balloon = Balloon::new(
                rand::random::<f32>() * self.window_size.0,
                self.window_size.0/24.0 * (rand::random::<f32>() * 0.2 + 1.0),
                time,
                BalloonColor::Purple,
                300,
                -((rand::random::<u32>() % 3) as i32 + 1)
            );
            let balloon: Arc<Box<dyn Object + Send + Sync>> = Arc::new(Box::new(balloon));
            self.add_objects(balloon.clone());
        }
        if time % 400 == 70 {
            let cloud = Cloud::new(
                rand::random::<f32>() * self.window_size.1,
                rand::random::<f32>() * 320.0 + 240.0,
                rand::random::<u32>() % 600 + 1200,
                time,
            );
            let cloud: Arc<Box<dyn Object + Send + Sync>> = Arc::new(Box::new(cloud));
            self.add_objects(cloud.clone());
        }


        let mut i = 0;
        while i < self.objects.len() {
            if self.objects[i].can_be_cleaned(time) {
                self.objects.remove(i);
            }
            i += 1;
        }
    }

    fn on_message(&mut self, client: u32, message: Message, time: u32, sound_tx: &mut mpsc::Sender<SoundType>) {
        match message {
            Message::Click(pos) => {
                let mut shooteds = vec![];
                let mut i = 0;
                while i < self.objects.len() {
                    if let Some(object_pos) = self.objects[i].shoot_check(pos, time, self.window_size) {
                        let x = self.objects.remove(i);
                        wait_unwrap_and_map(x, |mut x| {
                            x.shoot(object_pos, time, client, &mut self.scoreboard, sound_tx);
                            // this causes a scoreboard change, resulting in a object update
                            self.objects_was_updated = true;
                            shooteds.push(Arc::new(x));
                        });
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

    fn objects(&mut self) -> Vec<ObjectWrapper> {
        let mut ret: Vec<ObjectWrapper> = self.objects.iter().map(|x| ObjectWrapper::Weak(Arc::downgrade(x))).collect();
        ret.push(ObjectWrapper::Arc(Arc::new(Box::new(ScoreboardObject::from(&self.scoreboard)))));
        return ret;
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

    fn background_color(&self, time: u32) -> Color {
        Color::from_rgba(147, 169, 209, 0)
    }
}
