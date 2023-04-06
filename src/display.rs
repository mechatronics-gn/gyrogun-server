use std::thread;
use macroquad::prelude::*;
use macroquad::Window;
use tokio::sync::watch;
use crate::client::PosCoord;

pub fn launch(pos_rxs: Vec<watch::Receiver<PosCoord>>, window_size: (f32, f32)) {
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
            draw(pos_rxs, window_size)
        );
    });
}

async fn draw(mut pos_rxs: Vec<watch::Receiver<PosCoord>>, window_size: (f32, f32)) {
    let (width, height) = window_size;
    loop {
        clear_background(WHITE);
        draw_text("IT WORKS!", 20.0, 20.0, 30.0, DARKGRAY);
        draw_circle(width / 2.0 - height / 2.0, height / 2.0, 20.0, GREEN);
        draw_circle(width / 2.0 + height / 2.0, height / 2.0, 20.0, GREEN);
        for pos_rx in &mut pos_rxs {
            let (x, y) = *pos_rx.borrow_and_update();
            draw_circle(width / 2.0 + x, height / 2.0 - y, 20.0, RED);
        }
        next_frame().await;
    }
}
