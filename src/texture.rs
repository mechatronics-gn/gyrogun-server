use std::collections::HashMap;
use macroquad::prelude::ImageFormat;
use macroquad::texture::Texture2D;
use crate::game::object::balloon::BalloonColor;

pub struct TextureStore {
    store: HashMap<String, Texture2D>
}

impl TextureStore {
    pub fn new() -> Self {
        let mut map = HashMap::new();

        map.insert("crosshair-red".to_string(), Texture2D::from_file_with_format(include_bytes!("../res/crosshair_red_small.png"), Some(ImageFormat::Png)));
        map.insert("crosshair-green".to_string(), Texture2D::from_file_with_format(include_bytes!("../res/crosshair_green_small.png"), Some(ImageFormat::Png)));
        map.insert("crosshair-yellow".to_string(), Texture2D::from_file_with_format(include_bytes!("../res/crosshair_yellow_small.png"), Some(ImageFormat::Png)));
        map.insert("crosshair-blue".to_string(), Texture2D::from_file_with_format(include_bytes!("../res/crosshair_blue_small.png"), Some(ImageFormat::Png)));

        map.insert("balloon-blue-1".to_string(), Texture2D::from_file_with_format(include_bytes!("../res/balloons/blue-balloon/1.png"), Some(ImageFormat::Png)));
        map.insert("balloon-blue-2".to_string(), Texture2D::from_file_with_format(include_bytes!("../res/balloons/blue-balloon/2.png"), Some(ImageFormat::Png)));
        map.insert("balloon-blue-3".to_string(), Texture2D::from_file_with_format(include_bytes!("../res/balloons/blue-balloon/3.png"), Some(ImageFormat::Png)));
        map.insert("balloon-blue-4".to_string(), Texture2D::from_file_with_format(include_bytes!("../res/balloons/blue-balloon/4.png"), Some(ImageFormat::Png)));
        map.insert("balloon-blue-5".to_string(), Texture2D::from_file_with_format(include_bytes!("../res/balloons/blue-balloon/5.png"), Some(ImageFormat::Png)));
        map.insert("balloon-blue-6".to_string(), Texture2D::from_file_with_format(include_bytes!("../res/balloons/blue-balloon/6.png"), Some(ImageFormat::Png)));
        map.insert("balloon-green-1".to_string(), Texture2D::from_file_with_format(include_bytes!("../res/balloons/green-balloon/1.png"), Some(ImageFormat::Png)));
        map.insert("balloon-green-2".to_string(), Texture2D::from_file_with_format(include_bytes!("../res/balloons/green-balloon/2.png"), Some(ImageFormat::Png)));
        map.insert("balloon-green-3".to_string(), Texture2D::from_file_with_format(include_bytes!("../res/balloons/green-balloon/3.png"), Some(ImageFormat::Png)));
        map.insert("balloon-green-4".to_string(), Texture2D::from_file_with_format(include_bytes!("../res/balloons/green-balloon/4.png"), Some(ImageFormat::Png)));
        map.insert("balloon-green-5".to_string(), Texture2D::from_file_with_format(include_bytes!("../res/balloons/green-balloon/5.png"), Some(ImageFormat::Png)));
        map.insert("balloon-green-6".to_string(), Texture2D::from_file_with_format(include_bytes!("../res/balloons/green-balloon/6.png"), Some(ImageFormat::Png)));
        map.insert("balloon-orange-1".to_string(), Texture2D::from_file_with_format(include_bytes!("../res/balloons/orange-balloon/1.png"), Some(ImageFormat::Png)));
        map.insert("balloon-orange-2".to_string(), Texture2D::from_file_with_format(include_bytes!("../res/balloons/orange-balloon/2.png"), Some(ImageFormat::Png)));
        map.insert("balloon-orange-3".to_string(), Texture2D::from_file_with_format(include_bytes!("../res/balloons/orange-balloon/3.png"), Some(ImageFormat::Png)));
        map.insert("balloon-orange-4".to_string(), Texture2D::from_file_with_format(include_bytes!("../res/balloons/orange-balloon/4.png"), Some(ImageFormat::Png)));
        map.insert("balloon-orange-5".to_string(), Texture2D::from_file_with_format(include_bytes!("../res/balloons/orange-balloon/5.png"), Some(ImageFormat::Png)));
        map.insert("balloon-orange-6".to_string(), Texture2D::from_file_with_format(include_bytes!("../res/balloons/orange-balloon/6.png"), Some(ImageFormat::Png)));
        map.insert("balloon-pink-1".to_string(), Texture2D::from_file_with_format(include_bytes!("../res/balloons/pink-balloon/1.png"), Some(ImageFormat::Png)));
        map.insert("balloon-pink-2".to_string(), Texture2D::from_file_with_format(include_bytes!("../res/balloons/pink-balloon/2.png"), Some(ImageFormat::Png)));
        map.insert("balloon-pink-3".to_string(), Texture2D::from_file_with_format(include_bytes!("../res/balloons/pink-balloon/3.png"), Some(ImageFormat::Png)));
        map.insert("balloon-pink-4".to_string(), Texture2D::from_file_with_format(include_bytes!("../res/balloons/pink-balloon/4.png"), Some(ImageFormat::Png)));
        map.insert("balloon-pink-5".to_string(), Texture2D::from_file_with_format(include_bytes!("../res/balloons/pink-balloon/5.png"), Some(ImageFormat::Png)));
        map.insert("balloon-pink-6".to_string(), Texture2D::from_file_with_format(include_bytes!("../res/balloons/pink-balloon/6.png"), Some(ImageFormat::Png)));
        map.insert("balloon-purple-1".to_string(), Texture2D::from_file_with_format(include_bytes!("../res/balloons/purple-balloon/1.png"), Some(ImageFormat::Png)));
        map.insert("balloon-purple-2".to_string(), Texture2D::from_file_with_format(include_bytes!("../res/balloons/purple-balloon/2.png"), Some(ImageFormat::Png)));
        map.insert("balloon-purple-3".to_string(), Texture2D::from_file_with_format(include_bytes!("../res/balloons/purple-balloon/3.png"), Some(ImageFormat::Png)));
        map.insert("balloon-purple-4".to_string(), Texture2D::from_file_with_format(include_bytes!("../res/balloons/purple-balloon/4.png"), Some(ImageFormat::Png)));
        map.insert("balloon-purple-5".to_string(), Texture2D::from_file_with_format(include_bytes!("../res/balloons/purple-balloon/5.png"), Some(ImageFormat::Png)));
        map.insert("balloon-purple-6".to_string(), Texture2D::from_file_with_format(include_bytes!("../res/balloons/purple-balloon/6.png"), Some(ImageFormat::Png)));
        map.insert("balloon-red-1".to_string(), Texture2D::from_file_with_format(include_bytes!("../res/balloons/red-balloon/1.png"), Some(ImageFormat::Png)));
        map.insert("balloon-red-2".to_string(), Texture2D::from_file_with_format(include_bytes!("../res/balloons/red-balloon/2.png"), Some(ImageFormat::Png)));
        map.insert("balloon-red-3".to_string(), Texture2D::from_file_with_format(include_bytes!("../res/balloons/red-balloon/3.png"), Some(ImageFormat::Png)));
        map.insert("balloon-red-4".to_string(), Texture2D::from_file_with_format(include_bytes!("../res/balloons/red-balloon/4.png"), Some(ImageFormat::Png)));
        map.insert("balloon-red-5".to_string(), Texture2D::from_file_with_format(include_bytes!("../res/balloons/red-balloon/5.png"), Some(ImageFormat::Png)));
        map.insert("balloon-red-6".to_string(), Texture2D::from_file_with_format(include_bytes!("../res/balloons/red-balloon/6.png"), Some(ImageFormat::Png)));
        map.insert("balloon-yellow-1".to_string(), Texture2D::from_file_with_format(include_bytes!("../res/balloons/yellow-balloon/1.png"), Some(ImageFormat::Png)));
        map.insert("balloon-yellow-2".to_string(), Texture2D::from_file_with_format(include_bytes!("../res/balloons/yellow-balloon/2.png"), Some(ImageFormat::Png)));
        map.insert("balloon-yellow-3".to_string(), Texture2D::from_file_with_format(include_bytes!("../res/balloons/yellow-balloon/3.png"), Some(ImageFormat::Png)));
        map.insert("balloon-yellow-4".to_string(), Texture2D::from_file_with_format(include_bytes!("../res/balloons/yellow-balloon/4.png"), Some(ImageFormat::Png)));
        map.insert("balloon-yellow-5".to_string(), Texture2D::from_file_with_format(include_bytes!("../res/balloons/yellow-balloon/5.png"), Some(ImageFormat::Png)));
        map.insert("balloon-yellow-6".to_string(), Texture2D::from_file_with_format(include_bytes!("../res/balloons/yellow-balloon/6.png"), Some(ImageFormat::Png)));

        map.insert("balloon-string-1".to_string(), Texture2D::from_file_with_format(include_bytes!("../res/balloons/string-1.png"), Some(ImageFormat::Png)));
        map.insert("balloon-string-2".to_string(), Texture2D::from_file_with_format(include_bytes!("../res/balloons/string-2.png"), Some(ImageFormat::Png)));

        map.insert("cloud-1".to_string(), Texture2D::from_file_with_format(include_bytes!("../res/cloud1.PNG"), Some(ImageFormat::Png)));
        map.insert("cloud-2".to_string(), Texture2D::from_file_with_format(include_bytes!("../res/cloud2.PNG"), Some(ImageFormat::Png)));
        map.insert("cloud-3".to_string(), Texture2D::from_file_with_format(include_bytes!("../res/cloud3.PNG"), Some(ImageFormat::Png)));

        map.insert("fsi-1".to_string(), Texture2D::from_file_with_format(include_bytes!("../res/fsi1.png"), Some(ImageFormat::Png)));
        map.insert("fsi-2".to_string(), Texture2D::from_file_with_format(include_bytes!("../res/fsi2.png"), Some(ImageFormat::Png)));
        map.insert("fsi-3".to_string(), Texture2D::from_file_with_format(include_bytes!("../res/fsi3.png"), Some(ImageFormat::Png)));
        map.insert("fsi-4".to_string(), Texture2D::from_file_with_format(include_bytes!("../res/fsi4.png"), Some(ImageFormat::Png)));

        TextureStore {
            store: map
        }
    }

