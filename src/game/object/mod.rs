use std::cmp::Ordering;
use std::sync::{Arc, mpsc, Weak};
use crate::game::object::scoreboard::Scoreboard;
use crate::sound::SoundType;
use crate::texture::TextureStore;

pub mod balloon;
pub mod scoreboard;
pub mod cloud;
pub mod special_balloon;
pub mod timer;
pub mod game_result;

type Coord = (f32, f32);

pub trait Object {
    fn draw(&self, center: Coord, age: u32, window_size: (f32, f32), texture_store: Arc<TextureStore>);
    fn pos(&self, age: u32, window_size: (f32, f32)) -> Coord;
    fn depth(&self) -> Depth;
    fn max_age(&self) -> Option<u32>;
    fn born_time(&self) -> u32;
    fn shoot_check(&self, coord: Coord, time: u32, window_size: (f32, f32)) -> Option<Coord>;
    fn shoot(&mut self, coord: Coord, time: u32, client: u32, scoreboard: &mut Scoreboard, sound_tx: &mut mpsc::Sender<SoundType>);
    fn can_be_cleaned(&self, time: u32) -> bool;
}

#[derive(Clone)]
pub enum ObjectWrapper {
    Weak(Weak<Box<dyn Object + Send + Sync>>),
    Arc(Arc<Box<dyn Object + Send + Sync>>),
}

#[derive(Eq, PartialEq)]
pub enum Depth {
    Background(i32),
    Main(i32),
    Foreground(i32),
}

impl PartialOrd for Depth {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self {
            Depth::Background(i) => {
                if let Depth::Background(j) = other {
                    Some(i.cmp(j))
                } else {
                    Some(Ordering::Less)
                }
            }
            Depth::Main(i) => {
                match other {
                    Depth::Background(_) => { Some(Ordering::Greater) }
                    Depth::Main(j) => { Some(i.cmp(j)) }
                    Depth::Foreground(_) => { Some(Ordering::Less) }
                }
            }
            Depth::Foreground(i) => {
                if let Depth::Foreground(j) = other {
                    Some(i.cmp(j))
                } else {
                    Some(Ordering::Greater)
                }
            }
        }
    }
}

impl Ord for Depth {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_or(Ordering::Equal)
    }
}
