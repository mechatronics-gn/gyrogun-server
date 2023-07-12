use std::thread;
use macroquad::prelude::*;
use macroquad::Window;
use mpsc::Sender;
use tokio::sync::watch;
use std::sync::{Arc, mpsc};
use macroquad::audio::play_sound_once;
use crate::client::PosCoord;
use crate::client::fake;
use crate::game::object::{Depth, ObjectWrapper};
use crate::sound::{SoundStore, SoundType};
use crate::texture::TextureStore;

pub fn launch(
    pos_rxs: Vec<watch::Receiver<PosCoord>>,
    window_size: (f32, f32),
    fake_input_tx: Option<Sender<fake::RawMessage>>,
    objects_rx: watch::Receiver<Vec<ObjectWrapper>>,
    time_rx: watch::Receiver<u32>,
    bg_color_rx: watch::Receiver<Color>,
    sounds_rx: mpsc::Receiver<SoundType>,
) {
    thread::spawn(move || {
        Window::from_config(
            Conf {
                window_title: "gyrogun".to_string(),
                window_width: window_size.0 as i32,
                window_height: window_size.1 as i32,
                high_dpi: true,
                fullscreen: false,
                sample_count: 0,
                window_resizable: false,
                icon: None,
                platform: Default::default(),
            },
            draw(pos_rxs, window_size, fake_input_tx, objects_rx, time_rx, bg_color_rx, sounds_rx)
        );
    });
}

async fn draw(
    mut pos_rxs: Vec<watch::Receiver<PosCoord>>,
    window_size: (f32, f32),
    fake_input_tx: Option<Sender<fake::RawMessage>>,
    objects_rx: watch::Receiver<Vec<ObjectWrapper>>,
    mut time_rx: watch::Receiver<u32>,
    bg_color_rx: watch::Receiver<Color>,
    sounds_rx: mpsc::Receiver<SoundType>,
) {
    let (width, height) = window_size;

    let texture_store = Arc::new(TextureStore::new());
    let sound_store = SoundStore::new().await;

    loop {
        clear_background(bg_color_rx.borrow().to_owned());

        if let Some(x) = &fake_input_tx {
            let mouse_pos = mouse_position();

            if is_mouse_button_pressed(MouseButton::Left) {
                x.send(fake::RawMessage::LeftClick(mouse_pos)).ok();
            } else if is_mouse_button_pressed(MouseButton::Right) {
                x.send(fake::RawMessage::RightClick(mouse_pos)).ok();
            } else if is_mouse_button_pressed(MouseButton::Middle) {
                x.send(fake::RawMessage::MiddleClick).ok();
            } else {
                x.send(fake::RawMessage::Hover(mouse_pos)).ok();
            }
        }

        while let Some(x) = sounds_rx.try_iter().next() {
            if let Some(sound) = sound_store.get(&x) {
                play_sound_once(sound);
            }
        }

        let time = *time_rx.borrow_and_update();
        let mut objects = objects_rx.borrow().to_owned();
        objects.sort_by_key(|k| match k {
            ObjectWrapper::Weak(i) => {
                if let Some(i) = i.upgrade() {
                    i.depth()
                } else {
                    Depth::Main(0)
                }
            }
            ObjectWrapper::Arc(i) => {
                i.depth()
            }
        });

        for i in &objects {
            match i {
                ObjectWrapper::Weak(i) => {
                    /*
                    This might actually cause instant disappearance if Arc is gone, but it won't
                    normally happen unless tick can't end before rendering,
                    which requires a lot of load.
                    Having about 15k objects at once cause instant disappearance for 40% chance.
                     */
                    if let Some(i) = i.upgrade() {
                        if time > i.born_time() {
                            i.draw(i.pos(time - i.born_time(), window_size), time - i.born_time(), window_size, texture_store.clone());
                        }
                    }
                }
                ObjectWrapper::Arc(i) => {
                    if time > i.born_time() {
                        i.draw(i.pos(time - i.born_time(), window_size), time - i.born_time(), window_size, texture_store.clone());
                    }
                }
            }
        }

        let mut i = 0;
        for pos_rx in &mut pos_rxs {
            let (x, y) = *pos_rx.borrow_and_update();
            let crosshair = texture_store.crosshair(i % 4);
            draw_texture_ex(crosshair, width / 2.0 + x - width / 48.0, height / 2.0 - y - height / 27.0, WHITE, DrawTextureParams {
                dest_size: Some(Vec2 { x: width / 36.0, y: height / 20.25 }),
                source: None, rotation: 0.0, flip_x: false, flip_y: false, pivot: None,
            });
            i += 1;
        }

        draw_text(format!("FPS: {:03}", get_fps()).as_str(), 50.0, 50.0, 80.0, if get_fps() < 60 { RED } else { BLACK });

        next_frame().await;
    }
}
