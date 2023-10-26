use std::collections::HashMap;
use macroquad::audio::{load_sound_from_bytes, Sound};

#[derive(Eq, PartialEq, Hash)]
pub enum SoundType {
    BalloonExplosion,
}

pub struct SoundStore {
    store: HashMap<SoundType, Sound>
}

impl SoundStore {
    pub async fn new() -> SoundStore {
        let mut store = HashMap::new();

        store.insert(SoundType::BalloonExplosion, load_sound_from_bytes(include_bytes!("../res/mixkit-ballon-blows-up-3071.wav")).await.unwrap());

        SoundStore {
            store
        }
    }

    pub fn get(&self, sound_type: &SoundType) -> Option<Sound> {
        self.store.get(sound_type).map_or(None, |x| Some(x.clone()))
    }
}