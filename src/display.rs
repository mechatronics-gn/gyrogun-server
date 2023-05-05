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
            draw(pos_rxs, window_size, fake_input_tx, objects_rx, time_rx)
        );
    });
}

async fn draw(
    mut pos_rxs: Vec<watch::Receiver<PosCoord>>,
    window_size: (f32, f32),
    fake_input_tx: Option<Sender<fake::RawMessage>>,
    objects_rx: watch::Receiver<Vec<ObjectWrapper>>,
    time_rx: watch::Receiver<u32>,
) {
    let (width, height) = window_size;
    loop {
        clear_background(Color::from_rgba(147, 169, 209, 0));

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

        let time = *time_rx.borrow();
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
                    if let Some(i) = i.upgrade() {
                        i.draw(i.pos(time - i.born_time(), window_size), time - i.born_time(), window_size);
                    }
                }
                ObjectWrapper::Arc(i) => {
                    i.draw(i.pos(time - i.born_time(), window_size), time - i.born_time(), window_size);
                }
            }
        }

        draw_circle(width / 2.0 - height / 2.0, height / 2.0, 20.0, GREEN);
        draw_circle(width / 2.0 + height / 2.0, height / 2.0, 20.0, GREEN);
        for pos_rx in &mut pos_rxs {
            let (x, y) = *pos_rx.borrow_and_update();
            draw_circle(width / 2.0 + x, height / 2.0 - y, 20.0, RED);
        }
        next_frame().await;
    }
}
