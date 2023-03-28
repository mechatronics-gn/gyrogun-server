use std::error::Error;
use std::f32::consts::PI;
use std::{env, thread};
use std::str::FromStr;
use macroquad::window::clear_background;
use tokio::io::AsyncReadExt;
use tokio::net::TcpListener;

use macroquad::prelude::*;
use macroquad::Window;

#[derive(Debug)]
enum Message {
    Click { pos: (f32, f32) },
    DoubleClick { pos: (f32, f32) },
    Disconnect,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    let addr = "0.0.0.0:11076";

    let listener = TcpListener::bind(&addr).await?;

    let client_count = i32::from_str(args.get(1).unwrap_or(&String::from("1")).as_str())?;

    let mut pos_rxs: Vec<tokio::sync::watch::Receiver<(f32, f32)>> = vec![];

    let (msg_tx, mut msg_rx) = tokio::sync::mpsc::channel(128);

    println!("Server up, waiting for {client_count} clients");

    for i in 0..client_count {
        let (mut socket, addr) = listener.accept().await.unwrap();

        println!("Connected with client #{i}, {addr}");

        let (pos_tx, pos_rx) = tokio::sync::watch::channel((0.0, 0.0));
        pos_rxs.push(pos_rx);

        let msg_tx = msg_tx.clone();

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
                    msg_tx.send((i, Message::Disconnect)).await.unwrap();
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
                    let pos = screen_pos(my, y, mp, p, shooter);
                    pos_tx.send(pos).unwrap();
                    if message_type == 1 {
                        msg_tx.send((i, Message::Click { pos })).await.unwrap();
                    }
                }

                if message_type == 2 {
                    state = 0;
                }

            }
        });
    }

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

    let mut disconnect_count = 0;

    loop {
        let (client, msg)= msg_rx.recv().await.unwrap();
        println!("Client #{client}: {:?}", msg);

        if let Message::Disconnect = msg {
            disconnect_count += 1;

            if disconnect_count == client_count {
                println!("All clients disconnected, exiting");
                break;
            }
        }
    }

    Ok(())
}

async fn draw(mut pos_rxs: Vec<tokio::sync::watch::Receiver<(f32, f32)>>) {
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
