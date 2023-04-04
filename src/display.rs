use std::thread;
use macroquad::prelude::*;
use macroquad::Window;
use tokio::sync::watch;
use crate::client::PosCoord;

pub fn launch(pos_rxs: Vec<watch::Receiver<PosCoord>>) {
    thread::spawn(|| {
        Window::from_config(
            Conf {
                window_title: "gyrogun".to_string(),
                window_width: 1920,
                window_height: 1080,
                high_dpi: true,
                fullscreen: false,
                sample_count: 0,
                window_resizable: false,
                icon: None,
                platform: Default::default(),
            },
            draw(pos_rxs)
        );
    });
}

async fn draw(mut pos_rxs: Vec<watch::Receiver<PosCoord>>) {
    loop {
        clear_background(WHITE);
        draw_text("IT WORKS!", 20.0, 20.0, 30.0, DARKGRAY);
        draw_circle(960.0 - 540.0, 540.0, 20.0, GREEN);
        draw_circle(960.0 + 540.0, 540.0, 20.0, GREEN);
        for pos_rx in &mut pos_rxs {
            let (x, y) = *pos_rx.borrow_and_update();
            draw_circle(960.0 + x, 540.0 - y, 20.0, RED);
        }
        next_frame().await;
    }
}
