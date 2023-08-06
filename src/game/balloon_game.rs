use std::sync::{Arc, mpsc};
use macroquad::color::Color;
use crate::client::Message;
use crate::game::Game;
use crate::game::object::balloon::{Balloon, BalloonColor};
use crate::game::object::{Object, ObjectWrapper};
use crate::game::object::cloud::Cloud;
use crate::game::object::scoreboard::{Scoreboard, ScoreboardObject};
use crate::game::object::special_balloon::{SpecialBalloon, SpecialBalloonEffect};
use crate::game::object::timer::Timer;
use crate::sound::SoundType;
use crate::wait_unwrap_and_map;

pub struct BalloonGame {
    window_size: (f32, f32),
    objects: Vec<Arc<Box<dyn Object + Send + Sync>>>,
    objects_was_updated: bool,
    scoreboard_was_updated: bool,
    scoreboard: Scoreboard,
    latest_scoreboard_object: ScoreboardObject,
}

impl BalloonGame {
    pub fn new(window_size: (f32, f32), player_count: u32, duration: u32) -> Self {
        Self {
            window_size,
            objects: vec![Arc::new(Box::new(Timer::new(duration)))],
            objects_was_updated: false,
            scoreboard_was_updated: false,
            scoreboard: Scoreboard::new(player_count),
            latest_scoreboard_object: ScoreboardObject::new(0, window_size, player_count),
        }
    }

    pub fn scores(&self) -> Vec<i32> {
        self.scoreboard.scores()
    }

    fn was_scoreboard_updated(&mut self) -> bool {
        if self.scoreboard_was_updated {
            self.scoreboard_was_updated = false;
            return true;
        }
        false
    }

}

impl Game for BalloonGame {
    fn on_time(&mut self, time: u32) {
        if time % if time > 6000 { 66 } else { 33 } == 0 {
            let balloon = Balloon::new(
                rand::random::<f32>() * self.window_size.0,
                self.window_size.0 / 32.0 * (rand::random::<f32>() * 0.2 + 1.0),
                time,
                BalloonColor::Orange,
                360,
                1
            );
            let balloon: Arc<Box<dyn Object + Send + Sync>> = Arc::new(Box::new(balloon));
            self.add_objects(balloon.clone());
        }
        if time % if time > 6000 { 33 } else { 66 } == 25 {
            let balloon = Balloon::new(
                rand::random::<f32>() * self.window_size.0,
                self.window_size.0 / 32.0 * (rand::random::<f32>() * 0.2 + 1.0),
                time,
                BalloonColor::Red,
                240,
                2
            );
            let balloon: Arc<Box<dyn Object + Send+ Sync>> = Arc::new(Box::new(balloon));
            self.add_objects(balloon.clone());
        }
        if rand::random::<i32>() % 1000 == 0 {
            let balloon = Balloon::new(
                rand::random::<f32>() * self.window_size.0,
                self.window_size.0 / 32.0 * (rand::random::<f32>() * 0.2 + 1.0),
                time,
                BalloonColor::Green,
                160,
                5
            );
            let balloon: Arc<Box<dyn Object + Send + Sync>> = Arc::new(Box::new(balloon));
            self.add_objects(balloon.clone());
        }
        if rand::random::<i32>() % 400 == 0 {
            let balloon = SpecialBalloon::new(
                rand::random::<f32>() * self.window_size.0,
                self.window_size.0 / 32.0 * (rand::random::<f32>() * 0.2 + 1.0),
                time,
                BalloonColor::Yellow,
                270,
                SpecialBalloonEffect::MultiplyScore(2, 1000),
            );
            let balloon: Arc<Box<dyn Object + Send + Sync>> = Arc::new(Box::new(balloon));
            self.add_objects(balloon.clone());
        }
        if time % 50 == 20 {
            let balloon = Balloon::new(
                rand::random::<f32>() * self.window_size.0,
                self.window_size.0 / 32.0 * (rand::random::<f32>() * 0.2 + 1.0),
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
                rand::random::<f32>() * 320.0 + 360.0,
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
                            self.scoreboard_was_updated = true;
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

    fn objects(&mut self, time: u32) -> Vec<ObjectWrapper> {
        let mut ret: Vec<ObjectWrapper> = self.objects.iter().map(|x| ObjectWrapper::Weak(Arc::downgrade(x))).collect();
        if self.was_scoreboard_updated() {
            let scoreboard_object = ScoreboardObject::from(&self.scoreboard, &self.latest_scoreboard_object, time, 150, self.window_size);
            ret.push(ObjectWrapper::Arc(Arc::new(Box::new(scoreboard_object.clone()))));
            self.latest_scoreboard_object = scoreboard_object;
        } else {
            ret.push(ObjectWrapper::Arc(Arc::new(Box::new(self.latest_scoreboard_object.clone()))));
        }
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

    fn background_color(&self, _time: u32) -> Color {
        Color::from_rgba(147, 169, 209, 0)
    }
}