    pub fn crosshair(&self, variant: i32) -> Texture2D {
        self.store.get(format!("crosshair-{}", match variant { 0 => { "red" }, 1 => { "green" }, 2 => { "yellow" }, 3 => { "blue" }, _ => { "" } }).as_str()).map_or(Texture2D::empty(), |x| *x)
    }

    pub fn balloon(&self, color: &BalloonColor, variant: i32) -> Texture2D {
        let color = match color {
            BalloonColor::Blue => { "blue" }
            BalloonColor::Green => { "green" }
            BalloonColor::Orange => { "orange" }
            BalloonColor::Pink => { "pink" }
            BalloonColor::Purple => { "purple" }
            BalloonColor::Red => { "red" }
            BalloonColor::Yellow => { "yellow" }
        };
        self.store.get(format!("balloon-{}-{}", color, variant).as_str()).map_or(Texture2D::empty(), |x| *x)
    }

    pub fn balloon_string(&self, variant: i32) -> Texture2D {
        self.store.get(format!("balloon-string-{}", variant).as_str()).map_or(Texture2D::empty(), |x| *x)
    }
    
    pub fn cloud(&self, variant: i32) -> Texture2D {
        self.store.get(format!("cloud-{}", variant).as_str()).map_or(Texture2D::empty(), |x| *x)
    }

    pub fn full_screen_image(&self, idx: i32) -> Texture2D {
        self.store.get(format!("fsi-{}", idx).as_str()).map_or(Texture2D::empty(), |x| *x)
    }
}