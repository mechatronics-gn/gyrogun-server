use std::error::Error;
use std::f32::consts::PI;
use std::thread;
use macroquad::window::clear_background;
use tokio::io::AsyncReadExt;
use tokio::net::TcpListener;

use macroquad::prelude::*;
use macroquad::Window;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = "0.0.0.0:11076";

    let listener = TcpListener::bind(&addr).await?;

    let (pos_tx, mut pos_rx) = tokio::sync::watch::channel((0.0, 0.0));

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
            draw(pos_rx)
        );
    });

    //loop {
        let (mut socket, _) = listener.accept().await.unwrap();

        tokio::spawn(async move {
            let mut buf = vec![0 as u8; 16];

            let mut state = 0;

            let (mut my, mut mp, mut mr) = (0.0, 0.0, 0.0);
            let (mut ay, mut ap, mut ar) = (0.0, 0.0, 0.0);
            let (mut by, mut bp, mut br) = (0.0, 0.0, 0.0);
            let mut shooter = (0.0, 0.0, 0.0);

            loop {
                let n = socket.read_exact(&mut buf).await;

                if let Err(_) = n {
                    println!("Closing server");
                    return;
                }

                let message_type = [buf[0], buf[1], buf[2], buf[3]];
                let message_type = i32::from_be_bytes(message_type);

                let y = [buf[4], buf[5], buf[6], buf[7]];
                let y = f32::from_be_bytes(y);

                let p = [buf[8], buf[9], buf[10], buf[11]];
                let p = f32::from_be_bytes(p);

                let r = [buf[12], buf[13], buf[14], buf[15]];
                let r = f32::from_be_bytes(r);

                if message_type == 1 {
                    println!("message_type {message_type} y {:.3} p {:.3} r {:.3}", y, p, r);
                }

                if state == 0 {
                    if message_type == 1 {
                        (my, mp, mr) = (y, p, r);
                        println!("{my} {mp}");
                        state = 1;
                    }
                } else if state == 1 {
                    if message_type == 1 {
                        (ay, ap, ar) = (y, p, r);
                        println!("{ay} {ap}");
                        state = 2;
                    }
                } else if state == 2 {
                    if message_type == 1 {
                        (by, bp, br) = (y, p, r);
                        println!("{by} {bp}");
                        state = 3;
                        shooter = shooter_pos(my, ay, by, mp, ap);
                    }
                } else {
                    pos_tx.send(screen_pos(my, y, mp, p, shooter)).unwrap();
                }

                if message_type == 2 {
                    state = 0;
                }

            }
        });
    //}

    loop {

    }

    Ok(())
}

async fn draw(mut pos_rx: tokio::sync::watch::Receiver<(f32, f32)>) {
    loop {
        clear_background(WHITE);
        draw_text("IT WORKS!", 20.0, 20.0, 30.0, DARKGRAY);
        draw_circle(960.0 - 540.0, 540.0, 20.0, GREEN);
        draw_circle(960.0 + 540.0, 540.0, 20.0, GREEN);
        {
            let (x, y) = *pos_rx.borrow_and_update();
            //println!("{:.2} {:.2}", 960.0 + x, 540.0 - y);
            draw_circle(960.0 + x, 540.0 - y, 20.0, RED);
        }
        next_frame().await;
    }
}

fn shooter_pos(monitor_yaw: f32, a_yaw: f32, b_yaw: f32, monitor_pitch: f32, avg_pitch: f32) -> (f32, f32, f32) {
    let a_tan = ((monitor_yaw - a_yaw + 90.0) * PI / 180.0).tan();
    let b_tan = ((monitor_yaw - b_yaw + 90.0) * PI / 180.0).tan();
    let h_tan = (avg_pitch * PI / 180.0).tan();
    println!("a_tan {a_tan} b_tan {b_tan}");

    let x = (a_tan + b_tan) / (a_tan - b_tan) * (-540.0);
    let y = a_tan * (x + 540.0);
    let h = ((x * x + y * y) * h_tan).sqrt();

    return (x, y, h);
}

fn screen_pos(monitor_yaw: f32, curr_yaw: f32, monitor_pitch: f32, curr_pitch: f32, shooter_pos: (f32, f32, f32)) -> (f32, f32) {
    let xy_tan = ((monitor_yaw - curr_yaw + 90.0) * PI / 180.0).tan();
    let (x, y, h) = shooter_pos;

    let my_tan = ((90.0 - monitor_pitch) * PI / 180.0).tan();
    let my_cos = (monitor_pitch * PI / 180.0).cos();
    let cy_tan = (curr_pitch * PI / 180.0).tan();

    let retx = x - y / xy_tan;
    let rety = ((x * x + y * y).sqrt() * -cy_tan + h) / (my_tan + cy_tan) * my_tan / my_cos;

    return (retx, rety);
}
