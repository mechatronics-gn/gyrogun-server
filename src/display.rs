use std::thread;
use macroquad::prelude::*;
use macroquad::Window;
use mpsc::Sender;
use tokio::sync::watch;
use std::sync::mpsc;
use crate::client::PosCoord;
use crate::client::fake;
use crate::game::object::{Depth, ObjectWrapper};

pub fn launch(
    pos_rxs: Vec<watch::Receiver<PosCoord>>,
    window_size: (f32, f32),
    fake_input_tx: Option<Sender<fake::RawMessage>>,
    objects_rx: watch::Receiver<Vec<ObjectWrapper>>,
    time_rx: watch::Receiver<u32>,
    bg_color_rx: watch::Receiver<Color>,
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
            draw(pos_rxs, window_size, fake_input_tx, objects_rx, time_rx, bg_color_rx)
        );
    });
}

async fn draw(
    mut pos_rxs: Vec<watch::Receiver<PosCoord>>,
    window_size: (f32, f32),
    fake_input_tx: Option<Sender<fake::RawMessage>>,
    objects_rx: watch::Receiver<Vec<ObjectWrapper>>,
    mut time_rx: watch::Receiver<u32>,
    bg_color_rx: watch::Receiver<Color>
) {
    let (width, height) = window_size;
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
                            i.draw(i.pos(time - i.born_time(), window_size), time - i.born_time(), window_size);
                        }
                    }
                }
                ObjectWrapper::Arc(i) => {
                    if time > i.born_time() {
                        i.draw(i.pos(time - i.born_time(), window_size), time - i.born_time(), window_size);
                    }
                }
            }
        }

        draw_circle(width / 2.0 - height / 2.0, height / 2.0, 20.0, GREEN);
        draw_circle(width / 2.0 + height / 2.0, height / 2.0, 20.0, GREEN);
        let mut i = 0;
        for pos_rx in &mut pos_rxs {
            let (x, y) = *pos_rx.borrow_and_update();
            let crosshair;
            if i % 2 == 0 {
                crosshair = Texture2D::from_file_with_format(include_bytes!("../res/crosshair_red_small.png"), Some(ImageFormat::Png));
            } else {
                crosshair = Texture2D::from_file_with_format(include_bytes!("../res/crosshair_green_small.png"), Some(ImageFormat::Png));
            }
            draw_texture_ex(crosshair, width / 2.0 + x - width / 48.0, height / 2.0 - y - height / 27.0, WHITE, DrawTextureParams {
                dest_size: Some(Vec2 { x: width / 24.0, y: height / 13.5 }),
                source: None, rotation: 0.0, flip_x: false, flip_y: false, pivot: None,
            });
            i += 1;
        }


        draw_text(format!("FPS: {:03}", get_fps()).as_str(), 50.0, 50.0, 80.0, if get_fps() < 60 { RED } else { BLACK });

        while !time_rx.has_changed().unwrap_or(false) {

        }

        next_frame().await;
    }
}
